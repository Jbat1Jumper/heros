use super::{
    api::{Board, BoardDelta, EffectArgument, Location, Mat, PlayerAction},
    cards::{Card, CardInField, Effect, PerAmount, Setup},
};
use crate::smallrng::{Rng, SRng};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MasterBoard {
    pub shop: Vec<Card>,
    pub shop_deck: Vec<Card>,
    pub gems: Vec<Card>,
    pub sacrificed: Vec<Card>,

    pub current_player: usize,
    pub players: usize,
    pub game_over: bool,
    pub mats: Vec<MasterMat>,
    pub rng: SRng,
}

impl MasterBoard {
    pub fn new(players: usize, setup: &Setup, mut rng: SRng) -> MasterBoard {
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
                MasterMat::new(
                    format!("Player {}", i + 1),
                    starting_cards,
                    &setup,
                    rng.fork(),
                )
            })
            .collect();

        MasterBoard {
            shop,
            players,
            shop_deck: shop_deck,
            gems: setup.gems.clone(),
            sacrificed: vec![],
            current_player,
            game_over: false,
            mats,
            rng,
        }
    }
}

pub fn draw(amount: usize, source: &mut Vec<Card>) -> Vec<Card> {
    source.split_off(source.len() - amount)
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MasterMat {
    pub name: String,
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

impl MasterMat {
    pub fn new(name: String, starting_cards: usize, setup: &Setup, mut rng: SRng) -> MasterMat {
        let mut deck = setup.player_deck.clone();
        rng.shuffle(&mut deck);
        let hand = draw(starting_cards, &mut deck);
        MasterMat {
            name,
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

    pub fn is_alive(&self) -> bool {
        self.lives > 0
    }
}

impl MasterBoard {
    pub fn apply_effects(
        &mut self,
        mut effects: Vec<Effect>,
        mut effect_args: Vec<EffectArgument>,
    ) -> Result<Vec<BoardDelta>, &'static str> {
        let mut deltas = vec![];
        effects.reverse();
        effect_args.reverse();
        while !effects.is_empty() {
            match effects.pop().unwrap() {
                Effect::Gold(x) => {
                    self.mats[self.current_player].gold += x;
                    deltas.push(BoardDelta::IncreaseGold(self.current_player, x));
                }
                Effect::Combat(x) => {
                    self.mats[self.current_player].combat += x;
                    deltas.push(BoardDelta::IncreaseCombat(self.current_player, x));
                }
                Effect::Heal(x) => {
                    self.mats[self.current_player].lives += x;
                    deltas.push(BoardDelta::IncreaseHealth(self.current_player, x));
                }
                Effect::Nothing => {}
                Effect::Draw(x) => {
                    let ref mut mat = self.mats[self.current_player];
                    for _ in 0..x {
                        if mat.deck.is_empty() && !mat.discard.is_empty() {
                            while !mat.discard.is_empty() {
                                let card = mat.discard.remove(0);
                                deltas.push(BoardDelta::Move(
                                    Location::Discard(self.current_player),
                                    0,
                                    Location::Deck(self.current_player),
                                    Some(card.clone()),
                                ));
                                mat.deck.push(card);
                            }

                            self.rng.shuffle(&mut mat.deck);
                            deltas.push(BoardDelta::ShuffleDeck(self.current_player));
                        }

                        if let Some(card) = mat.deck.pop() {
                            mat.hand.push(card.clone());
                            deltas.push(BoardDelta::Move(
                                Location::Deck(self.current_player),
                                0,
                                Location::Hand(self.current_player),
                                Some(card),
                            ));
                        }
                    }
                }
                Effect::OpponentDiscards(x) => {
                    if let Some(EffectArgument::Opponent(o)) = effect_args.pop() {
                        self.mats[o].must_discard += x;
                        deltas.push(BoardDelta::IncreaseDiscardAmount(o, x))
                    } else {
                        return Err("Wrong arguments, expected opponent");
                    }
                }
                Effect::PlayerDiscards(x) => {
                    self.mats[self.current_player].must_discard += x;
                    deltas.push(BoardDelta::IncreaseDiscardAmount(self.current_player, x))
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
                                self.sacrificed.push(card.clone());
                                deltas.push(BoardDelta::Move(
                                    Location::Hand(self.current_player),
                                    i,
                                    Location::Sacrifice,
                                    Some(card),
                                ));
                            }
                            Some(EffectArgument::CardInDiscard(i)) => {
                                let card = self.mats[self.current_player].discard.remove(i);
                                self.sacrificed.push(card.clone());
                                deltas.push(BoardDelta::Move(
                                    Location::Discard(self.current_player),
                                    i,
                                    Location::Sacrifice,
                                    Some(card),
                                ));
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
                Effect::NextActionPurchaseToTopOfDeck => {
                    self.mats[self.current_player].next_action_purchase_to_top_of_deck += 1;
                }
                Effect::PrepareChampion => match effect_args.pop() {
                    Some(EffectArgument::Champion(p, id)) if p == self.current_player => {
                        if id >= self.mats[p].field.len() {
                            return Err("No such card in field");
                        }
                        let ref mut cif = self.mats[p].field[id];
                        if !cif.card.is_champion() {
                            return Err("Target card is not a champion");
                        }
                        cif.expend_ability_used = false;
                        deltas.push(BoardDelta::SetExpendAbilityUsed(p, id, false));
                    }
                    _ => return Err("Wrong arguments, expected player champion"),
                },
                Effect::StunChampion => match effect_args.pop() {
                    Some(EffectArgument::Champion(p, id)) if p != self.current_player => {
                        if p >= self.players {
                            return Err("No such player");
                        }
                        if id >= self.mats[p].field.len() {
                            return Err("No such champion");
                        }
                        if !self.mats[p].field[id].card.is_champion() {
                            return Err("Target card is not a champion");
                        }

                        let card = self.mats[p].field.remove(id).card;
                        self.mats[p].discard.push(card.clone());
                        deltas.push(BoardDelta::Move(
                            Location::Field(p),
                            id,
                            Location::Discard(p),
                            Some(card),
                        ));
                    }
                    _ => return Err("Wrong arguments, exprected opponent champion"),
                },
                _ => return Err("Unsupported effect"),
            }
        }

        Ok(deltas)
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

    pub fn do_action(&mut self, action: PlayerAction) -> Result<Vec<BoardDelta>, &'static str> {
        if self.game_over {
            return Err("The game is already over");
        }

        let mut state = self.clone();
        let mut deltas = vec![];

        if state.mats[state.current_player].must_discard > 0 {
            if let PlayerAction::Discard(card_in_hand) = action {
                let ref mut mat = state.mats[state.current_player];
                if card_in_hand >= mat.hand.len() {
                    return Err("No such card in hand");
                }
                let card = mat.hand.remove(card_in_hand);
                mat.discard.push(card.clone());
                deltas.push(BoardDelta::Move(
                    Location::Hand(state.current_player),
                    card_in_hand,
                    Location::Discard(state.current_player),
                    Some(card.clone()),
                ));
                mat.must_discard -= 1;
                deltas.push(BoardDelta::DecreaseDiscardAmount(state.current_player, 1));

                *self = state;
                return Ok(deltas);
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

                deltas.push(BoardDelta::Move(
                    Location::Hand(state.current_player),
                    position,
                    Location::Field(state.current_player),
                    Some(card.clone()),
                ));

                if let Some(effects) = card.primary_ability() {
                    deltas.append(&mut state.apply_effects(effects, effect_args)?);
                }
            }

            PlayerAction::ActivateSacrificeAbility(card_in_field, effect_args) => {
                let ref mut mat = state.mats[state.current_player];
                if card_in_field >= mat.field.len() {
                    return Err("No such card in field");
                }

                let card = mat.field.remove(card_in_field).card;
                state.sacrificed.push(card.clone());
                deltas.push(BoardDelta::Move(
                    Location::Field(state.current_player),
                    card_in_field,
                    Location::Sacrifice,
                    Some(card.clone()),
                ));

                if let Some(effects) = card.sacrifice_ability() {
                    deltas.append(&mut state.apply_effects(effects, effect_args)?);
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
                deltas.push(BoardDelta::SetExpendAbilityUsed(
                    state.current_player,
                    card_in_field,
                    true,
                ));

                if let Some(effects) = card.expend_ability() {
                    mat.field[card_in_field].expend_ability_used = true;
                    deltas.append(&mut state.apply_effects(effects, effect_args)?);
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
                    deltas.push(BoardDelta::SetAllyAbilityUsed(
                        state.current_player,
                        card_in_field,
                        true,
                    ));
                    mat.field[card_in_field].ally_ability_used = true;
                    deltas.append(&mut state.apply_effects(effects, effect_args)?);
                } else {
                    return Err("Card does not have ally ability");
                }
            }

            PlayerAction::EndTurn => {
                let ref mut mat = state.mats[state.current_player];

                if mat.gold > 0 {
                    deltas.push(BoardDelta::DecreaseGold(state.current_player, mat.gold));
                    mat.gold = 0;
                }
                if mat.combat > 0 {
                    deltas.push(BoardDelta::DecreaseCombat(state.current_player, mat.combat));
                    mat.combat = 0;
                }

                mat.next_action_purchase_to_top_of_deck = 0;
                mat.next_purchase_to_top_of_deck = 0;
                if mat.next_purchase_to_hand > 0 {
                    mat.next_purchase_to_hand = 0;
                }

                while let Some((i, cif)) = mat
                    .field
                    .iter()
                    .enumerate()
                    .find(|(_, cif)| !cif.card.is_champion())
                {
                    deltas.push(BoardDelta::Move(
                        Location::Field(state.current_player),
                        i,
                        Location::Discard(state.current_player),
                        Some(cif.card.clone()),
                    ));
                    mat.discard.push(cif.card.clone());
                    mat.field.remove(i);
                }

                for (i, cif) in mat.field.iter_mut().enumerate() {
                    cif.expend_ability_used = false;
                    cif.ally_ability_used = false;

                    deltas.push(BoardDelta::SetExpendAbilityUsed(
                        state.current_player,
                        i,
                        false,
                    ));
                    deltas.push(BoardDelta::SetAllyAbilityUsed(
                        state.current_player,
                        i,
                        false,
                    ));
                }

                while mat.hand.len() > 0 {
                    deltas.push(BoardDelta::Move(
                        Location::Hand(state.current_player),
                        0,
                        Location::Discard(state.current_player),
                        Some(mat.hand[0].clone()),
                    ));
                    mat.discard.push(mat.hand.remove(0));
                }

                deltas.append(&mut state.apply_effects(vec![Effect::Draw(5)], vec![])?);

                state.current_player = (state.current_player + 1) % state.players;
                deltas.push(BoardDelta::ChangeCurrentPlayer(state.current_player));
            }

            PlayerAction::PurchaseFromShop(position) => {
                let ref mut mat = state.mats[state.current_player];

                if position >= state.shop.len() {
                    return Err("No such card in shop");
                }
                let card = state.shop[position].clone();

                let cost = card.cost();

                if mat.gold < cost {
                    return Err("Not enough gold");
                }

                mat.gold -= cost;
                deltas.push(BoardDelta::DecreaseGold(state.current_player, cost));

                if mat.next_purchase_to_hand > 0 {
                    mat.next_purchase_to_hand -= 1;
                    mat.hand.push(card.clone());
                    deltas.push(BoardDelta::Move(
                        Location::Shop,
                        position,
                        Location::Hand(state.current_player),
                        Some(card.clone()),
                    ));
                } else if mat.next_action_purchase_to_top_of_deck > 0 && card.is_action() {
                    mat.next_action_purchase_to_top_of_deck -= 1;
                    mat.deck.push(card.clone());
                    deltas.push(BoardDelta::Move(
                        Location::Shop,
                        position,
                        Location::Deck(state.current_player),
                        Some(card.clone()),
                    ));
                } else if mat.next_purchase_to_top_of_deck > 0 {
                    mat.next_purchase_to_top_of_deck -= 1;
                    mat.deck.push(card.clone());
                    deltas.push(BoardDelta::Move(
                        Location::Shop,
                        position,
                        Location::Deck(state.current_player),
                        Some(card.clone()),
                    ));
                } else {
                    mat.discard.push(card.clone());
                    deltas.push(BoardDelta::Move(
                        Location::Shop,
                        position,
                        Location::Discard(state.current_player),
                        Some(card.clone()),
                    ));
                }

                state.shop.remove(position);
                if let Some(card) = state.shop_deck.pop() {
                    deltas.push(BoardDelta::Move(
                        Location::ShopDeck,
                        0,
                        Location::Shop,
                        Some(card.clone()),
                    ));
                    state.shop.push(card);
                } else {
                }
            }

            PlayerAction::PurchaseFireGem => {
                let ref mut mat = state.mats[state.current_player];

                if state.gems.is_empty() {
                    return Err("No more fire gems");
                }
                let cost = Card::FireGem.cost();

                if mat.gold < cost {
                    return Err("Not enough gold");
                }
                mat.gold -= cost;
                mat.discard.push(state.gems.pop().unwrap());

                deltas.push(BoardDelta::DecreaseGold(state.current_player, cost));
                deltas.push(BoardDelta::Move(
                    Location::FireGems,
                    0,
                    Location::Discard(state.current_player),
                    Some(Card::FireGem),
                ));
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

                deltas.push(BoardDelta::DecreaseCombat(state.current_player, amount));
                state.mats[state.current_player].combat -= amount;
                deltas.push(BoardDelta::DecreaseHealth(player, amount));
                state.mats[player].lives = state.mats[player].lives.saturating_sub(amount);
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
                if !state.mats[player].field[champion].card.is_guard()
                    && state.mats[player]
                        .field
                        .iter()
                        .any(|cif| cif.card.is_guard())
                {
                    return Err("Can't attack champion player with guards");
                }

                let def = state.mats[player].field[champion].card.defense();
                if def > state.mats[state.current_player].combat {
                    return Err("Not enough combat");
                }
                state.mats[state.current_player].combat -= def;
                let card = state.mats[player].field.remove(champion).card;
                state.mats[player].discard.push(card.clone());
                deltas.push(BoardDelta::DecreaseCombat(state.current_player, def));
                deltas.push(BoardDelta::Move(
                    Location::Field(player),
                    champion,
                    Location::Discard(player),
                    Some(card),
                ));
            }

            _ => return Err("Unsupported action"),
        }

        if state.mats.iter().filter(|m| m.is_alive()).count() == 1 {
            deltas.push(BoardDelta::GameOver);
            state.game_over = true;
        }

        *self = state;
        Ok(deltas)
    }

    pub fn scoped_to(&self, player: usize) -> Board {
        Board {
            shop: self.shop.clone(),
            shop_deck: self.shop_deck.len(),
            gems: self.gems.len(),
            sacrificed: self.sacrificed.clone(),
            current_player: self.current_player,
            players: self.players,
            game_over: self.game_over,
            mats: self
                .mats
                .iter()
                .map(|mat| Mat {
                    name: mat.name.clone(),
                    field: mat.field.clone(),
                    hand: mat.hand.len(),
                    discard: mat.discard.clone(),
                    deck: mat.deck.len(),
                    lives: mat.lives,
                    combat: mat.combat,
                    gold: mat.gold,
                    must_discard: mat.must_discard,
                })
                .collect(),
            you: player,
            your_hand: self.mats[player].hand.clone(),
        }
    }
}
