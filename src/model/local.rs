use super::{api::*, cards::*, master::*};
use crate::smallrng::*;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Duration;

pub struct LocalServer {
    board: MasterBoard,
    remaining_deltas: Vec<Vec<BoardDelta>>,
    connections: Vec<LocalServerConnection>,
}

struct LocalServerConnection {
    deltas: Sender<BoardDelta>,
    actions: Receiver<PlayerAction>,
}

impl LocalServer {
    pub fn new(seed: u64, players: usize) -> (Self, Vec<LocalClient>) {
        let mut connections = vec![];
        let mut clients = vec![];
        let board = MasterBoard::new(2, &Setup::base(), SRng::new(seed));
        for i in 0..players {
            let (send_delta, receive_delta) = channel::<BoardDelta>();
            let (send_action, receive_action) = channel::<PlayerAction>();

            connections.push(LocalServerConnection {
                deltas: send_delta,
                actions: receive_action,
            });
            clients.push(LocalClient {
                board: board.scoped_to(i),
                player: i,
                send_action,
                receive_delta,
            });
        }
        let server = LocalServer {
            board,
            remaining_deltas: std::iter::repeat(vec![]).take(players).collect(),
            connections,
        };

        (server, clients)
    }

    pub fn process_action(&mut self) -> Result<(), &'static str> {
        let action = self.connections[self.board.current_player]
            .actions
            .recv_timeout(Duration::from_secs(5))
            .map_err(|_| "Timeout when waiting for player action")?;

        for player in 0..self.board.players {
            self.connections[player]
                .deltas
                .send(BoardDelta::PlayerDeclaredAction(action.clone()))
                .map_err(|_| "A client has died")?;
        }

        for delta in self.board.do_action(action.clone())? {
            for player in 0..self.board.players {
                self.connections[player]
                    .deltas
                    .send(self.hide_card_info(player, delta.clone()))
                    .map_err(|_| "A client has died")?;
            }
        }

        Ok(())
    }

    fn hide_card_info(&self, player: Player, delta: BoardDelta) -> BoardDelta {
        match delta {
            BoardDelta::Move(from, index, to, card) => BoardDelta::Move(
                from.clone(),
                index,
                to.clone(),
                if self.can_see(player, from) || self.can_see(player, to) {
                    card
                } else {
                    None
                },
            ),
            d => d,
        }
    }

    fn can_see(&self, player: Player, loc: Location) -> bool {
        match loc {
            Location::Deck(_) => false,
            Location::Discard(_) => true,
            Location::Field(_) => true,
            Location::FireGems => true,
            Location::Hand(p) => player == p,
            Location::Sacrifice => true,
            Location::Shop => true,
            Location::ShopDeck => false,
        }
    }
}

pub struct LocalClient {
    board: Board,
    player: usize,
    send_action: Sender<PlayerAction>,
    receive_delta: Receiver<BoardDelta>,
}

impl LocalClient {}

