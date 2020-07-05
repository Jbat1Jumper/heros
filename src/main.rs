extern crate pancurses;
extern crate serde;

mod model;
mod smallrng;
mod tui;

mod player_api_tui {
    use crate::model::api::{Api, Board};
    use crate::tui::*;

    pub struct PlayerViewTui<A: Api> {
        api: A,
        current_column: usize,
        position_in_column: Vec<usize>,
        //command: String,
    }

    impl<A: Api> PlayerViewTui<A> {
        pub fn new(api: A) -> Self {
            let columns = 2 + api.get_board().players;
            PlayerViewTui {
                api,
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

use model::api::{Api, PlayerAction};
use model::local::LocalServer;
use player_api_tui::PlayerViewTui;
use std::thread;
use std::time::Duration;

fn main() {
    let (mut server, mut clients) = LocalServer::new(239, 2);
    let mut player = clients.remove(0);
    let mut bot = clients.remove(0);

    thread::spawn(move || loop {
        if server.process_action().is_err() {
            break;
        }
    });

    thread::spawn(move || {
        let you = bot.get_board().you;
        loop {
            // here be dragons

            thread::sleep(Duration::from_millis(200));
            bot.poll_deltas();
            if bot.get_board().current_player == you {
                for _ in 0..bot.get_board().mats[you].must_discard {
                    bot.do_action(PlayerAction::Discard(0)).expect("Oh no");
                }
                bot.do_action(PlayerAction::EndTurn).expect("Oh no");
            }
        }
    });

    let pvtui = PlayerViewTui::new(player);
    tui::main(pvtui);

    println!("{}", tui::CARD_EXAMPLE);
}
