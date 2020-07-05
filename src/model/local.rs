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
        test_game_21022778(p1, p2)
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

    fn test_game_21022778<A: Api<Error = &'static str>>(
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

        Ok(())
    }
}
