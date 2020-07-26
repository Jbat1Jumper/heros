use super::cards::*;
use serde::{Deserialize, Serialize};
use std::ops::AddAssign;

pub trait Api {
    type Error;
    fn get_board<'a>(&'a self) -> &'a Board;
    fn do_action(&mut self, action: PlayerAction) -> Result<(), Self::Error>;
    fn poll_deltas(&mut self) -> Vec<BoardDelta>;
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum PlayerAction {
    Play(usize, Vec<EffectArgument>),
    ActivateExpendAbility(usize, Vec<EffectArgument>),
    ActivateAllyAbility(usize, Vec<EffectArgument>),
    ActivateSacrificeAbility(usize, Vec<EffectArgument>),
    AttackPlayer(Player, usize),
    AttackPlayerChampion(Player, usize),
    PurchaseFromShop(usize),
    Discard(usize),
    PurchaseFireGem,
    EndTurn,
}

pub type Player = usize;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum EffectArgument {
    ChooseFirst,
    ChooseSecond,
    Champion(Player, usize),
    CardInHand(usize),
    CardInDiscard(usize),
    Opponent(usize),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Location {
    Hand(Player),
    Discard(Player),
    Deck(Player),
    Field(Player),
    Sacrifice,
    Shop,
    ShopDeck,
    FireGems,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum BoardDelta {
    Move(Location, usize, Location, Option<Card>),
    DecreaseDiscardAmount(Player, usize),
    DecreaseHealth(Player, usize),
    DecreaseCombat(Player, usize),
    DecreaseGold(Player, usize),
    IncreaseDiscardAmount(Player, usize),
    IncreaseHealth(Player, usize),
    IncreaseCombat(Player, usize),
    IncreaseGold(Player, usize),
    ChangeCurrentPlayer(Player),
    SetExpendAbilityUsed(Player, usize, bool),
    SetAllyAbilityUsed(Player, usize, bool),
    // These do not actually change the board but are here
    // so each player can know what the other one was doing.
    PlayerDeclaredAction(PlayerAction),
    ShuffleDeck(Player),
    GameOver,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Board {
    pub shop: Vec<Card>,
    pub shop_deck: usize,
    pub gems: usize,
    pub sacrificed: Vec<Card>,

    pub current_player: Player,
    pub players: usize,
    pub mats: Vec<Mat>,

    pub you: Player,
    pub your_hand: Vec<Card>,
    pub game_over: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Mat {
    pub field: Vec<CardInField>,
    pub hand: usize,
    pub name: String,
    pub discard: Vec<Card>,
    pub deck: usize,
    pub lives: usize,
    pub combat: usize,
    pub gold: usize,
    pub must_discard: usize,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum BoardDeltaError {
    CardMismatch(Card, Option<Card>),
    MissingCard(Location),
    WrongSourceLocation,
    StatUnderflow,
}

impl Board {
    pub fn apply(&mut self, delta: BoardDelta) -> Result<(), BoardDeltaError> {
        match delta {
            BoardDelta::Move(from, index, to, card) => {
                let mut removed_card: Option<Card> = None;
                match from {
                    Location::Deck(player) => {
                        self.mats[player].deck -= 1;
                    }
                    Location::Hand(player) => {
                        self.mats[player].hand -= 1;
                        if player == self.you {
                            removed_card = Some(self.your_hand.remove(index));
                        }
                    }
                    Location::Field(player) => {
                        removed_card = Some(self.mats[player].field.remove(index).card);
                    }
                    Location::Discard(player) => {
                        removed_card = Some(self.mats[player].discard.remove(index));
                    }
                    Location::Shop => {
                        removed_card = Some(self.shop.remove(index));
                    }
                    Location::Sacrifice => return Err(BoardDeltaError::WrongSourceLocation),
                    Location::ShopDeck => self.shop_deck -= 1,
                    Location::FireGems => {
                        self.gems -= 1;
                        removed_card = Some(Card::FireGem);
                    }
                };

                if removed_card.is_some() && removed_card != card {
                    return Err(BoardDeltaError::CardMismatch(removed_card.unwrap(), card));
                }

                match to {
                    Location::Deck(player) => {
                        self.mats[player].deck += 1;
                    }
                    Location::Hand(player) => {
                        self.mats[player].hand += 1;
                        if player == self.you {
                            self.your_hand
                                .push(card.ok_or(BoardDeltaError::MissingCard(to))?);
                        }
                    }
                    Location::Field(player) => self.mats[player].field.push(CardInField::new(
                        card.ok_or(BoardDeltaError::MissingCard(to))?,
                    )),
                    Location::Discard(player) => self.mats[player]
                        .discard
                        .push(card.ok_or(BoardDeltaError::MissingCard(to))?),
                    Location::Sacrifice => self
                        .sacrificed
                        .push(card.ok_or(BoardDeltaError::MissingCard(to))?),
                    Location::Shop => self
                        .shop
                        .push(card.ok_or(BoardDeltaError::MissingCard(to))?),

                    Location::ShopDeck => self.shop_deck += 1,
                    Location::FireGems => {
                        self.gems += 1;
                        if card != Some(Card::FireGem) {
                            return Err(BoardDeltaError::CardMismatch(Card::FireGem, card));
                        }
                    }
                }
            }
            BoardDelta::IncreaseHealth(player, amount) => self.mats[player].lives += amount,
            BoardDelta::DecreaseHealth(player, amount) => {
                self.mats[player].lives = self.mats[player].lives.saturating_sub(amount);
            }
            BoardDelta::IncreaseCombat(player, amount) => self.mats[player].combat += amount,
            BoardDelta::DecreaseCombat(player, amount) => {
                self.mats[player].combat = self.mats[player].combat.saturating_sub(amount);
            }
            BoardDelta::IncreaseGold(player, amount) => self.mats[player].gold += amount,
            BoardDelta::DecreaseGold(player, amount) => {
                self.mats[player].gold = self.mats[player].gold.saturating_sub(amount);
            }
            BoardDelta::IncreaseDiscardAmount(player, amount) => {
                self.mats[player].must_discard += amount
            }
            BoardDelta::DecreaseDiscardAmount(player, amount) => {
                self.mats[player].must_discard =
                    self.mats[player].must_discard.saturating_sub(amount);
            }
            BoardDelta::ChangeCurrentPlayer(player) => {
                self.current_player = player;
            }
            BoardDelta::PlayerDeclaredAction(_action) => {}
            BoardDelta::ShuffleDeck(_player) => {}
            BoardDelta::SetExpendAbilityUsed(player, index, value) => {
                self.mats[player].field[index].expend_ability_used = value;
            },
            BoardDelta::SetAllyAbilityUsed(player, index, value) => {
                self.mats[player].field[index].ally_ability_used = value;
            },
            BoardDelta::GameOver => {
                self.game_over = true;
            }
        }

        Ok(())
    }
}

impl AddAssign<BoardDelta> for Board {
    fn add_assign(&mut self, delta: BoardDelta) {
        self.apply(delta);
    }
}

impl AddAssign<Vec<BoardDelta>> for Board {
    fn add_assign(&mut self, deltas: Vec<BoardDelta>) {
        for delta in deltas {
            self.apply(delta);
        }
    }
}
