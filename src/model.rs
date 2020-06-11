use crate::smallrng::{Rng, SRng};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct State {
    pub shop: Vec<Card>,
    pub shop_deck: Vec<Card>,
    pub gems: Vec<Card>,

    pub current_player: usize,
    pub players: usize,
    pub mats: Vec<PlayerMat>,
    pub rng: SRng,
}

impl State {
    pub fn new(players: usize, setup: &Setup, mut rng: SRng) -> State {
        let mut shop_deck = setup.shop_deck.clone();
        rng.shuffle(&mut shop_deck);
        let shop = draw(6, &mut shop_deck);
        let current_player = rng.gen::<usize>() % players;
        let mats: Vec<_> = (0..players)
            .map(|i| {
                let starting_cards = if i == current_player {
                    3
                } else if players > 2 && i == current_player + 1 {
                    4
                } else {
                    5
                };
                PlayerMat::new(starting_cards, &setup, rng.fork())
            })
            .collect();

        State {
            shop,
            players,
            shop_deck: shop_deck,
            gems: setup.gems.clone(),
            current_player,
            mats,
            rng,
        }
    }
}

pub fn draw(amount: usize, source: &mut Vec<Card>) -> Vec<Card> {
    source.split_off(source.len() - amount)
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PlayerMat {
    pub field: Vec<Card>,
    pub hand: Vec<Card>,
    pub discard: Vec<Card>,
    pub deck: Vec<Card>,
    pub scrap: Vec<Card>,
    pub lives: usize,
    pub combat: usize,
    pub gold: usize,
    pub must_discard: usize,
}

impl PlayerMat {
    pub fn new(starting_cards: usize, setup: &Setup, mut rng: SRng) -> PlayerMat {
        let mut deck = setup.player_deck.clone();
        rng.shuffle(&mut deck);
        let hand = draw(starting_cards, &mut deck);
        PlayerMat {
            field: vec![],
            hand,
            discard: vec![],
            deck,
            scrap: vec![],
            lives: 50,
            combat: 0,
            gold: 0,
            must_discard: 0,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Setup {
    pub shop_deck: Vec<Card>,
    pub gems: Vec<Card>,
    pub player_deck: Vec<Card>,
}

impl Setup {
    pub fn test() -> Setup {
        Setup {
            shop_deck: vec![
                Card::Spark,
                Card::Spark,
                Card::Spark,
                Card::Spark,
                Card::Spark,
                Card::Spark,
                Card::Spark,
                Card::Spark,
                Card::Spark,
                Card::Spark,
            ],
            gems: vec![
                Card::FireGem,
                Card::FireGem,
                Card::FireGem,
                Card::FireGem,
                Card::FireGem,
                Card::FireGem,
                Card::FireGem,
                Card::FireGem,
                Card::FireGem,
                Card::FireGem,
            ],
            player_deck: vec![
                Card::Gold,
                Card::Gold,
                Card::Gold,
                Card::Gold,
                Card::Gold,
                Card::Gold,
                Card::Gold,
                Card::ShortSword,
                Card::Dagger,
                Card::Ruby,
            ],
        }
    }
}

impl Default for Setup {
    fn default() -> Setup {
        Setup::test()
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum Faction {
    NoFaction,
    Wild,
    Necros,
    Guild,
    Imperial,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum Card {
    Unknown,
    Gold,
    ShortSword,
    Dagger,
    Ruby,
    FireGem,
    Spark,
}

impl Card {
    pub fn faction(&self) -> Faction {
        match self {
            Card::Unknown => Faction::NoFaction,
            Card::Gold => Faction::NoFaction,
            Card::ShortSword => Faction::NoFaction,
            Card::Dagger => Faction::NoFaction,
            Card::Ruby => Faction::NoFaction,
            Card::FireGem => Faction::NoFaction,

            Card::Spark => Faction::Wild,
        }
    }

    pub fn is_champion(&self) -> bool {
        false
    }

    pub fn is_action(&self) -> bool {
        !self.is_champion()
    }

    pub fn cost(&self) -> Option<usize> {
        match self {
            Card::Unknown => None,
            Card::Gold => None,
            Card::ShortSword => None,
            Card::Dagger => None,
            Card::Ruby => None,
            Card::FireGem => Some(2),
            Card::Spark => Some(1),
        }
    }

    pub fn primary_ability(&self) -> Option<Vec<Effect>> {
        let effects = match self {
            Card::Gold => vec![Effect::Gold(1)],
            Card::Ruby => vec![Effect::Gold(2)],
            Card::Dagger => vec![Effect::Combat(1)],
            Card::ShortSword => vec![Effect::Combat(2)],
            Card::FireGem => vec![Effect::Gold(2)],
            _ => { return None; }
        };
        Some(effects)
    }

    pub fn expend_ability(&self) -> Option<Vec<Effect>> { None }

    pub fn ally_ability(&self) -> Option<Vec<Effect>> { None }

    pub fn sacrifice_ability(&self) -> Option<Vec<Effect>> {
        let effects = match self {
            Card::FireGem => {
                vec![Effect::Combat(3)]
            }
            _ => { return None; }
        };
        Some(effects)
    }
}

pub enum Effect {
    Gold(usize),
    Combat(usize),
    Heal(usize),
    Draw(usize),

    Choice(Vec<Effect>, Vec<Effect>),

    CombatPer(PerAmount),
    HealPer(PerAmount),
    Nothing,
    PutOverDeckFromDiscard,
    PutInHandFromDiscard,
    StunChampion,
    PrepareChampion,
    Sacrifice(usize),
    OpponentDiscards(usize),
}

pub enum PerAmount {
    AdditionalFactionCard(Faction),
    Champion(),
    AdditionalChampion(),
    AdditionalGuardian(),
    // Filter(Filter),
}

// pub enum Pointer {
//     Hand(usize, usize),
//     Discard(usize, usize),
// }
// pub enum Filter {
//     Player,
//     Opponent,
//     Discard,
//     Hand,
//     Champion,
//     Field,
//     Action,
//     Faction(Faction),
//     Shop,
//     Or(Vec<Filter>),
//     And(Vec<Filter>),
// }
// 
// impl Filter {
//     pub fn player_hand_or_discard() -> Filter {
//         Filter::And(vec![Filter::Player, Filter::Or(vec![Filter::Hand, Filter::Discard])])
//     }
//     pub fn accepts_pointer(&self, p: Pointer) -> bool {}
// }

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum EffectArgument {
    ChooseFirst,
    ChooseSecond,
    Champion { player: usize, champion: usize },
    CardInHand(usize),
    CardInDiscard(usize),
    Opponent(usize),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum PlayerAction {
    Play(usize, Vec<EffectArgument>),
    ActivateExpendAbility(usize, Vec<EffectArgument>),
    ActivateAllyAbility(usize, Vec<EffectArgument>),
    ActivateSacrificeAbility(usize, Vec<EffectArgument>),
    AttackPlayer(usize, usize),
    AttackPlayerChampion(usize, usize, usize),
    PurchaseFromShop(usize),
    PurchaseFireGem,
    EndTurn,
}

impl State {
    pub fn apply_effects(&mut self, mut effects: Vec<Effect>, _effect_args: Vec<EffectArgument>) -> Result<(), &'static str> {
        let ref mut mat = self.mats[self.current_player];
        while !effects.is_empty() {
            match effects.pop().unwrap() {
                Effect::Gold(x) => mat.gold += x,
                Effect::Combat(x) => mat.combat += x,
                Effect::Heal(x) => mat.lives += x,
                Effect::Nothing => {},
                Effect::Draw(x) => {
                    for _ in 0..x {
                        if mat.deck.is_empty() && !mat.discard.is_empty() {
                            mat.deck.append(&mut mat.discard);
                            self.rng.shuffle(&mut mat.deck);
                        }

                        if let Some(card) = mat.deck.pop() {
                            mat.hand.push(card);
                        }
                    }
                }
                _ => return Err("Unsupported effect"),
            }
        }
        Ok(())
    }

    pub fn do_action(&mut self, action: PlayerAction) -> Result<(), &'static str> {
        let mut state = self.clone();

        match action {
            PlayerAction::Play(position, effect_args) => {
                let ref mut mat = state.mats[state.current_player];
                if position >= mat.hand.len() {
                    return Err("No such card in hand");
                }

                let card = mat.hand.remove(position);
                mat.field.push(card.clone());

                if let Some(effects) = card.primary_ability() {
                    state.apply_effects(effects, effect_args)?;
                }
            }

            PlayerAction::EndTurn => {
                let ref mut mat = state.mats[state.current_player];

                mat.gold = 0;
                mat.combat = 0;
                let mut to_discard: Vec<_> = mat
                    .field
                    .iter()
                    .filter(|card| card.is_action())
                    .map(|card| card.clone())
                    .collect();
                mat.field.retain(Card::is_champion);
                mat.discard.append(&mut to_discard);
                mat.discard.append(&mut mat.hand);

                state.apply_effects(vec![Effect::Draw(5)], vec![])?;

                state.current_player = (state.current_player + 1) % state.players;
            }

            PlayerAction::PurchaseFromShop(position) => {
                let ref mut mat = state.mats[state.current_player];

                if position >= state.shop.len() {
                    return Err("No such card in shop");
                }

                let cost = match state.shop[position].cost() {
                    Some(cost) => cost,
                    None => return Err("Card with no cost, should not be in the shop"),
                };

                if mat.gold < cost {
                    return Err("Not enough gold");
                }

                mat.gold -= cost;
                mat.discard.push(state.shop[position].clone());

                if let Some(card) = state.shop_deck.pop() {
                    state.shop[position] = card;
                } else {
                    state.shop.remove(position);
                }
            }
            PlayerAction::PurchaseFireGem => {
                let ref mut mat = state.mats[state.current_player];

                if state.gems.is_empty() {
                    return Err("No more fire gems");
                }
                let cost = Card::FireGem.cost().unwrap();

                if mat.gold < cost {
                    return Err("Not enough gold");
                }
                mat.gold -= cost;
                mat.discard.push(state.gems.pop().unwrap());
            }
            PlayerAction::AttackPlayer(player, amount) => {
                if player >= state.players {
                    return Err("No such player");
                }
                if state.current_player == player {
                    return Err("Player can't attack himself");
                }
                if state.mats[state.current_player].combat < amount {
                    return Err("Not enough combat");
                }
                state.mats[state.current_player].combat -= amount;
                state.mats[player].lives -= amount;
            }
            _ => {}
        }

        *self = state;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_initial_state() {
        let state = State::new(2, &Setup::test(), SRng::new(0));

        let ref p1 = state.mats[state.current_player];
        let ref p2 = state.mats[(state.current_player + 1) % 2];

        assert_eq!(p1.hand.len(), 3);
        assert_eq!(p1.deck.len(), 7);
        assert_eq!(p1.scrap.len(), 0);
        assert_eq!(p1.discard.len(), 0);
        assert_eq!(p1.lives, 50);
        assert_eq!(p1.combat, 0);
        assert_eq!(p1.gold, 0);
        assert_eq!(p1.must_discard, 0);

        assert_eq!(p2.hand.len(), 5);
        assert_eq!(p2.deck.len(), 5);
        assert_eq!(p2.scrap.len(), 0);
        assert_eq!(p2.discard.len(), 0);
        assert_eq!(p2.lives, 50);
        assert_eq!(p2.combat, 0);
        assert_eq!(p2.gold, 0);
        assert_eq!(p2.must_discard, 0);

        assert_eq!(state.shop.len(), 6);
        assert_eq!(state.shop_deck.len(), 4);
    }

    #[test]
    fn simple_test_run() -> Result<(), &'static str> {
        let mut state = State::new(2, &Setup::test(), SRng::new(0));
        let p1 = state.current_player;
        let p2 = (state.current_player + 1) % 2;

        {
            assert_vec_eq(
                &state.mats[p1].hand,
                &vec![Card::Gold, Card::ShortSword, Card::Dagger],
            );
        }

        state.do_action(PlayerAction::Play(0, vec![]))?;

        {
            assert_vec_eq(&state.mats[p1].hand, &vec![Card::ShortSword, Card::Dagger]);
            assert_vec_eq(&state.mats[p1].field, &vec![Card::Gold]);
            assert_eq!(state.mats[p1].gold, 1);
        }

        state.do_action(PlayerAction::Play(1, vec![]))?;

        state.do_action(PlayerAction::Play(0, vec![]))?;

        {
            assert_eq!(state.mats[p1].combat, 3);
        }

        state.do_action(PlayerAction::EndTurn)?;

        {
            assert_eq!(state.current_player, p2);
            assert_eq!(state.mats[p1].hand.len(), 5);
            assert_eq!(state.mats[p1].deck.len(), 2);
            assert_eq!(state.mats[p1].discard.len(), 3);
            assert_eq!(state.mats[p1].field.len(), 0);
            assert_eq!(state.mats[p1].gold, 0);
            assert_eq!(state.mats[p1].combat, 0);

            assert_vec_eq(
                &state.mats[p2].hand,
                &vec![Card::Gold, Card::Gold, Card::Gold, Card::Dagger, Card::Gold],
            );
        }

        for _ in 0..5 {
            state.do_action(PlayerAction::Play(0, vec![]))?;
        }

        {
            assert_eq!(state.mats[p2].gold, 4);
            assert_eq!(state.mats[p2].combat, 1);

            assert_vec_eq(
                &state.shop,
                &vec![
                    Card::Spark,
                    Card::Spark,
                    Card::Spark,
                    Card::Spark,
                    Card::Spark,
                    Card::Spark,
                ],
            );
        }

        state.do_action(PlayerAction::PurchaseFromShop(0))?;

        {
            assert_eq!(state.mats[p2].gold, 3);
            assert_eq!(state.mats[p2].combat, 1);
            assert_vec_eq(
                &state.shop,
                &vec![
                    Card::Spark,
                    Card::Spark,
                    Card::Spark,
                    Card::Spark,
                    Card::Spark,
                    Card::Spark,
                ],
            );
            assert_vec_eq(&state.mats[p2].discard, &vec![Card::Spark]);
        }

        state.do_action(PlayerAction::AttackPlayer(p1, 1))?;

        {
            assert_eq!(state.mats[p2].combat, 0);
            assert_eq!(state.mats[p1].lives, 49);
        }

        state.do_action(PlayerAction::PurchaseFireGem)?;

        {
            assert_eq!(state.mats[p2].gold, 1);
            assert_vec_eq(&state.mats[p2].discard, &vec![Card::Spark, Card::FireGem]);
        }

        Ok(())
    }

    fn assert_vec_eq<T>(a: &Vec<T>, b: &Vec<T>)
    where
        T: PartialEq + std::fmt::Debug,
    {
        if a.len() != b.len() {
            panic!(format!("Differnt array len: {:?} != {:?}", a, b))
        } else if (0..a.len()).any(|i| a[i] != b[i]) {
            panic!(format!("Arrays differ: {:?} != {:?}", a, b))
        }
    }
}
