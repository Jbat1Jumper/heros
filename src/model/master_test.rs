use super::{api::*, cards::*, master::*};
use crate::smallrng::*;

#[test]
fn test_initial_state() {
    let state = MasterBoard::new(2, &Setup::test(), SRng::new(0));

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
    let mut state = MasterBoard::new(2, &Setup::test(), SRng::new(0));
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
        .expect_err("Should not be able to activate ability twice");
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

fn play_all_hand(state: &mut MasterBoard) -> Result<Vec<BoardDelta>, &'static str> {
    let mut ds = vec![];
    while !state.mats[state.current_player].hand.is_empty() {
        ds.append(&mut state.do_action(PlayerAction::Play(0, vec![]))?);
    }
    Ok(ds)
}

fn purchase(state: &mut MasterBoard, card: Card) -> Result<Vec<BoardDelta>, &'static str> {
    for (i, c) in state.shop.iter().enumerate() {
        if *c == card {
            return state.do_action(PlayerAction::PurchaseFromShop(i));
        }
    }
    panic!(format!("No {:?} in shop", card));
}

fn attack_all(state: &mut MasterBoard) -> Result<Vec<BoardDelta>, &'static str> {
    let opponent = (state.current_player + 1) % 2;
    let amount = state.mats[state.current_player].combat;
    state.do_action(PlayerAction::AttackPlayer(opponent, amount))
}

#[test]
fn second_test_run() -> Result<(), &'static str> {
    let mut state = MasterBoard::new(2, &Setup::base(), SRng::new(14279));
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

    // p1 turn
    play_all_hand(&mut state)?;
    state.do_action(PlayerAction::ActivateSacrificeAbility(5, vec![]))?;
    state.do_action(PlayerAction::ActivateExpendAbility(0, vec![]))?;
    state.do_action(PlayerAction::ActivateExpendAbility(1, vec![]))?;
    state.do_action(PlayerAction::AttackPlayerChampion(p2, 0))?;
    attack_all(&mut state)?;
    purchase(&mut state, Card::Deception)?;
    purchase(&mut state, Card::DeathCultist)?;
    state.do_action(PlayerAction::EndTurn)?;

    // p2 turn
    state.do_action(PlayerAction::Play(4, vec![]))?;
    state.do_action(PlayerAction::ActivateExpendAbility(
        0,
        vec![
            EffectArgument::ChooseSecond,
            EffectArgument::CardInDiscard(3),
        ],
    ))?;
    {
        // Lys effects
        assert_eq!(state.sacrificed.last(), Some(&Card::Gold));
        assert_eq!(state.sacrificed.len(), 3);
    }
    play_all_hand(&mut state)?;
    state.do_action(PlayerAction::PurchaseFireGem)?;
    state.do_action(PlayerAction::PurchaseFireGem)?;
    state.do_action(PlayerAction::AttackPlayerChampion(p1, 0))?;
    attack_all(&mut state)?;
    state.do_action(PlayerAction::EndTurn)?;

    // p1 turn
    state.do_action(PlayerAction::Play(
        2,
        vec![
            EffectArgument::ChooseSecond,
            EffectArgument::CardInDiscard(1),
        ],
    ))?;
    state.do_action(PlayerAction::Play(2, vec![EffectArgument::Opponent(p2)]))?;
    state.do_action(PlayerAction::AttackPlayerChampion(p2, 0))?;
    state.do_action(PlayerAction::ActivateExpendAbility(0, vec![]))?;
    play_all_hand(&mut state)?;
    attack_all(&mut state)?;
    purchase(&mut state, Card::ElvenGift)?;
    state.do_action(PlayerAction::EndTurn)?;

    // p2 turn
    state.do_action(PlayerAction::Discard(0))?;
    play_all_hand(&mut state)?;
    purchase(&mut state, Card::CristovTheJust)?;
    attack_all(&mut state)?;
    state.do_action(PlayerAction::EndTurn)?;

    // p1 turn
    state.do_action(PlayerAction::Play(0, vec![EffectArgument::Opponent(p2)]))?;
    play_all_hand(&mut state)?;
    state.do_action(PlayerAction::ActivateExpendAbility(0, vec![]))?;
    state.do_action(PlayerAction::ActivateExpendAbility(2, vec![]))?;
    attack_all(&mut state)?;
    purchase(&mut state, Card::RasmusTheSmuggler)?;
    state.do_action(PlayerAction::EndTurn)?;

    // p2 turn
    state.do_action(PlayerAction::Discard(4))?;
    play_all_hand(&mut state)?;
    purchase(&mut state, Card::KrakaHighPriest)?;
    state.do_action(PlayerAction::ActivateSacrificeAbility(1, vec![]))?;
    state.do_action(PlayerAction::AttackPlayerChampion(p1, 1))?;
    state.do_action(PlayerAction::EndTurn)?;

    // p1 turn
    state.do_action(PlayerAction::Play(
        4,
        vec![EffectArgument::ChooseSecond, EffectArgument::CardInHand(1)],
    ))?;
    state.do_action(PlayerAction::Play(0, vec![]))?;
    play_all_hand(&mut state)?;
    purchase(&mut state, Card::BorgOgreMercenary)?;
    state.do_action(PlayerAction::EndTurn)?;

    // p2 turn
    play_all_hand(&mut state)?;
    state.do_action(PlayerAction::ActivateExpendAbility(2, vec![]))?;
    state.do_action(PlayerAction::ActivateExpendAbility(4, vec![]))?;
    state.do_action(PlayerAction::ActivateSacrificeAbility(3, vec![]))?;
    state.do_action(PlayerAction::AttackPlayerChampion(p1, 0))?;
    attack_all(&mut state)?;
    state.do_action(PlayerAction::EndTurn)?;

    // at this point they got bored...

    // p1 turn
    state.do_action(PlayerAction::EndTurn)?;
    // p2 turn
    state.do_action(PlayerAction::EndTurn)?;

    // p1 turn
    state.do_action(PlayerAction::Play(0, vec![]))?;
    state.do_action(PlayerAction::Play(0, vec![]))?;
    state.do_action(PlayerAction::Play(
        1,
        vec![EffectArgument::ChooseSecond, EffectArgument::CardInHand(1)],
    ))?;
    state.do_action(PlayerAction::Play(0, vec![]))?;
    state.do_action(PlayerAction::EndTurn)?;

    // p2 turn
    play_all_hand(&mut state)?;
    state.do_action(PlayerAction::ActivateExpendAbility(0, vec![]))?;
    state.do_action(PlayerAction::ActivateExpendAbility(1, vec![]))?;
    state.do_action(PlayerAction::ActivateExpendAbility(2, vec![]))?;
    state.do_action(PlayerAction::ActivateAllyAbility(2, vec![]))?;
    state.do_action(PlayerAction::ActivateAllyAbility(6, vec![]))?;
    {
        assert_eq!(state.mats[p2].lives, 29);
    }
    play_all_hand(&mut state)?;
    state.do_action(PlayerAction::ActivateExpendAbility(
        7,
        vec![
            EffectArgument::ChooseSecond,
            EffectArgument::CardInDiscard(2),
        ],
    ))?;
    state.do_action(PlayerAction::ActivateAllyAbility(1, vec![]))?;
    play_all_hand(&mut state)?;
    state.do_action(PlayerAction::AttackPlayerChampion(p1, 2))?;
    state.do_action(PlayerAction::AttackPlayerChampion(p1, 1))?;
    attack_all(&mut state)?;
    purchase(&mut state, Card::Recruit)?;
    state.do_action(PlayerAction::EndTurn)?;

    // ----------------------
    // lets_see(&state);
    // ----------------------

    Ok(())
}

