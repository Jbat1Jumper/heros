use super::{api::*, cards::*};

fn create_test_board() -> Board {
    Board {
        shop: vec![Card::Bribe, Card::DarkEnergy, Card::Spark],
        shop_deck: 5,
        gems: 16,
        sacrificed: vec![],

        current_player: 0,
        players: 2,
        mats: vec![
            Mat {
                name: "Ana".into(),
                field: vec![],
                hand: 3,
                discard: vec![],
                deck: 7,
                lives: 50,
                combat: 0,
                gold: 0,
                must_discard: 0,
            },
            Mat {
                name: "Bob".into(),
                field: vec![],
                hand: 4,
                discard: vec![],
                deck: 6,
                lives: 50,
                combat: 0,
                gold: 0,
                must_discard: 0,
            },
        ],

        you: 0,
        your_hand: vec![Card::Gold, Card::Gold, Card::ShortSword],
    }
}

#[test]
fn test_board_delta_movements() {
    println!("Creating test board");
    let mut b = create_test_board();
    let opponent = (b.you + 1) % 2;

    println!("Moving card from hand to field");
    b.apply(BoardDelta::Move(
        Location::Hand(b.you),
        1,
        Location::Field(b.you),
        Some(Card::Gold),
    ))
    .expect("Could not apply");

    assert_eq!(b.mats[b.you].field.len(), 1);
    assert_eq!(b.mats[b.you].field[0], CardInField::new(Card::Gold));
    assert_eq!(b.mats[b.you].hand, 2);
    assert_eq!(b.your_hand.len(), 2);

    println!("Moving card from deck to opponent hand");
    b.apply(BoardDelta::Move(
        Location::Deck(opponent),
        0,
        Location::Hand(opponent),
        None,
    ))
    .expect("Could not apply");

    assert_eq!(b.mats[opponent].deck, 5);
    assert_eq!(b.mats[opponent].hand, 5);

    println!("Moving card from deck to player hand");
    b.apply(BoardDelta::Move(
        Location::Deck(b.you),
        0,
        Location::Hand(b.you),
        Some(Card::Dagger),
    ))
    .expect("Could not apply");

    assert_eq!(b.mats[b.you].deck, 6);
    assert_eq!(b.mats[b.you].hand, 3);
    assert_eq!(b.your_hand[2], Card::Dagger);

    println!("Moving card from field to discard");
    b.apply(BoardDelta::Move(
        Location::Field(b.you),
        0,
        Location::Discard(b.you),
        Some(Card::Gold),
    ))
    .expect("Could not apply");

    assert_eq!(b.mats[b.you].field.len(), 0);
    assert_eq!(b.mats[b.you].discard.len(), 1);
    assert_eq!(b.mats[b.you].discard[0], Card::Gold);

    println!("Moving card from hand to discard");
    b.apply(BoardDelta::Move(
        Location::Hand(b.you),
        1,
        Location::Discard(b.you),
        Some(Card::ShortSword),
    ))
    .expect("Could not apply");

    assert_eq!(b.mats[b.you].hand, 2);
    assert_eq!(b.mats[b.you].discard.len(), 2);
    assert_eq!(b.mats[b.you].discard[1], Card::ShortSword);

    println!("Moving card from discard to deck");
    b.apply(BoardDelta::Move(
        Location::Discard(b.you),
        0,
        Location::Deck(b.you),
        Some(Card::Gold),
    ))
    .expect("Could not apply");

    assert_eq!(b.mats[b.you].deck, 7);
    assert_eq!(b.mats[b.you].discard.len(), 1);
    assert_eq!(b.mats[b.you].discard[0], Card::ShortSword);

    println!("Moving card from shop to opponent discard");
    b.apply(BoardDelta::Move(
        Location::Shop,
        2,
        Location::Discard(opponent),
        Some(Card::Spark),
    ))
    .expect("Could not apply");

    assert_eq!(b.mats[opponent].discard.len(), 1);
    assert_eq!(b.shop.len(), 2);

    println!("Moving card from shop deck to shop");
    b.apply(BoardDelta::Move(
        Location::ShopDeck,
        0,
        Location::Shop,
        Some(Card::VarrickTheNecromancer),
    ))
    .expect("Could not apply");

    assert_eq!(b.shop_deck, 4);
    assert_eq!(b.shop.len(), 3);

    println!("Buying a fire gem");
    b.apply(BoardDelta::Move(
        Location::FireGems,
        0,
        Location::Hand(opponent),
        Some(Card::FireGem),
    ))
    .expect("Could not apply");

    assert_eq!(b.gems, 15);
    assert_eq!(b.mats[opponent].hand, 6);

    println!("Sorry, returning fire gem");
    b.apply(BoardDelta::Move(
        Location::Hand(opponent),
        0,
        Location::FireGems,
        Some(Card::FireGem),
    ))
    .expect("Could not apply");

    assert_eq!(b.gems, 16);
    assert_eq!(b.mats[opponent].hand, 5);
}

