use super::cards::*;
use serde::{Deserialize, Serialize};

pub trait Api {
    type Error;
    fn get_state(&self) -> Board;
    fn do_action(&mut self, action: PlayerAction) -> Result<Vec<BoardDelta>, Self::Error>;
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
    ChangeDiscardAmount(Player, i32),
    ChangeHealth(Player, i32),
    ChangeCombat(Player, i32),
    ChangeGold(Player, i32),
    // This does not actually change the board but is here
    // so each player can know what the other one was doing.
    PlayerDeclaredAction(PlayerAction),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
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
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Mat {
    pub field: Vec<CardInField>,
    pub hand: usize,
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
            BoardDelta::ChangeHealth(player, amount) => {
                let new = self.mats[player].lives as i32 + amount;
                if new < 0 {
                    return Err(BoardDeltaError::StatUnderflow);
                }
                self.mats[player].lives = new as usize;
            }
            BoardDelta::ChangeCombat(player, amount) => {
                let new = self.mats[player].combat as i32 + amount;
                if new < 0 {
                    return Err(BoardDeltaError::StatUnderflow);
                }
                self.mats[player].combat = new as usize;
            }
            BoardDelta::ChangeGold(player, amount) => {
                let new = self.mats[player].gold as i32 + amount;
                if new < 0 {
                    return Err(BoardDeltaError::StatUnderflow);
                }
                self.mats[player].gold = new as usize;
            }
            BoardDelta::ChangeDiscardAmount(player, amount) => {
                let new = self.mats[player].must_discard as i32 + amount;
                if new < 0 {
                    return Err(BoardDeltaError::StatUnderflow);
                }
                self.mats[player].must_discard = new as usize;
            }
            BoardDelta::PlayerDeclaredAction(_action) => {}
        }

        Ok(())
    }
}