extern crate pancurses;
extern crate serde;

mod model;
mod smallrng;
mod tui;

mod player_api_tui {
    use crate::model::api::{Api, Board};
    use crate::tui::*;



    pub struct PlayerViewTui<A: Api>
    {
        api: A,
        board: Board,
        current_column: usize,
        position_in_column: Vec<usize>,
        //command: String,
    }

    impl<A: Api> PlayerViewTui<A>
    {
        pub fn new(api: A) -> Self {
            let board = api.get_state();
            let columns = 2 + board.players;
            PlayerViewTui {
                api,
                board,
                current_column: 0,
                //command: "".into(),
                position_in_column: std::iter::repeat(0).take(columns).collect(),
            }
        }

        fn column_len(&self, column: usize) -> usize {
            1
        }
    }

    impl<A: Api> Tui for PlayerViewTui<A> {
        fn on_event(&mut self, event: Event) {
            match event {
                Event::Character(c) => match c {
                    'k' => {
                        if self.position_in_column[self.current_column] > 0 {
                            self.position_in_column[self.current_column] -= 1;
                        }
                    }
                    'j' => {
                        if self.position_in_column[self.current_column]
                            < self.column_len(self.current_column) - 1
                        {
                            self.position_in_column[self.current_column] += 1;
                        }
                    }
                    'h' => {
                        if self.current_column > 0 {
                            self.current_column -= 1;
                        }
                    }
                    'l' => {
                        if self.current_column < self.position_in_column.len() {
                            self.current_column -= 1;
                        }
                    }
                    _ => println!("Unknown key {}", c),
                },
                _ => (),
            }
        }
        fn draw(&mut self, size: XY) -> Result<Vec<Draw>, ()> {
            // subdivide space in 2 + #players
            // if each column is less than 28 then give
            // 28 width to the currently selected column
            // and fit the rest

            Ok(vec![])
        }
    }
}

use model::local::Local1vs1;
use player_api_tui::PlayerViewTui;

fn main() {
    let game = Local1vs1::new(239);
    let pvtui = PlayerViewTui::new(game);
    tui::main(pvtui);
    println!("{}", tui::CARD_EXAMPLE);
}
