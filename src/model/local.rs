use super::{api::*, cards::*, master::*};
use crate::smallrng::*;

pub struct Local1vs1 {
    board: MasterBoard,
    player: usize,
}

impl Local1vs1 {
    pub fn new(seed: u64) -> Self {
        let rng = SRng::new(seed);
        Local1vs1 {
            player: rng.clone().gen::<usize>() % 2,
            board: MasterBoard::new(2, &Setup::base(), rng),
        }
    }

    fn can_see(&self, loc: Location) -> bool {
        match loc {
            Location::Deck(_) => false,
            Location::Discard(_) => true,
            Location::Field(_) => true,
            Location::FireGems => true,
            Location::Hand(player) => player == self.player,
            Location::Sacrifice => true,
            Location::Shop => true,
            Location::ShopDeck => false,
        }
    }
}

impl Api for Local1vs1 {
    type Error = &'static str;
    fn get_state(&self) -> Board {
        self.board.scoped_to(self.player)
    }
    fn do_action(&mut self, action: PlayerAction) -> Result<Vec<BoardDelta>, Self::Error> {
        if self.board.current_player != self.player {
            Err("Not the current player")
        } else {
            let mut deltas = self.board.do_action(action.clone())?;
            deltas.insert(0, BoardDelta::PlayerDeclaredAction(action.clone()));

            if let PlayerAction::EndTurn = action {
                panic!("Here opponent should play");
            }

            Ok(deltas
                .into_iter()
                .map(|delta| match delta {
                    BoardDelta::Move(from, index, to, card) => BoardDelta::Move(
                        from.clone(),
                        index,
                        to.clone(),
                        if self.can_see(from) || self.can_see(to) {
                            card
                        } else {
                            None
                        },
                    ),
                    d => d,
                })
                .collect())
        }
    }
}