fn lets_see(state: &MasterBoard) {
    println!("Opponent:");
    println!("{:#?}", state.mats[(state.current_player + 1) % 2]);
    println!("Shop:");
    for (i, card) in state.shop.iter().enumerate() {
        println!(
            "  {}) {:?} - {:?} / {:?}",
            i,
            card,
            card.cost(),
            card.faction()
        );
    }
    println!("Current player:");
    println!("{:#?}", state.mats[state.current_player]);
    panic!("!");
}

#[test]
fn test_master_board_2_board() -> Result<(), &'static str> {
    let mut master = MasterBoard::new(2, &Setup::base(), SRng::new(14279));

    let mut board = master.scoped_to(master.current_player);

    let mut deltas = vec![];
    deltas.append(&mut master.do_action(PlayerAction::Play(0, vec![]))?);
    deltas.append(&mut master.do_action(PlayerAction::Play(0, vec![]))?);
    deltas.append(&mut master.do_action(PlayerAction::Play(0, vec![]))?);
    deltas.append(&mut master.do_action(PlayerAction::PurchaseFireGem)?);

    for d in deltas {
        board.apply(d).map_err(|_| "Error appliying delta")?;
    }

    let new_board = master.scoped_to(master.current_player);

    assert_eq!(board, new_board);

    Ok(())
}


#[test]
fn missing_cards() {
    let cards = Setup::base().shop_deck;
    let mut unimplemented_cards = vec![];

    for c in cards {
        use std::panic;
        let card = c.clone();

        let res = panic::catch_unwind(|| {
            let eff = if c.is_action() {
                c.primary_ability()
            } else {
                c.expend_ability()
            };
            print!("{}", eff.unwrap().len());
        });

        if res.is_err() {
            unimplemented_cards.push(card);
        }
    }
    if !unimplemented_cards.is_empty() {
        panic!(format!("Unimplemented cards {:#?}", unimplemented_cards));
    }
}

