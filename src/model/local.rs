use super::{api::*, cards::*, master::*};
use crate::smallrng::*;

pub struct Local1vs1 {
    board: MasterBoard,
    player: usize,
}

impl Local1vs1 {
    pub fn new(seed: u64) -> Self {
        let mut rng = SRng::new(seed);
        Local1vs1 {
            player: rng.gen::<usize>() % 2,
            board: MasterBoard::new(2, &Setup::base(), rng.fork()),
        }
    }
}

impl Api for Local1vs1 {
    type Error = &'static str;
    fn get_state(&self) -> Board {
        panic!("No board yet")
    }
    fn do_action(&mut self, action: PlayerAction) -> Result<Vec<BoardDelta>, Self::Error> {
        if self.board.current_player != self.player {
            Err("Not the current player")
        } else {
            self.board.do_action(action.clone());

            if let PlayerAction::EndTurn = action {
                panic!("Here opponent should play");
            }
            panic!("No do actions yet");
        }
    }
}

// fn state_scoped_to(state: &Board, player: usize) -> Board {
//     Board {
//         shop_deck: state.shop_deck.iter().map(|_| Card::Unknown).collect(),
//         mats: state
//             .mats
//             .iter()
//             .enumerate()
//             .map(|(i, mat)| {
//                 if i == state.current_player {
//                     player_mat_owner_view(mat)
//                 } else {
//                     player_mat_other_view(mat)
//                 }
//             })
//             .collect(),
//         rng: SRng::new(0),
//         ..state.clone()
//     }
// }
//
// fn player_mat_owner_view(mat: &PlayerMat) -> PlayerMat {
//     PlayerMat {
//         deck: mat.deck.iter().map(|_| Card::Unknown).collect(),
//         ..mat.clone()
//     }
// }
//
// fn player_mat_other_view(mat: &PlayerMat) -> PlayerMat {
//     PlayerMat {
//         hand: mat.hand.iter().map(|_| Card::Unknown).collect(),
//         deck: mat.deck.iter().map(|_| Card::Unknown).collect(),
//         ..mat.clone()
//     }
// }
//