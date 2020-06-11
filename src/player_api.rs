use crate::model::*;
use crate::smallrng::SRng;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
struct MatchView {
    player: usize,
    state: State,
}

impl MatchView {
    fn new(state: State, player: usize) -> MatchView {
        MatchView { state, player }
    }
}

trait PlayerApi {
    fn get_state(&self) -> State;
    fn do_action(&mut self, action: PlayerAction) -> Result<(), &'static str>;
}

impl PlayerApi for MatchView {
    fn get_state(&self) -> State {
        state_scoped_to(&self.state, self.player)
    }
    fn do_action(&mut self, action: PlayerAction) -> Result<(), &'static str> {
        if self.state.current_player != self.player {
            Err("Not the current player")
        } else {
            self.state.do_action(action)
        }
    }
}

fn state_scoped_to(state: &State, player: usize) -> State {
    State {
        shop_deck: state.shop_deck.iter().map(|_| Card::Unknown).collect(),
        mats: state
            .mats
            .iter()
            .enumerate()
            .map(|(i, mat)| {
                if i == state.current_player {
                    player_mat_owner_view(mat)
                } else {
                    player_mat_other_view(mat)
                }
            })
            .collect(),
        rng: SRng::new(0),
        ..state.clone()
    }
}

fn player_mat_owner_view(mat: &PlayerMat) -> PlayerMat {
    PlayerMat {
        deck: mat.deck.iter().map(|_| Card::Unknown).collect(),
        ..mat.clone()
    }
}

fn player_mat_other_view(mat: &PlayerMat) -> PlayerMat {
    PlayerMat {
        hand: mat.hand.iter().map(|_| Card::Unknown).collect(),
        deck: mat.deck.iter().map(|_| Card::Unknown).collect(),
        ..mat.clone()
    }
}