impl Api for LocalClient {
    type Error = &'static str;
    fn get_board<'a>(&'a self) -> &'a Board {
        &self.board
    }
    fn do_action(&mut self, action: PlayerAction) -> Result<(), Self::Error> {
        if self.board.current_player != self.player {
            Err("Not the current player")
        } else {
            self.send_action
                .send(action)
                .map_err(|_| "Failed to send action")?;
            Ok(())
        }
    }

    fn poll_deltas(&mut self) -> Vec<BoardDelta> {
        let deltas: Vec<_> = self.receive_delta.try_iter().collect();
        for d in deltas.iter() {
            self.board.apply(d.clone());
        }
        deltas
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::sync::Mutex;

    #[test]
    fn test_local1v1() -> Result<(), &'static str> {
        let (s, mut c) = LocalServer::new(777, 2);
        let s = Mutex::new(s);
        let wait = || s.lock().unwrap().process_action();
        let p1 = TestApi {
            wait,
            api: c.remove(0),
        };
        let p2 = TestApi {
            wait,
            api: c.remove(0),
        };
        test_game_777(p1, p2)
    }

    struct TestApi<A: Api, W>
    where
        W: FnMut() -> Result<(), A::Error>,
    {
        wait: W,
        api: A,
    }

    impl<A, W> Api for TestApi<A, W>
    where
        A: Api,
        W: FnMut() -> Result<(), A::Error>,
    {
        type Error = A::Error;
        fn get_board<'a>(&'a self) -> &'a Board {
            self.api.get_board()
        }
        fn do_action(&mut self, action: PlayerAction) -> Result<(), Self::Error> {
            self.api.do_action(action)?;
            (self.wait)()
        }
        fn poll_deltas(&mut self) -> Vec<BoardDelta> {
            self.api.poll_deltas()
        }
    }

    fn test_game_777<A: Api<Error = &'static str>>(
        mut p1: A,
        mut p2: A,
    ) -> Result<(), &'static str> {
        let mut b1: Board = p1.get_board().clone();
        assert_eq!(b1.current_player, b1.you);
        assert_eq!(
            b1.shop,
            vec![
                Card::KrakaHighPriest,
                Card::Profit,
                Card::StreetThug,
                Card::LifeDrain,
                Card::ParovTheEnforcer,
                Card::Influence
            ]
        );

        assert_eq!(b1.your_hand, vec![Card::Gold, Card::Gold, Card::Ruby,]);

        p1.do_action(PlayerAction::Play(0, vec![]))?;
        p1.do_action(PlayerAction::Play(0, vec![]))?;
        p1.do_action(PlayerAction::Play(0, vec![]))?;
        b1 += p1.poll_deltas();

        assert_eq!(b1.mats[b1.you].gold, 4);
        assert_eq!(b1.mats[b1.you].field.len(), 3);

        p1.do_action(PlayerAction::PurchaseFromShop(1))?;
        p1.do_action(PlayerAction::PurchaseFromShop(5))?;
        p1.do_action(PlayerAction::EndTurn)?;
        b1 += p1.poll_deltas();

        let mut b2: Board = p2.get_board().clone();
        b2 += p2.poll_deltas();

        for _ in 0..5 {
            p2.do_action(PlayerAction::Play(0, vec![]))?;
        }
        p2.do_action(PlayerAction::PurchaseFromShop(4))?;
        p2.do_action(PlayerAction::PurchaseFromShop(4))?;
        p2.do_action(PlayerAction::AttackPlayer(b1.you, 1))?;
        p2.do_action(PlayerAction::EndTurn)?;

        b1 += p1.poll_deltas();
        for _ in 0..5 {
            p1.do_action(PlayerAction::Play(0, vec![]))?;
        }
        p1.do_action(PlayerAction::AttackPlayer(b2.you, 3))?;
        p1.do_action(PlayerAction::PurchaseFromShop(1))?;
        p1.do_action(PlayerAction::EndTurn)?;

        b2 += p2.poll_deltas();
        for _ in 0..5 {
            p2.do_action(PlayerAction::Play(0, vec![]))?;
        }
        p2.do_action(PlayerAction::PurchaseFromShop(4))?;
        p2.do_action(PlayerAction::AttackPlayer(b1.you, 2))?;
        p2.do_action(PlayerAction::EndTurn)?;

        b1 += p1.poll_deltas();
        for _ in 0..5 {
            p1.do_action(PlayerAction::Play(0, vec![]))?;
        }
        p1.do_action(PlayerAction::ActivateExpendAbility(
            4,
            vec![EffectArgument::ChooseSecond],
        ))?;
        p1.do_action(PlayerAction::PurchaseFromShop(4))?;
        p1.do_action(PlayerAction::AttackPlayer(b2.you, 4))?;
        p1.do_action(PlayerAction::EndTurn)?;

        b2 += p2.poll_deltas();
        for _ in 0..5 {
            p2.do_action(PlayerAction::Play(0, vec![]))?;
        }
        p2.do_action(PlayerAction::PurchaseFromShop(1))?;
        p2.do_action(PlayerAction::AttackPlayer(b1.you, 2))?;
        p2.do_action(PlayerAction::EndTurn)?;

        b1 += p1.poll_deltas();
        p1.do_action(PlayerAction::Play(4, vec![]))?;
        for _ in 0..4 {
            p1.do_action(PlayerAction::Play(0, vec![]))?;
        }
        p1.do_action(PlayerAction::PurchaseFromShop(4))?;
        p1.do_action(PlayerAction::PurchaseFromShop(1))?;
        p1.do_action(PlayerAction::ActivateExpendAbility(
            0,
            vec![EffectArgument::ChooseSecond],
        ))?;
        p1.do_action(PlayerAction::ActivateAllyAbility(1, vec![]))?;
        p1.do_action(PlayerAction::AttackPlayer(b2.you, 6))?;
        p1.do_action(PlayerAction::EndTurn)?;

        for _ in 0..5 {
            p2.do_action(PlayerAction::Play(0, vec![]))?;
        }
        p2.do_action(PlayerAction::PurchaseFromShop(4))?;
        p2.do_action(PlayerAction::AttackPlayerChampion(b1.you, 0))?;
        p2.do_action(PlayerAction::AttackPlayer(b1.you, 2))?;
        p2.do_action(PlayerAction::EndTurn)?;

        for _ in 0..5 {
            p1.do_action(PlayerAction::Play(0, vec![]))?;
        }
        p1.do_action(PlayerAction::ActivateAllyAbility(3, vec![]))?;
        p1.do_action(PlayerAction::ActivateAllyAbility(2, vec![]))?;
        p1.do_action(PlayerAction::PurchaseFromShop(5))?;
        p1.do_action(PlayerAction::PurchaseFromShop(1))?;
        p1.do_action(PlayerAction::PurchaseFromShop(3))?;
        p1.do_action(PlayerAction::AttackPlayer(b2.you, 6))?;
        p1.do_action(PlayerAction::EndTurn)?;

        for _ in 0..5 {
            p2.do_action(PlayerAction::Play(0, vec![]))?;
        }
        p2.do_action(PlayerAction::PurchaseFromShop(0))?;
        p2.do_action(PlayerAction::AttackPlayer(b1.you, 1))?;
        p2.do_action(PlayerAction::EndTurn)?;
        p1.do_action(PlayerAction::Play(3, vec![]))?;
        p1.do_action(PlayerAction::Play(0, vec![EffectArgument::ChooseSecond]))?;
        p1.do_action(PlayerAction::Discard(1))?;
        for _ in 0..4 {
            p1.do_action(PlayerAction::Play(0, vec![]))?;
        }
        p1.do_action(PlayerAction::AttackPlayer(b2.you, 1))?;
        p1.do_action(PlayerAction::PurchaseFromShop(2))?;
        p1.do_action(PlayerAction::PurchaseFromShop(4))?;
        p1.do_action(PlayerAction::PurchaseFromShop(0))?;
        p1.do_action(PlayerAction::PurchaseFromShop(2))?;
        p1.do_action(PlayerAction::EndTurn)?;

        p2.do_action(PlayerAction::Play(0, vec![]))?;
        p2.do_action(PlayerAction::Play(0, vec![]))?;
        p2.do_action(PlayerAction::ActivateExpendAbility(0, vec![]))?;
        p2.do_action(PlayerAction::ActivateAllyAbility(1, vec![]))?;
        for _ in 0..3 {
            p2.do_action(PlayerAction::Play(0, vec![]))?;
        }
        p2.do_action(PlayerAction::PurchaseFromShop(1))?;
        p2.do_action(PlayerAction::AttackPlayer(b1.you, 11))?;
        p2.do_action(PlayerAction::EndTurn)?;

        p1.do_action(PlayerAction::Play(0, vec![]))?;
        p1.do_action(PlayerAction::Play(1, vec![]))?;
        p1.do_action(PlayerAction::ActivateAllyAbility(0, vec![]))?;
        p1.do_action(PlayerAction::ActivateExpendAbility(1, vec![]))?;
        p1.do_action(PlayerAction::ActivateAllyAbility(1, vec![]))?;
        p1.do_action(PlayerAction::Play(2, vec![]))?;
        for _ in 0..3 {
            p1.do_action(PlayerAction::Play(0, vec![]))?;
        }
        p1.do_action(PlayerAction::PurchaseFromShop(5))?;
        p1.do_action(PlayerAction::PurchaseFromShop(5))?;
        p1.do_action(PlayerAction::PurchaseFromShop(3))?;
        p1.do_action(PlayerAction::AttackPlayerChampion(b2.you, 0))?;
        p1.do_action(PlayerAction::AttackPlayer(b2.you, 3))?;
        p1.do_action(PlayerAction::EndTurn)?;

        p2.do_action(PlayerAction::Play(
            0,
            vec![
                EffectArgument::ChooseSecond,
                EffectArgument::CardInDiscard(2),
            ],
        ))?;
        for _ in 0..4 {
            p2.do_action(PlayerAction::Play(0, vec![]))?;
        }
        p2.do_action(PlayerAction::PurchaseFromShop(0))?;
        p2.do_action(PlayerAction::AttackPlayerChampion(b1.you, 0))?;
        p2.do_action(PlayerAction::AttackPlayer(b1.you, 3))?;
        p2.do_action(PlayerAction::EndTurn)?;

        p1.do_action(PlayerAction::Play(4, vec![]))?;
        p1.do_action(PlayerAction::Play(
            4,
            vec![EffectArgument::Opponent(b2.you)],
        ))?;
        for _ in 0..4 {
            p1.do_action(PlayerAction::Play(0, vec![]))?;
        }
        p1.do_action(PlayerAction::ActivateAllyAbility(5, vec![]))?;
        p1.do_action(PlayerAction::AttackPlayer(b2.you, 9))?;
        p1.do_action(PlayerAction::PurchaseFromShop(4))?;
        p1.do_action(PlayerAction::EndTurn)?;

        p2.do_action(PlayerAction::Discard(0))?;
        p2.do_action(PlayerAction::Play(0, vec![]))?;
        p2.do_action(PlayerAction::ActivateExpendAbility(0, vec![]))?;
        p2.do_action(PlayerAction::Play(2, vec![]))?;
        p2.do_action(PlayerAction::Play(
            2,
            vec![EffectArgument::ChooseSecond, EffectArgument::CardInHand(0)],
        ))?;
        p2.do_action(PlayerAction::ActivateAllyAbility(2, vec![]))?;
        p2.do_action(PlayerAction::Play(0, vec![]))?;
        p2.do_action(PlayerAction::ActivateAllyAbility(
            3,
            vec![EffectArgument::Champion(b2.you, 0)],
        ))?;
        p2.do_action(PlayerAction::ActivateExpendAbility(0, vec![]))?;
        p2.do_action(PlayerAction::Play(1, vec![]))?;
        p2.do_action(PlayerAction::Play(0, vec![]))?;
        p2.do_action(PlayerAction::ActivateExpendAbility(4, vec![]))?;
        p2.do_action(PlayerAction::ActivateAllyAbility(5, vec![]))?;
        p2.do_action(PlayerAction::ActivateAllyAbility(0, vec![]))?;
        p2.do_action(PlayerAction::PurchaseFromShop(3))?;
        p2.do_action(PlayerAction::PurchaseFromShop(4))?;
        p2.do_action(PlayerAction::EndTurn)?;
        // spare life
        p1.do_action(PlayerAction::Play(0, vec![]))?;
        p1.do_action(PlayerAction::Play(3, vec![]))?;
        p1.do_action(PlayerAction::ActivateAllyAbility(1, vec![]))?;
        for _ in 0..4 {
            p1.do_action(PlayerAction::Play(0, vec![]))?;
        }
        p1.do_action(PlayerAction::ActivateExpendAbility(0, vec![]))?;
        p1.do_action(PlayerAction::ActivateExpendAbility(1, vec![]))?;
        p1.do_action(PlayerAction::AttackPlayerChampion(b2.you, 1))?;
        p1.do_action(PlayerAction::AttackPlayer(b2.you, 3))?;
        p1.do_action(PlayerAction::PurchaseFireGem)?;
        p1.do_action(PlayerAction::EndTurn)?;

        p2.do_action(PlayerAction::ActivateExpendAbility(0, vec![]))?;
        for _ in 0..6 {
            p2.do_action(PlayerAction::Play(0, vec![]))?;
        }
        p2.do_action(PlayerAction::ActivateExpendAbility(3, vec![]))?;
        p2.do_action(PlayerAction::PurchaseFromShop(4))?;
        p2.do_action(PlayerAction::AttackPlayerChampion(b1.you, 1))?;
        p2.do_action(PlayerAction::AttackPlayer(b1.you, 2))?;
        p2.do_action(PlayerAction::EndTurn)?;

        p1.do_action(PlayerAction::Play(
            0,
            vec![EffectArgument::Opponent(b2.you)],
        ))?;
        p1.do_action(PlayerAction::Play(0, vec![]))?;
        p1.do_action(PlayerAction::Play(0, vec![EffectArgument::ChooseFirst]))?;
        p1.do_action(PlayerAction::Play(0, vec![]))?;
        p1.do_action(PlayerAction::Play(0, vec![]))?;
        p1.do_action(PlayerAction::ActivateAllyAbility(4, vec![]))?;
        p1.do_action(PlayerAction::ActivateAllyAbility(5, vec![]))?;
        p1.do_action(PlayerAction::ActivateExpendAbility(0, vec![]))?;
        p1.do_action(PlayerAction::ActivateExpendAbility(
            2,
            vec![EffectArgument::ChooseFirst],
        ))?;
        p1.do_action(PlayerAction::ActivateAllyAbility(1, vec![]))?;
        p1.do_action(PlayerAction::ActivateAllyAbility(3, vec![]))?;
        p1.do_action(PlayerAction::AttackPlayerChampion(b2.you, 0))?;
        p1.do_action(PlayerAction::AttackPlayer(b2.you, 16))?;
        p1.do_action(PlayerAction::PurchaseFromShop(2))?;
        p1.do_action(PlayerAction::EndTurn)?;

        p2.do_action(PlayerAction::Discard(1))?;
        p2.do_action(PlayerAction::Play(0, vec![]))?;
        p2.do_action(PlayerAction::ActivateAllyAbility(0, vec![]))?;
        p2.do_action(PlayerAction::ActivateExpendAbility(0, vec![]))?;
        for _ in 0..4 {
            p2.do_action(PlayerAction::Play(0, vec![]))?;
        }
        p2.do_action(PlayerAction::AttackPlayerChampion(b1.you, 0))?;
        p2.do_action(PlayerAction::AttackPlayerChampion(b1.you, 0))?;
        p2.do_action(PlayerAction::AttackPlayer(b1.you, 4))?;
        p2.do_action(PlayerAction::PurchaseFromShop(4))?;
        p2.do_action(PlayerAction::PurchaseFromShop(5))?;
        p2.do_action(PlayerAction::PurchaseFireGem)?;
        p2.do_action(PlayerAction::EndTurn)?;

        p1.do_action(PlayerAction::Play(3, vec![]))?;
        p1.do_action(PlayerAction::Play(
            2,
            vec![EffectArgument::Opponent(b2.you)],
        ))?;
        p1.do_action(PlayerAction::ActivateAllyAbility(0, vec![]))?;
        p1.do_action(PlayerAction::Play(
            0,
            vec![EffectArgument::Champion(b2.you, 0)],
        ))?;
        p1.do_action(PlayerAction::Play(3, vec![]))?;
        p1.do_action(PlayerAction::ActivateAllyAbility(3, vec![]))?;
        p1.do_action(PlayerAction::Play(2, vec![EffectArgument::ChooseSecond]))?;
        p1.do_action(PlayerAction::Discard(2))?;
        p1.do_action(PlayerAction::Play(
            0,
            vec![EffectArgument::Opponent(b2.you)],
        ))?;
        p1.do_action(PlayerAction::Play(0, vec![]))?;
        p1.do_action(PlayerAction::Play(0, vec![]))?;
        p1.do_action(PlayerAction::ActivateExpendAbility(0, vec![]))?;
        p1.do_action(PlayerAction::ActivateExpendAbility(3, vec![]))?;
        p1.do_action(PlayerAction::ActivateAllyAbility(4, vec![]))?;
        p1.do_action(PlayerAction::ActivateAllyAbility(5, vec![]))?;
        p1.do_action(PlayerAction::ActivateAllyAbility(6, vec![]))?;
        p1.do_action(PlayerAction::PurchaseFromShop(5))?;
        p1.do_action(PlayerAction::PurchaseFromShop(5))?;
        p1.do_action(PlayerAction::PurchaseFromShop(3))?;
        p1.do_action(PlayerAction::AttackPlayer(b2.you, 2))?;
        p1.do_action(PlayerAction::EndTurn)?;
        // spare life too
        p2.do_action(PlayerAction::Discard(0))?;
        p2.do_action(PlayerAction::Discard(2))?;
        p2.do_action(PlayerAction::Play(0, vec![]))?;
        p2.do_action(PlayerAction::Play(0, vec![]))?;
        p2.do_action(PlayerAction::Play(0, vec![]))?;
        p2.do_action(PlayerAction::ActivateExpendAbility(0, vec![]))?;
        p2.do_action(PlayerAction::ActivateExpendAbility(1, vec![]))?;
        p2.do_action(PlayerAction::PurchaseFromShop(4))?;
        p2.do_action(PlayerAction::ActivateSacrificeAbility(2, vec![]))?;
        p2.do_action(PlayerAction::AttackPlayerChampion(b1.you, 1))?;
        p2.do_action(PlayerAction::EndTurn)?;

        p1.do_action(PlayerAction::Play(
            0,
            vec![EffectArgument::Opponent(b2.you)],
        ))?;
        p1.do_action(PlayerAction::Play(
            2,
            vec![EffectArgument::Opponent(b2.you)],
        ))?;
        p1.do_action(PlayerAction::ActivateAllyAbility(0, vec![]))?;
        for _ in 0..4 {
            p1.do_action(PlayerAction::Play(0, vec![]))?;
        }
        p1.do_action(PlayerAction::ActivateExpendAbility(0, vec![]))?;
        for i in 1..6 {
            p1.do_action(PlayerAction::ActivateAllyAbility(i, vec![]))?;
        }
        p1.do_action(PlayerAction::AttackPlayerChampion(b2.you, 1))?;
        p1.do_action(PlayerAction::AttackPlayerChampion(b2.you, 0))?;
        p1.do_action(PlayerAction::AttackPlayer(b2.you, 16))?;
        p1.do_action(PlayerAction::PurchaseFromShop(0))?;
        p1.do_action(PlayerAction::EndTurn)?;

        p2.do_action(PlayerAction::Discard(0))?;
        p2.do_action(PlayerAction::Discard(0))?;

        p2.do_action(PlayerAction::Play(0, vec![]))?;
        p2.do_action(PlayerAction::Play(0, vec![]))?;
        p2.do_action(PlayerAction::Play(0, vec![EffectArgument::ChooseSecond, EffectArgument::CardInDiscard(6)]))?;
        p2.do_action(PlayerAction::ActivateAllyAbility(0, vec![]))?;
        p2.do_action(PlayerAction::AttackPlayerChampion(b1.you, 0))?;
        p2.do_action(PlayerAction::AttackPlayer(b1.you, 10))?;
        p2.do_action(PlayerAction::EndTurn)?;

        p1.do_action(PlayerAction::Play(3, vec![]))?;
        for _ in 0..5 {
            p1.do_action(PlayerAction::Play(0, vec![]))?;
        }
        for i in 1..4 {
            p1.do_action(PlayerAction::ActivateAllyAbility(i, vec![]))?;
        }
        p1.do_action(PlayerAction::PurchaseFromShop(1))?;
        p1.do_action(PlayerAction::PurchaseFromShop(4))?;
        p1.do_action(PlayerAction::AttackPlayer(b2.you, 5))?;
        p1.do_action(PlayerAction::EndTurn)?;

        p2.do_action(PlayerAction::Play(2, vec![EffectArgument::ChooseSecond, EffectArgument::CardInHand(3)]))?;
        p2.do_action(PlayerAction::Play(1, vec![EffectArgument::ChooseFirst]))?;
        p2.do_action(PlayerAction::ActivateAllyAbility(0, vec![]))?;
        p2.do_action(PlayerAction::ActivateAllyAbility(1, vec![]))?;
        p2.do_action(PlayerAction::Play(2, vec![]))?;
        p2.do_action(PlayerAction::ActivateExpendAbility(2, vec![]))?;
        p2.do_action(PlayerAction::Play(0, vec![]))?;
        p2.do_action(PlayerAction::Play(0, vec![]))?;
        p2.do_action(PlayerAction::Play(0, vec![]))?;
        p2.do_action(PlayerAction::ActivateAllyAbility(2, vec![]))?;
        p2.do_action(PlayerAction::ActivateAllyAbility(3, vec![]))?;
        p2.do_action(PlayerAction::ActivateAllyAbility(4, vec![]))?;
        p2.do_action(PlayerAction::PurchaseFromShop(0))?;
        p2.do_action(PlayerAction::AttackPlayer(b1.you, 12))?;
        p2.do_action(PlayerAction::EndTurn)?;

        for _ in 0..5 {
            p1.do_action(PlayerAction::Play(0, vec![]))?;
        }

        p1.do_action(PlayerAction::ActivateSacrificeAbility(2, vec![]))?;
        p1.do_action(PlayerAction::AttackPlayerChampion(b2.you, 0))?;
        p1.do_action(PlayerAction::PurchaseFromShop(3))?;
        p1.do_action(PlayerAction::EndTurn)?;

        for _ in 0..6 {
            p2.do_action(PlayerAction::Play(0, vec![]))?;
        }
        p2.do_action(PlayerAction::AttackPlayer(b1.you, 10))?;

        b2 += p2.poll_deltas();
        assert_eq!(true, b2.game_over);

        b1 += p1.poll_deltas();
        assert_eq!(true, b1.game_over);

        Ok(())
    }

    use crate::tui::{draw_as_string, Draw};
    use std::iter::repeat;

    fn lets_see_and_panic(board: &Board) {
        let (lines, cmds) = draw_board(board, 120, 0, 0);
        println!("{}", draw_as_string(lines + 1, 120, cmds));

        println!("Your discard: {:?}", board.mats[board.you].discard);
        println!(
            "Opponent's discard: {:?}",
            board.mats[(board.you + 1) % board.players].discard
        );

        panic!("IF YOU REACHED THIS PANIC, THEN YOU ARE DOING GREAT LOL!");
    }

    fn draw_board(
        board: &Board,
        w: usize,
        focused_column: usize,
        focused_row: usize,
    ) -> (usize, Vec<Draw>) {
        let wpc = w / (2 + board.players);
        let mut maxh = 0;
        let mut cmd = vec![];

        // subdivide space in 2 + #players
        // if each column is less than 28 then give
        // 28 width to the currently selected column
        // and fit the rest
        let (l, d) = draw_cards(&board.your_hand, wpc);
        cmd.push(Draw::Print(3, 1 + wpc / 2 - 5, "YOUR HAND".into()));
        cmd.push(Draw::WithOffset(5, 0, d));
        let l = l + 5;
        if l > maxh {
            maxh = l
        }

        let (l1, d) = draw_player_status(&board.mats[board.you], wpc);
        cmd.push(Draw::WithOffset(0, wpc, d));
        let (l2, d) = draw_player_field(&board.mats[board.you], wpc);
        cmd.push(Draw::WithOffset(l1, wpc, d));
        let l = l1 + l2;
        if l > maxh {
            maxh = l
        }

        let (l, d) = draw_shop(&board, wpc);
        cmd.push(Draw::WithOffset(0, wpc * 2, d));
        if l > maxh {
            maxh = l
        }

        for i in 1..board.players {
            let op = (board.you + i) % board.players;

            let col_offset = wpc * (2 + i);

            let (l1, d) = draw_player_status(&board.mats[op], wpc);
            cmd.push(Draw::WithOffset(0, col_offset, d));
            let (l2, d) = draw_player_field(&board.mats[op], wpc);
            cmd.push(Draw::WithOffset(l1, col_offset, d));
            let l = l1 + l2;
            if l > maxh {
                maxh = l
            }
        }

        (maxh, cmd)
    }

    fn draw_shop(board: &Board, w: usize) -> (usize, Vec<Draw>) {
        let mut cmd = vec![];
        let mut line = 1;
        cmd.push(Draw::Print(line, w / 2 - 2, "SHOP".into()));
        line += 2;

        let (l, d) = draw_cards(&board.shop, w);
        cmd.push(Draw::WithOffset(line, 0, d));
        line += l;
        let (l, d) = draw_card_body(w, &Card::FireGem, true);
        cmd.push(Draw::WithOffset(line, 0, d));
        line += l;

        (line, cmd)
    }

    fn draw_cards(cards: &Vec<Card>, w: usize) -> (usize, Vec<Draw>) {
        let mut cmd = draw_card_top(w);
        let mut lines = 1;

        if cards.len() == 0 {
            cmd.push(Draw::Print(lines, w / 2 - 4, "empty...".into()));
        } else {
            for card in cards.iter() {
                let (l, d) = draw_card_body(w, &card, true);
                cmd.push(Draw::WithOffset(lines, 0, d));
                lines += l;
            }
        }

        (lines, cmd)
    }

    fn draw_player_field(mat: &Mat, w: usize) -> (usize, Vec<Draw>) {
        let mut cmd = draw_card_top(w);
        let mut lines = 1;

        if mat.field.len() == 0 {
            cmd.push(Draw::Print(lines, 1 + w / 2 - 4, "empty...".into()));
        } else {
            for cif in mat.field.iter() {
                let (l, d) = draw_card_body(w, &cif.card, true);
                cmd.push(Draw::WithOffset(lines, 0, d));
                lines += l;

                let mut s = String::new();
                if !cif.expend_ability_used && cif.card.expend_ability().is_some() {
                    s.push('E');
                }
                if !cif.ally_ability_used
                    && cif.card.ally_ability().is_some()
                    && mat
                        .field
                        .iter()
                        .filter(|cif_| cif_.card.faction() == cif.card.faction())
                        .count()
                        >= 2
                {
                    s.push('A');
                }
                if cif.card.sacrifice_ability().is_some() {
                    s.push('S');
                }

                if s != "" {
                    cmd.push(Draw::Print(lines - 1, 0, format!("\\{}\\", s)));
                }
            }
        }

        (lines, cmd)
    }

    fn draw_player_status(mat: &Mat, w: usize) -> (usize, Vec<Draw>) {
        let mut cmd = vec![];
        let mut line = 1;

        cmd.push(Draw::Print(
            line,
            w / 2 - mat.name.len() / 2,
            mat.name.clone(),
        ));
        line += 2;
        cmd.push(Draw::Print(line, 1, format!("   LIVES: {}", mat.lives)));
        line += 1;
        cmd.push(Draw::Print(line, 1, format!("  COMBAT: {}", mat.combat)));
        line += 1;
        cmd.push(Draw::Print(line, 1, format!("    GOLD: {}", mat.gold)));

        if mat.must_discard > 0 {
            line += 1;
            cmd.push(Draw::Print(
                line,
                1,
                format!("DISCARDS: {}", mat.must_discard),
            ));
        }

        line += 2;
        cmd.push(Draw::Print(
            line,
            1,
            format!(
                "HAND {}, DECK {}, DISCARD {}",
                mat.hand,
                mat.deck,
                mat.discard.len()
            ),
        ));

        (line + 1, cmd)
    }

    fn draw_card_top(w: usize) -> Vec<Draw> {
        vec![Draw::Print(
            0,
            1,
            repeat('_').take(w - 2).collect::<String>(),
        )]
    }

    fn draw_card_body(w: usize, card: &Card, full: bool) -> (usize, Vec<Draw>) {
        let mut cmd = vec![];
        if card.cost() > 0 {
            cmd.push(Draw::Print(0, w - 5, format!("({})", card.cost())));
        }

        let mut offset = 0;
        match card.faction() {
            Faction::NoFaction => (),
            f => {
                offset += 1;
                cmd.push(Draw::Print(0, 2, format!("<{:?}>", f)));
            }
        }

        cmd.push(Draw::Print(offset, 2, format!("{:?}", card)));

        if full {
            cmd.push(Draw::Print(
                1 + offset,
                2,
                repeat(" - ").take((w - 4) / 3).collect::<String>(),
            ));
            offset += 1;

            if let Some(a) = card.primary_ability() {
                cmd.push(Draw::Print(offset + 1, 2, "P: ".into()));
                let (lines, draw) = draw_ability(w - 7, a);
                cmd.push(Draw::WithOffset(offset + 1, 5, draw));
                offset += lines;
            }

            if let Some(a) = card.expend_ability() {
                cmd.push(Draw::Print(offset + 1, 2, "E: ".into()));
                let (lines, draw) = draw_ability(w - 7, a);
                cmd.push(Draw::WithOffset(offset + 1, 5, draw));
                offset += lines;
            }

            if let Some(a) = card.ally_ability() {
                cmd.push(Draw::Print(offset + 1, 2, "A: ".into()));
                let (lines, draw) = draw_ability(w - 7, a);
                cmd.push(Draw::WithOffset(offset + 1, 5, draw));
                offset += lines;
            }

            if let Some(a) = card.sacrifice_ability() {
                cmd.push(Draw::Print(offset + 1, 2, "S: ".into()));
                let (lines, draw) = draw_ability(w - 7, a);
                cmd.push(Draw::WithOffset(offset + 1, 5, draw));
                offset += lines;
            }
        }

        cmd.push(Draw::Print(
            1 + offset,
            0,
            format!("'{}'", repeat('_').take(w - 2).collect::<String>()),
        ));

        if card.is_guard() {
            cmd.push(Draw::Print(
                1 + offset,
                w - 7,
                format!("[*{}*]", card.defense()),
            ));
        } else if card.is_champion() {
            cmd.push(Draw::Print(
                1 + offset,
                w - 7,
                format!("[ {} ]", card.defense()),
            ));
        }

        for i in 0..offset + 1 {
            cmd.push(Draw::Print(i, 0, "|".into()));
            cmd.push(Draw::Print(i, w - 1, "|".into()));
        }

        (2 + offset, cmd)
    }

    fn draw_ability(w: usize, ability: Vec<Effect>) -> (usize, Vec<Draw>) {
        let mut text = ability_text(ability);
        text.push('.');
        let rem = text.split_off(1);
        text = text.to_uppercase();
        text.push_str(&rem);
        let mut lines: Vec<String> = vec![];
        for word in text.split(' ') {
            if lines.len() == 0 || lines.last().unwrap().len() + word.len() >= w {
                lines.push(String::new());
            } else {
                lines.last_mut().unwrap().push(' ');
            }
            lines.last_mut().unwrap().push_str(word);
        }
        (
            lines.len(),
            lines
                .into_iter()
                .enumerate()
                .map(|(i, l)| Draw::Print(i, 0, l))
                .collect(),
        )
    }

    fn ability_text(ability: Vec<Effect>) -> String {
        ability
            .into_iter()
            .map(|e| match e {
                Effect::Gold(x) => format!("gain {} gold", x),
                Effect::Combat(x) => format!("add {} combat", x),
                Effect::Heal(x) => format!("heal {}", x),
                Effect::Draw(x) if x == 1 => format!("draw a card"),
                Effect::Draw(x) if x > 1 => format!("draw {} cards", x),
                Effect::Choice(a, b) if a.len() > 0 => {
                    format!("choose between {} or {}", ability_text(a), ability_text(b))
                }
                Effect::Choice(a, b) if a.len() == 0 => format!("may {}", ability_text(b)),
                Effect::CombatPer(x, per) => {
                    format!("add {} combat per {}", x, per_amount_text(per))
                }
                Effect::HealPer(x, per) => format!("heal {} per {}", x, per_amount_text(per)),
                Effect::Nothing => "nothing".into(),
                Effect::Sacrifice(x) => format!("sacrifice {} from your hand/discard", x),
                Effect::OpponentDiscards(x) if x == 1 => format!("target opponent discards a card"),
                Effect::OpponentDiscards(x) if x > 1 => {
                    format!("target opponent discards {} cards", x)
                }
                Effect::PlayerDiscards(x) if x == 1 => format!("discard a card"),
                Effect::PlayerDiscards(x) if x > 1 => format!("discard {} cards", x),
                Effect::StunChampion => "stun target champion".into(),
                Effect::NextActionPurchaseToTopOfDeck => {
                    "put the next action you acquire this turn on top of your deck".into()
                }
                Effect::NextPurchaseToTopOfDeck => {
                    "put the next card you acquire this turn on top of your deck".into()
                }
                Effect::PrepareChampion => "prepare a champion".into(),
                _ => panic!(format!("ability text missing for: {:#?}", e)),
                // Effect::PutOverDeckFromDiscard,
                // Effect::PutInHandFromDiscard,
                // Effect::NextPurchaseToHand,
            })
            .collect::<Vec<_>>()
            .join(" and ")
    }

    fn per_amount_text(per_amount: PerAmount) -> String {
        match per_amount {
            PerAmount::AdditionalFactionCard(f) => format!("other {:?} card", f),
            PerAmount::Champion => "champion".into(),
            PerAmount::AdditionalChampion => "additional chamption".into(),
            PerAmount::AdditionalGuardian => "additional guardian".into(),
        }
    }
}