#[test]
fn test_board_delta_changes() {
    println!("Creating test board");
    let mut b = create_test_board();
    let opponent = (b.you + 1) % 2;

    b.apply(BoardDelta::IncreaseHealth(b.you, 2))
        .expect("Could not apply");
    assert_eq!(b.mats[b.you].lives, 52);

    b.apply(BoardDelta::IncreaseCombat(b.you, 10))
        .expect("Could not apply");
    assert_eq!(b.mats[b.you].combat, 10);

    b.apply(BoardDelta::DecreaseHealth(opponent, 10))
        .expect("Could not apply");
    assert_eq!(b.mats[opponent].lives, 40);

    b.apply(BoardDelta::DecreaseCombat(b.you, 10))
        .expect("Could not apply");
    assert_eq!(b.mats[b.you].combat, 0);

    b.apply(BoardDelta::IncreaseGold(b.you, 4))
        .expect("Could not apply");
    assert_eq!(b.mats[b.you].gold, 4);

    b.apply(BoardDelta::DecreaseGold(b.you, 2))
        .expect("Could not apply");
    b.apply(BoardDelta::DecreaseGold(b.you, 2))
        .expect("Could not apply");
    assert_eq!(b.mats[b.you].gold, 0);

    b.apply(BoardDelta::IncreaseDiscardAmount(opponent, 1))
        .expect("Could not apply");
    assert_eq!(b.mats[opponent].must_discard, 1);
    b.apply(BoardDelta::IncreaseDiscardAmount(opponent, 2))
        .expect("Could not apply");
    b.apply(BoardDelta::DecreaseDiscardAmount(opponent, 3))
        .expect("Could not apply");
    assert_eq!(b.mats[opponent].must_discard, 0);

    b.apply(BoardDelta::ChangeCurrentPlayer(opponent))
        .expect("Could not apply");
    assert_eq!(b.current_player, opponent);
}

#[test]
fn test_ability_flags() {
    println!("Creating test board");
    let mut b = create_test_board();

    b.apply(BoardDelta::Move(
        Location::Deck(b.you),
        0,
        Location::Hand(b.you),
        Some(Card::OrcGrunt),
    ))
    .expect("Could not apply");

    b.apply(BoardDelta::Move(
        Location::Deck(b.you),
        0,
        Location::Hand(b.you),
        Some(Card::OrcGrunt),
    ))
    .expect("Could not apply");

    b.apply(BoardDelta::Move(
        Location::Hand(b.you),
        3,
        Location::Field(b.you),
        Some(Card::OrcGrunt),
    ))
    .expect("Could not apply");

    b.apply(BoardDelta::Move(
        Location::Hand(b.you),
        3,
        Location::Field(b.you),
        Some(Card::OrcGrunt),
    ))
    .expect("Could not apply");

    b.apply(BoardDelta::SetExpendAbilityUsed(b.you, 0, true))
        .expect("Could not apply");

    assert_eq!(b.mats[b.you].field[0].expend_ability_used, true);

    b.apply(BoardDelta::SetAllyAbilityUsed(b.you, 1, true))
        .expect("Could not apply");

    assert_eq!(b.mats[b.you].field[1].ally_ability_used, true);

    b.apply(BoardDelta::SetExpendAbilityUsed(b.you, 0, false))
        .expect("Could not apply");

    assert_eq!(b.mats[b.you].field[0].expend_ability_used, false);
}
