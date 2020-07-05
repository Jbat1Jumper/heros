use serde::{Deserialize, Serialize};

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

    pub fn defense(&self) -> usize {
        match self {
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
            Card::WolfShaman => 4,
            _ => 0,
        }
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

    pub fn cost(&self) -> usize {
        match self {
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

            _ => 0,
        }
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
                Effect::Choice(vec![], vec![Effect::Sacrifice(1)]),
            ],
            Card::Bribe => vec![Effect::Gold(3)],
            Card::ElvenCurse => vec![Effect::Combat(6), Effect::OpponentDiscards(1)],
            Card::Taxation => vec![Effect::Gold(2)],
            Card::Deception => vec![Effect::Gold(2), Effect::Draw(1)],
            Card::Profit => vec![Effect::Gold(2)],
            Card::LifeDrain => vec![
                Effect::Combat(8),
                Effect::Choice(vec![], vec![Effect::Sacrifice(1)]),
            ],
            Card::CloseRanks => vec![Effect::Combat(5), Effect::CombatPer(2, PerAmount::Champion)],
            Card::DeathThreat => vec![Effect::Combat(1), Effect::Draw(1)],
            _ => {
                if !self.is_champion() {
                    panic!(format!("Unimplemented primary ability for {:?}", self));
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
            Card::LysTheUnseen => vec![
                Effect::Combat(2),
                Effect::Choice(vec![], vec![Effect::Sacrifice(1), Effect::Combat(2)]),
            ],
            Card::DeathCultist => vec![Effect::Combat(2)],
            Card::RaylaEndweaver => vec![Effect::Combat(3)],
            Card::KrakaHighPriest => vec![Effect::Heal(2), Effect::Draw(1)],
            Card::StreetThug => vec![Effect::Choice(
                vec![Effect::Gold(1)],
                vec![Effect::Combat(2)],
            )],
            Card::ParovTheEnforcer => vec![Effect::Combat(3)],
            Card::OrcGrunt => vec![Effect::Combat(2)],
            _ => {
                if self.is_champion() {
                    panic!(format!("Unimplemented expend ability for {:?}", self));
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
            Card::ElvenCurse => vec![Effect::Combat(3)],
            Card::Taxation => vec![Effect::Heal(6)],
            Card::KrakaHighPriest => vec![Effect::HealPer(2, PerAmount::Champion)],
            Card::RaylaEndweaver => vec![Effect::Draw(1)],
            Card::Profit => vec![Effect::Combat(4)],
            Card::LifeDrain => vec![Effect::Draw(1)],
            Card::ParovTheEnforcer => vec![Effect::Draw(1)],
            Card::CloseRanks => vec![Effect::Heal(6)],
            Card::DeathThreat => vec![Effect::StunChampion],
            Card::OrcGrunt => vec![Effect::Draw(1)],
            _ => return None,
        };
        Some(effect)
    }

    pub fn sacrifice_ability(&self) -> Option<Vec<Effect>> {
        let effects = match self {
            Card::FireGem => vec![Effect::Combat(3)],
            Card::Influence => vec![Effect::Combat(3)],
            Card::Bribe => vec![Effect::NextActionPurchaseToTopOfDeck],
            _ => return None,
        };
        Some(effects)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
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
