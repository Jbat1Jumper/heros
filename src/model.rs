use crate::smallrng::{Rng, SRng};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct State {
    pub shop: Vec<Card>,
    pub shop_deck: Vec<Card>,
    pub gems: Vec<Card>,
    pub sacrificed: Vec<Card>,

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
            sacrificed: vec![],
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
pub struct CardInField {
    pub card: Card,
    pub expend_ability_used: bool,
    pub ally_ability_used: bool,
}

impl CardInField {
    pub fn new(card: Card) -> CardInField {
        CardInField {
            card,
            expend_ability_used: false,
            ally_ability_used: false,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PlayerMat {
    pub field: Vec<CardInField>,
    pub hand: Vec<Card>,
    pub discard: Vec<Card>,
    pub deck: Vec<Card>,
    pub lives: usize,
    pub combat: usize,
    pub gold: usize,
    pub must_discard: usize,
    pub next_action_purchase_to_top_of_deck: usize,
    pub next_purchase_to_top_of_deck: usize,
    pub next_purchase_to_hand: usize,
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
            lives: 50,
            combat: 0,
            gold: 0,
            must_discard: 0,
            next_action_purchase_to_top_of_deck: 0,
            next_purchase_to_top_of_deck: 0,
            next_purchase_to_hand: 0,
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

    pub fn base() -> Setup {
        Setup {
            shop_deck: vec![
                Card::ArkusImperialDragon,
                Card::CloseRanks,
                Card::Command,
                Card::DarianWarMage,
                Card::Domination,
                Card::CristovTheJust,
                Card::KrakaHighPriest,
                Card::ManAtArms,
                Card::ManAtArms,
                Card::MasterWeyan,
                Card::RallyTheTroops,
                Card::Recruit,
                Card::Recruit,
                Card::Recruit,
                Card::TithePriest,
                Card::TithePriest,
                Card::Taxation,
                Card::Taxation,
                Card::Taxation,
                Card::WordOfPower,
                Card::BorgOgreMercenary,
                Card::Bribe,
                Card::Bribe,
                Card::Bribe,
                Card::DeathThreat,
                Card::Deception,
                Card::FireBomb,
                Card::HitJob,
                Card::Intimidation,
                Card::Intimidation,
                Card::MyrosGuildMage,
                Card::ParovTheEnforcer,
                Card::Profit,
                Card::Profit,
                Card::Profit,
                Card::RakeMasterAssassin,
                Card::RasmusTheSmuggler,
                Card::SmashAndGrab,
                Card::StreetThug,
                Card::StreetThug,
                Card::CultPriest,
                Card::CultPriest,
                Card::DarkEnergy,
                Card::DarkReward,
                Card::DeathCultist,
                Card::DeathCultist,
                Card::DeathTouch,
                Card::DeathTouch,
                Card::DeathTouch,
                Card::RaylaEndweaver,
                Card::Influence,
                Card::Influence,
                Card::Influence,
                Card::KrythosMasterVampire,
                Card::LifeDrain,
                Card::LysTheUnseen,
                Card::TheRot,
                Card::TheRot,
                Card::TyrannorTheDevourer,
                Card::VarrickTheNecromancer,
                Card::BroelynLoreweaver,
                Card::CronTheBerserker,
                Card::DireWolf,
                Card::ElvenCurse,
                Card::ElvenCurse,
                Card::ElvenGift,
                Card::ElvenGift,
                Card::ElvenGift,
                Card::GrakStormGiant,
                Card::NaturesBounty,
                Card::OrcGrunt,
                Card::OrcGrunt,
                Card::Rampage,
                Card::TorgenRocksplitter,
                Card::Spark,
                Card::Spark,
                Card::Spark,
                Card::WolfForm,
                Card::WolfShaman,
                Card::WolfShaman,
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

    ArkusImperialDragon,
    CloseRanks,
    Command,
    DarianWarMage,
    Domination,
    CristovTheJust,
    KrakaHighPriest,
    ManAtArms,
    MasterWeyan,
    RallyTheTroops,
    Recruit,
    TithePriest,
    Taxation,
    WordOfPower,

    BorgOgreMercenary,
    Bribe,
    DeathThreat,
    Deception,
    FireBomb,
    HitJob,
    Intimidation,
    MyrosGuildMage,
    ParovTheEnforcer,
    Profit,
    RakeMasterAssassin,
    RasmusTheSmuggler,
    SmashAndGrab,
    StreetThug,

    CultPriest,
    DarkEnergy,
    DarkReward,
    DeathCultist,
    DeathTouch,
    RaylaEndweaver,
    Influence,
    KrythosMasterVampire,
    LifeDrain,
    LysTheUnseen,
    TheRot,
    TyrannorTheDevourer,
    VarrickTheNecromancer,

    BroelynLoreweaver,
    CronTheBerserker,
    DireWolf,
    ElvenCurse,
    ElvenGift,
    GrakStormGiant,
    NaturesBounty,
    OrcGrunt,
    Rampage,
    TorgenRocksplitter,
    Spark,
    WolfForm,
    WolfShaman,
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

            Card::ArkusImperialDragon => Faction::Imperial,
            Card::CloseRanks => Faction::Imperial,
            Card::Command => Faction::Imperial,
            Card::DarianWarMage => Faction::Imperial,
            Card::Domination => Faction::Imperial,
            Card::CristovTheJust => Faction::Imperial,
            Card::KrakaHighPriest => Faction::Imperial,
            Card::ManAtArms => Faction::Imperial,
            Card::MasterWeyan => Faction::Imperial,
            Card::RallyTheTroops => Faction::Imperial,
            Card::Recruit => Faction::Imperial,
            Card::TithePriest => Faction::Imperial,
            Card::Taxation => Faction::Imperial,
            Card::WordOfPower => Faction::Imperial,

            Card::BorgOgreMercenary => Faction::Guild,
            Card::Bribe => Faction::Guild,
            Card::DeathThreat => Faction::Guild,
            Card::Deception => Faction::Guild,
            Card::FireBomb => Faction::Guild,
            Card::HitJob => Faction::Guild,
            Card::Intimidation => Faction::Guild,
            Card::MyrosGuildMage => Faction::Guild,
            Card::ParovTheEnforcer => Faction::Guild,
            Card::Profit => Faction::Guild,
            Card::RakeMasterAssassin => Faction::Guild,
            Card::RasmusTheSmuggler => Faction::Guild,
            Card::SmashAndGrab => Faction::Guild,
            Card::StreetThug => Faction::Guild,

            Card::CultPriest => Faction::Necros,
            Card::DarkEnergy => Faction::Necros,
            Card::DarkReward => Faction::Necros,
            Card::DeathCultist => Faction::Necros,
            Card::DeathTouch => Faction::Necros,
            Card::RaylaEndweaver => Faction::Necros,
            Card::Influence => Faction::Necros,
            Card::KrythosMasterVampire => Faction::Necros,
            Card::LifeDrain => Faction::Necros,
            Card::LysTheUnseen => Faction::Necros,
            Card::TheRot => Faction::Necros,
            Card::TyrannorTheDevourer => Faction::Necros,
            Card::VarrickTheNecromancer => Faction::Necros,

            Card::BroelynLoreweaver => Faction::Wild,
            Card::CronTheBerserker => Faction::Wild,
            Card::DireWolf => Faction::Wild,
            Card::ElvenCurse => Faction::Wild,
            Card::ElvenGift => Faction::Wild,
            Card::GrakStormGiant => Faction::Wild,
            Card::NaturesBounty => Faction::Wild,
            Card::OrcGrunt => Faction::Wild,
            Card::Rampage => Faction::Wild,
            Card::TorgenRocksplitter => Faction::Wild,
            Card::Spark => Faction::Wild,
            Card::WolfForm => Faction::Wild,
            Card::WolfShaman => Faction::Wild,
        }
    }

    pub fn defense(&self) -> Option<usize> {
        let life = match self {
            Card::ArkusImperialDragon => 6,
            Card::DarianWarMage => 5,
            Card::CristovTheJust => 5,
            Card::KrakaHighPriest => 6,
            Card::MasterWeyan => 4,
            Card::TithePriest => 3,
            Card::ManAtArms => 4,
            Card::BorgOgreMercenary => 6,
            Card::MyrosGuildMage => 3,
            Card::ParovTheEnforcer => 5,
            Card::RakeMasterAssassin => 7,
            Card::RasmusTheSmuggler => 5,
            Card::StreetThug => 4,
            Card::CultPriest => 4,
            Card::DeathCultist => 3,
            Card::RaylaEndweaver => 4,
            Card::KrythosMasterVampire => 6,
            Card::LysTheUnseen => 5,
            Card::TyrannorTheDevourer => 6,
            Card::VarrickTheNecromancer => 3,
            Card::BroelynLoreweaver => 6,
            Card::CronTheBerserker => 6,
            Card::DireWolf => 5,
            Card::GrakStormGiant => 7,
            Card::OrcGrunt => 3,
            Card::TorgenRocksplitter => 7,
            Card::WolfShaman => 5,
            _ => return None,
        };
        Some(life)
    }

    pub fn is_champion(&self) -> bool {
        match self {
            Card::ArkusImperialDragon
            | Card::DarianWarMage
            | Card::CristovTheJust
            | Card::KrakaHighPriest
            | Card::MasterWeyan
            | Card::TithePriest
            | Card::BorgOgreMercenary
            | Card::MyrosGuildMage
            | Card::ParovTheEnforcer
            | Card::RakeMasterAssassin
            | Card::RasmusTheSmuggler
            | Card::StreetThug
            | Card::CultPriest
            | Card::DeathCultist
            | Card::ManAtArms
            | Card::RaylaEndweaver
            | Card::KrythosMasterVampire
            | Card::LysTheUnseen
            | Card::TyrannorTheDevourer
            | Card::VarrickTheNecromancer
            | Card::BroelynLoreweaver
            | Card::CronTheBerserker
            | Card::DireWolf
            | Card::GrakStormGiant
            | Card::OrcGrunt
            | Card::TorgenRocksplitter
            | Card::WolfShaman => true,
            _ => false,
        }
    }

    pub fn is_guard(&self) -> bool {
        match self {
            Card::ArkusImperialDragon
            | Card::CristovTheJust
            | Card::MasterWeyan
            | Card::BorgOgreMercenary
            | Card::MyrosGuildMage
            | Card::ParovTheEnforcer
            | Card::DeathCultist
            | Card::LysTheUnseen
            | Card::TyrannorTheDevourer
            | Card::ManAtArms
            | Card::DireWolf
            | Card::ElvenCurse
            | Card::GrakStormGiant
            | Card::OrcGrunt
            | Card::TorgenRocksplitter => true,
            _ => false,
        }
    }

    pub fn is_object(&self) -> bool {
        match self {
            Card::Gold | Card::Ruby | Card::Dagger | Card::ShortSword | Card::FireGem => true,
            _ => false,
        }
    }

    pub fn is_action(&self) -> bool {
        !self.is_champion() && !self.is_object()
    }

    pub fn cost(&self) -> Option<usize> {
        let cost = match self {
            Card::FireGem => 2,

            Card::ArkusImperialDragon => 8,
            Card::CloseRanks => 3,
            Card::Command => 5,
            Card::DarianWarMage => 4,
            Card::Domination => 7,
            Card::CristovTheJust => 5,
            Card::KrakaHighPriest => 6,
            Card::ManAtArms => 3,
            Card::MasterWeyan => 4,
            Card::RallyTheTroops => 4,
            Card::Recruit => 2,
            Card::TithePriest => 2,
            Card::Taxation => 1,
            Card::WordOfPower => 6,

            Card::BorgOgreMercenary => 6,
            Card::Bribe => 3,
            Card::DeathThreat => 3,
            Card::Deception => 5,
            Card::FireBomb => 8,
            Card::HitJob => 4,
            Card::Intimidation => 2,
            Card::MyrosGuildMage => 5,
            Card::ParovTheEnforcer => 5,
            Card::Profit => 1,
            Card::RakeMasterAssassin => 7,
            Card::RasmusTheSmuggler => 4,
            Card::SmashAndGrab => 6,
            Card::StreetThug => 3,

            Card::CultPriest => 3,
            Card::DarkEnergy => 4,
            Card::DarkReward => 5,
            Card::DeathCultist => 2,
            Card::DeathTouch => 1,
            Card::RaylaEndweaver => 4,
            Card::Influence => 2,
            Card::KrythosMasterVampire => 7,
            Card::LifeDrain => 6,
            Card::LysTheUnseen => 6,
            Card::TheRot => 3,
            Card::TyrannorTheDevourer => 8,
            Card::VarrickTheNecromancer => 5,

            Card::BroelynLoreweaver => 4,
            Card::CronTheBerserker => 6,
            Card::DireWolf => 5,
            Card::ElvenCurse => 3,
            Card::ElvenGift => 2,
            Card::GrakStormGiant => 8,
            Card::NaturesBounty => 4,
            Card::OrcGrunt => 3,
            Card::Rampage => 6,
            Card::TorgenRocksplitter => 7,
            Card::Spark => 1,
            Card::WolfForm => 5,
            Card::WolfShaman => 2,

            _ => return None,
        };

        Some(cost)
    }

    pub fn primary_ability(&self) -> Option<Vec<Effect>> {
        let effects = match self {
            Card::Gold => vec![Effect::Gold(1)],
            Card::Ruby => vec![Effect::Gold(2)],
            Card::Dagger => vec![Effect::Combat(1)],
            Card::ShortSword => vec![Effect::Combat(2)],
            Card::FireGem => vec![Effect::Gold(2)],

            Card::Spark => vec![Effect::Combat(3), Effect::OpponentDiscards(1)],
            Card::Influence => vec![Effect::Gold(3)],
            Card::DeathTouch => vec![
                Effect::Combat(2),
                Effect::Choice(vec![Effect::Nothing], vec![Effect::Sacrifice(1)]),
            ],

            _ => {
                if !self.is_champion() {
                    panic!("Unimplemented primary ability");
                }
                return None;
            }
        };
        Some(effects)
    }

    pub fn expend_ability(&self) -> Option<Vec<Effect>> {
        let effect = match self {
            Card::WolfShaman => vec![
                Effect::Combat(2),
                Effect::CombatPer(
                    1,
                    PerAmount::AdditionalFactionCard(Card::WolfShaman.faction()),
                ),
            ],
            Card::TithePriest => vec![Effect::Choice(
                vec![Effect::Gold(1)],
                vec![Effect::HealPer(1, PerAmount::Champion)],
            )],
            Card::ManAtArms => vec![
                Effect::Combat(2),
                Effect::CombatPer(1, PerAmount::AdditionalGuardian),
            ],
            _ => {
                if self.is_champion() {
                    panic!("Unimplemented expend ability");
                }
                return None;
            }
        };
        Some(effect)
    }

    pub fn ally_ability(&self) -> Option<Vec<Effect>> {
        let effect = match self {
            Card::Spark => vec![Effect::Combat(2)],
            Card::DeathTouch => vec![Effect::Combat(2)],
            _ => return None,
        };
        Some(effect)
    }

    pub fn sacrifice_ability(&self) -> Option<Vec<Effect>> {
        let effects = match self {
            Card::FireGem => vec![Effect::Combat(3)],
            Card::Influence => vec![Effect::Combat(3)],
            _ => return None,
        };
        Some(effects)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum Effect {
    Gold(usize),
    Combat(usize),
    Heal(usize),
    Draw(usize),

    Choice(Vec<Effect>, Vec<Effect>),

    CombatPer(usize, PerAmount),
    HealPer(usize, PerAmount),
    Nothing,
    PutOverDeckFromDiscard,
    PutInHandFromDiscard,
    StunChampion,
    PrepareChampion,
    Sacrifice(usize),
    OpponentDiscards(usize),
    NextPurchaseToHand,
    NextActionPurchaseToTopOfDeck,
    NextPurchaseToTopOfDeck,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum PerAmount {
    AdditionalFactionCard(Faction),
    Champion,
    AdditionalChampion,
    AdditionalGuardian,
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
    AttackPlayerChampion(usize, usize),
    PurchaseFromShop(usize),
    Discard(usize),
    PurchaseFireGem,
    EndTurn,
}

impl State {
    pub fn apply_effects(
        &mut self,
        mut effects: Vec<Effect>,
        mut effect_args: Vec<EffectArgument>,
    ) -> Result<(), &'static str> {
        effects.reverse();
        effect_args.reverse();
        while !effects.is_empty() {
            match effects.pop().unwrap() {
                Effect::Gold(x) => self.mats[self.current_player].gold += x,
                Effect::Combat(x) => self.mats[self.current_player].combat += x,
                Effect::Heal(x) => self.mats[self.current_player].lives += x,
                Effect::Nothing => {}
                Effect::Draw(x) => {
                    let ref mut mat = self.mats[self.current_player];
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
                Effect::OpponentDiscards(x) => {
                    if let Some(EffectArgument::Opponent(o)) = effect_args.pop() {
                        self.mats[o].must_discard += x;
                    } else {
                        return Err("Wrong arguments, expected opponent");
                    }
                }
                Effect::Choice(first, second) => {
                    let mut branch = match effect_args.pop() {
                        Some(EffectArgument::ChooseFirst) => first,
                        Some(EffectArgument::ChooseSecond) => second,
                        _ => return Err("Wrong arguments, expected choice"),
                    };
                    branch.reverse();
                    effects.append(&mut branch);
                }
                Effect::Sacrifice(amount) => {
                    for _ in 0..amount {
                        match effect_args.pop() {
                            Some(EffectArgument::CardInHand(i)) => {
                                let card = self.mats[self.current_player].hand.remove(i);
                                self.sacrificed.push(card);
                            }
                            Some(EffectArgument::CardInDiscard(i)) => {
                                let card = self.mats[self.current_player].discard.remove(i);
                                self.sacrificed.push(card);
                            }
                            _ => return Err("Wrong arguments, expected card in hand or discard"),
                        };
                    }
                }
                Effect::HealPer(x, times) => {
                    let times = self.calculate_times(times);
                    effects.push(Effect::Heal(x * times));
                }
                Effect::CombatPer(x, times) => {
                    let times = self.calculate_times(times);
                    effects.push(Effect::Combat(x * times));
                }
                _ => return Err("Unsupported effect"),
            }
        }
        Ok(())
    }

    fn calculate_times(&self, times: PerAmount) -> usize {
        let sub = match times {
            PerAmount::Champion => 0,
            _ => 1,
        };
        let filter = |card: &Card| match times.clone() {
            PerAmount::Champion => card.is_champion(),
            PerAmount::AdditionalChampion => card.is_champion(),
            PerAmount::AdditionalFactionCard(f) => card.faction() == f,
            PerAmount::AdditionalGuardian => card.is_guard(),
        };
        self.mats[self.current_player]
            .field
            .iter()
            .filter(|cif| filter(&cif.card))
            .count()
            - sub
    }

    pub fn do_action(&mut self, action: PlayerAction) -> Result<(), &'static str> {
        let mut state = self.clone();

        if state.mats[state.current_player].must_discard > 0 {
            if let PlayerAction::Discard(card_in_hand) = action {
                let ref mut mat = state.mats[state.current_player];
                if card_in_hand >= mat.hand.len() {
                    return Err("No such card in hand");
                }
                mat.discard.push(mat.hand.remove(card_in_hand));
                mat.must_discard -= 1;

                *self = state;
                return Ok(());
            } else {
                return Err("Must discard first");
            }
        }

        match action {
            PlayerAction::Play(position, effect_args) => {
                let ref mut mat = state.mats[state.current_player];
                if position >= mat.hand.len() {
                    return Err("No such card in hand");
                }

                let card = mat.hand.remove(position);
                mat.field.push(CardInField::new(card.clone()));

                if let Some(effects) = card.primary_ability() {
                    state.apply_effects(effects, effect_args)?;
                }
            }

            PlayerAction::ActivateSacrificeAbility(card_in_field, effect_args) => {
                let ref mut mat = state.mats[state.current_player];
                if card_in_field >= mat.field.len() {
                    return Err("No such card in field");
                }

                let card = mat.field.remove(card_in_field).card;
                state.sacrificed.push(card.clone());

                if let Some(effects) = card.sacrifice_ability() {
                    state.apply_effects(effects, effect_args)?;
                } else {
                    return Err("No such sacrifice ability");
                }
            }

            PlayerAction::ActivateExpendAbility(card_in_field, effect_args) => {
                let ref mut mat = state.mats[state.current_player];
                if card_in_field >= mat.field.len() {
                    return Err("No such card in field");
                }
                if mat.field[card_in_field].expend_ability_used {
                    return Err("Expend ability already used");
                }
                let card = mat.field[card_in_field].card.clone();

                if let Some(effects) = card.expend_ability() {
                    mat.field[card_in_field].expend_ability_used = true;
                    state.apply_effects(effects, effect_args)?;
                } else {
                    return Err("Card does not have expend ability");
                }
            }

            PlayerAction::ActivateAllyAbility(card_in_field, effect_args) => {
                let ref mut mat = state.mats[state.current_player];
                if card_in_field >= mat.field.len() {
                    return Err("No such card in field");
                }
                if mat.field[card_in_field].ally_ability_used {
                    return Err("Ally ability already used");
                }

                let card = mat.field[card_in_field].card.clone();

                if let Some(effects) = card.ally_ability() {
                    if mat
                        .field
                        .iter()
                        .filter(|cif| cif.card.faction() == card.faction())
                        .count()
                        < 2
                    {
                        return Err("No ally in field");
                    }
                    mat.field[card_in_field].ally_ability_used = true;
                    state.apply_effects(effects, effect_args)?;
                } else {
                    return Err("Card does not have ally ability");
                }
            }

            PlayerAction::EndTurn => {
                let ref mut mat = state.mats[state.current_player];

                mat.gold = 0;
                mat.combat = 0;
                mat.next_action_purchase_to_top_of_deck = 0;
                mat.next_purchase_to_top_of_deck = 0;
                mat.next_purchase_to_hand = 0;

                let mut to_discard: Vec<_> = mat
                    .field
                    .iter()
                    .filter(|cif| !cif.card.is_champion())
                    .map(|cif| cif.card.clone())
                    .collect();
                mat.field.retain(|cif| cif.card.is_champion());
                for cif in mat.field.iter_mut() {
                    cif.expend_ability_used = false;
                    cif.ally_ability_used = false;
                }
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
                if state.mats[player]
                    .field
                    .iter()
                    .any(|cif| cif.card.is_guard())
                {
                    return Err("Can't attack player player with guards");
                }
                if state.mats[state.current_player].combat < amount {
                    return Err("Not enough combat");
                }
                state.mats[state.current_player].combat -= amount;
                state.mats[player].lives -= amount;
            }

            PlayerAction::AttackPlayerChampion(player, champion) => {
                if player >= state.players {
                    return Err("No such player");
                }
                if state.current_player == player {
                    return Err("Player can't attack his own champions");
                }
                if champion >= state.mats[player].field.len() {
                    return Err("No such card in field");
                }
                if !state.mats[player].field[champion].card.is_champion() {
                    return Err("Target card is not a champion");
                }
                let def = state.mats[player].field[champion].card.defense().unwrap();
                if def > state.mats[state.current_player].combat {
                    return Err("Not enough combat");
                }
                state.mats[state.current_player].combat -= def;
                let card = state.mats[player].field.remove(champion).card;
                state.mats[player].discard.push(card);
            }

            _ => return Err("Unsupported action"),
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
        assert_eq!(p1.discard.len(), 0);
        assert_eq!(p1.lives, 50);
        assert_eq!(p1.combat, 0);
        assert_eq!(p1.gold, 0);
        assert_eq!(p1.must_discard, 0);

        assert_eq!(p2.hand.len(), 5);
        assert_eq!(p2.deck.len(), 5);
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

        assert_vec_eq(
            &state.mats[p1].hand,
            &vec![Card::Gold, Card::ShortSword, Card::Dagger],
        );
        state.do_action(PlayerAction::Play(0, vec![]))?;
        {
            assert_vec_eq(&state.mats[p1].hand, &vec![Card::ShortSword, Card::Dagger]);
            assert_vec_eq(
                &state.mats[p1]
                    .field
                    .iter()
                    .map(|cif| cif.card.clone())
                    .collect::<Vec<Card>>(),
                &vec![Card::Gold],
            );
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
        }

        assert_vec_eq(
            &state.mats[p2].hand,
            &vec![Card::Gold, Card::Gold, Card::Gold, Card::Dagger, Card::Gold],
        );
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

        state.do_action(PlayerAction::EndTurn)?;

        assert_vec_eq(
            &state.mats[p1].hand,
            &vec![Card::Gold, Card::Gold, Card::Gold, Card::Gold, Card::Gold],
        );
        for _ in 0..5 {
            state.do_action(PlayerAction::Play(0, vec![]))?;
        }

        {
            assert_eq!(state.mats[p1].gold, 5);
        }

        state.do_action(PlayerAction::PurchaseFireGem)?;
        state.do_action(PlayerAction::PurchaseFireGem)?;
        state.do_action(PlayerAction::PurchaseFromShop(0))?;
        state.do_action(PlayerAction::EndTurn)?;

        assert_vec_eq(
            &state.mats[p2].hand,
            &vec![
                Card::Ruby,
                Card::Gold,
                Card::ShortSword,
                Card::Gold,
                Card::Gold,
            ],
        );
        for _ in 0..5 {
            state.do_action(PlayerAction::Play(0, vec![]))?;
        }
        state.do_action(PlayerAction::AttackPlayer(p1, 2))?;
        state.do_action(PlayerAction::PurchaseFireGem)?;
        state.do_action(PlayerAction::PurchaseFireGem)?;
        state.do_action(PlayerAction::PurchaseFromShop(0))?;
        state.do_action(PlayerAction::EndTurn)?;

        assert_vec_eq(
            &state.mats[p1].hand,
            &vec![Card::Ruby, Card::Gold, Card::Dagger, Card::Gold, Card::Gold],
        );
        for _ in 0..5 {
            state.do_action(PlayerAction::Play(0, vec![]))?;
        }
        state.do_action(PlayerAction::AttackPlayer(p2, 1))?;
        state.do_action(PlayerAction::PurchaseFireGem)?;
        state.do_action(PlayerAction::PurchaseFireGem)?;
        state.do_action(PlayerAction::PurchaseFromShop(0))?;
        state.do_action(PlayerAction::EndTurn)?;

        assert_vec_eq(
            &state.mats[p2].hand,
            &vec![
                Card::Ruby,
                Card::FireGem,
                Card::Gold,
                Card::Gold,
                Card::Gold,
            ],
        );
        state.do_action(PlayerAction::Play(1, vec![]))?;
        {
            assert_eq!(state.mats[p2].gold, 2);
            assert_eq!(state.mats[p2].combat, 0);
        }
        state.do_action(PlayerAction::ActivateSacrificeAbility(0, vec![]))?;
        {
            assert_eq!(state.mats[p2].gold, 2);
            assert_eq!(state.mats[p2].combat, 3);
            assert_eq!(state.mats[p2].field.len(), 0);
            assert_eq!(state.sacrificed.len(), 1);
            assert_eq!(state.sacrificed[0], Card::FireGem);
        }
        state.do_action(PlayerAction::EndTurn)?;

        assert_vec_eq(
            &state.mats[p1].hand,
            &vec![
                Card::Gold,
                Card::Spark,
                Card::FireGem,
                Card::ShortSword,
                Card::FireGem,
            ],
        );
        state.do_action(PlayerAction::Play(1, vec![EffectArgument::Opponent(p2)]))?;
        {
            assert_eq!(state.mats[p1].gold, 0);
            assert_eq!(state.mats[p1].combat, 3);
            assert_eq!(state.mats[p2].must_discard, 1);
        }
        state.do_action(PlayerAction::EndTurn)?;

        assert_vec_eq(
            &state.mats[p2].hand,
            &vec![
                Card::Spark,
                Card::FireGem,
                Card::Gold,
                Card::ShortSword,
                Card::Spark,
            ],
        );
        state
            .do_action(PlayerAction::Play(1, vec![]))
            .expect_err("Should not allow to play cards now");
        state.do_action(PlayerAction::Discard(2))?;
        assert_vec_eq(
            &state.mats[p2].hand,
            &vec![Card::Spark, Card::FireGem, Card::ShortSword, Card::Spark],
        );
        state.do_action(PlayerAction::Play(3, vec![EffectArgument::Opponent(p1)]))?;
        {
            assert_eq!(state.mats[p2].gold, 0);
            assert_eq!(state.mats[p2].combat, 3);
            assert_eq!(state.mats[p1].must_discard, 1);
        }
        state
            .do_action(PlayerAction::ActivateAllyAbility(0, vec![]))
            .expect_err("Shoult not be able to activate ability now");
        state.do_action(PlayerAction::Play(0, vec![EffectArgument::Opponent(p1)]))?;
        {
            assert_eq!(state.mats[p2].gold, 0);
            assert_eq!(state.mats[p2].combat, 6);
            assert_eq!(state.mats[p1].must_discard, 2);
        }
        state.do_action(PlayerAction::ActivateAllyAbility(0, vec![]))?;
        {
            assert_eq!(state.mats[p2].combat, 8);
        }
        state.do_action(PlayerAction::ActivateAllyAbility(1, vec![]))?;
        {
            assert_eq!(state.mats[p2].combat, 10);
        }
        state
            .do_action(PlayerAction::ActivateAllyAbility(1, vec![]))
            .expect_err("Shoult not be able to activate ability twice");
        {
            assert_eq!(state.mats[p2].combat, 10);
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

    fn play_all_hand(state: &mut State) -> Result<(), &'static str> {
        while !state.mats[state.current_player].hand.is_empty() {
            state.do_action(PlayerAction::Play(0, vec![]))?;
        }
        Ok(())
    }

    fn purchase(state: &mut State, card: Card) -> Result<(), &'static str> {
        for (i, c) in state.shop.iter().enumerate() {
            if *c == card {
                return state.do_action(PlayerAction::PurchaseFromShop(i));
            }
        }
        panic!(format!("No {:?} in shop", card));
    }

    fn attack_all(state: &mut State) -> Result<(), &'static str> {
        let opponent = (state.current_player + 1) % 2;
        let amount = state.mats[state.current_player].combat;
        state.do_action(PlayerAction::AttackPlayer(opponent, amount))
    }

    #[test]
    fn second_test_run() -> Result<(), &'static str> {
        let mut state = State::new(2, &Setup::base(), SRng::new(14279));
        let p1 = state.current_player;
        let p2 = (state.current_player + 1) % 2;
        assert_vec_eq(
            &state.shop,
            &vec![
                Card::TithePriest,
                Card::Influence,
                Card::WolfShaman,
                Card::ElvenCurse,
                Card::LysTheUnseen,
                Card::DeathTouch,
            ],
        );

        // p1 turn
        play_all_hand(&mut state)?;
        purchase(&mut state, Card::WolfShaman)?;
        attack_all(&mut state)?;
        state.do_action(PlayerAction::EndTurn)?;

        // p2 turn
        play_all_hand(&mut state)?;
        purchase(&mut state, Card::TithePriest)?;
        purchase(&mut state, Card::Influence)?;
        attack_all(&mut state)?;
        state.do_action(PlayerAction::EndTurn)?;

        // p1 turn
        play_all_hand(&mut state)?;
        purchase(&mut state, Card::DeathTouch)?;
        purchase(&mut state, Card::WolfShaman)?;
        attack_all(&mut state)?;
        state.do_action(PlayerAction::EndTurn)?;

        // p2 turn
        play_all_hand(&mut state)?;
        purchase(&mut state, Card::ManAtArms)?;
        attack_all(&mut state)?;
        state.do_action(PlayerAction::EndTurn)?;

        // p1 turn
        play_all_hand(&mut state)?;
        purchase(&mut state, Card::ElvenCurse)?;
        attack_all(&mut state)?;
        state.do_action(PlayerAction::PurchaseFireGem)?;
        state.do_action(PlayerAction::EndTurn)?;

        // p2 turn
        play_all_hand(&mut state)?;
        purchase(&mut state, Card::LysTheUnseen)?;
        attack_all(&mut state)?;
        state.do_action(PlayerAction::EndTurn)?;

        // p1 turn
        state.do_action(PlayerAction::Play(
            0,
            vec![
                EffectArgument::ChooseSecond,
                EffectArgument::CardInDiscard(3),
            ],
        ))?;
        {
            // DeathTouch effects
            assert_eq!(state.sacrificed[0], Card::Gold);
            assert_eq!(state.mats[p1].discard.len(), 6);
            assert_eq!(state.mats[p1].combat, 2);
        }
        play_all_hand(&mut state)?;
        purchase(&mut state, Card::Bribe)?;
        attack_all(&mut state)?;
        state.do_action(PlayerAction::EndTurn)?;

        // p2 turn
        state.do_action(PlayerAction::Play(4, vec![]))?;
        state.do_action(PlayerAction::Play(2, vec![]))?;
        state.do_action(PlayerAction::ActivateExpendAbility(
            0,
            vec![EffectArgument::ChooseSecond],
        ))?;
        {
            // TithePriest effects
            assert_eq!(state.mats[p2].lives, 44);
            assert_eq!(state.mats[p2].gold, 0);
        }
        state.do_action(PlayerAction::ActivateExpendAbility(1, vec![]))?;
        {
            // ManAtArms effects
            assert_eq!(state.mats[p2].combat, 2);
        }
        attack_all(&mut state)?;
        state.do_action(PlayerAction::EndTurn)?;

        // p1 turn
        state.do_action(PlayerAction::Play(0, vec![]))?;
        state.do_action(PlayerAction::Play(0, vec![]))?;
        state.do_action(PlayerAction::ActivateExpendAbility(0, vec![]))?;
        state.do_action(PlayerAction::ActivateExpendAbility(1, vec![]))?;
        {
            // WolfShaman effects
            assert_eq!(state.mats[p1].combat, 6);
        }
        play_all_hand(&mut state)?;
        state.do_action(PlayerAction::PurchaseFireGem)?;
        attack_all(&mut state).expect_err("Must attack guardian first");
        state.do_action(PlayerAction::AttackPlayerChampion(p2, 1))?;
        {
            assert_eq!(state.mats[p2].field.len(), 1);
            assert_eq!(state.mats[p2].discard.last(), Some(&Card::ManAtArms));
        }
        attack_all(&mut state)?;
        state.do_action(PlayerAction::EndTurn)?;

        // p2 turn
        play_all_hand(&mut state)?;
        purchase(&mut state, Card::RaylaEndweaver)?;
        state.do_action(PlayerAction::ActivateExpendAbility(
            0,
            vec![EffectArgument::ChooseFirst],
        ))?;
        purchase(&mut state, Card::Taxation)?;
        attack_all(&mut state)?;
        state.do_action(PlayerAction::EndTurn)?;

        // ----------------------
        println!("Opponent:");
        println!("{:#?}", state.mats[(state.current_player + 1) % 2]);
        println!("Shop:");
        for (i, card) in state.shop.iter().enumerate() {
            println!(
                "  {}) {:?} - {:?} / {:?}",
                i,
                card,
                card.cost().unwrap(),
                card.faction()
            );
        }
        println!("Current player:");
        println!("{:#?}", state.mats[state.current_player]);
        panic!("!");
        // ----------------------

        Ok(())
    }
}
