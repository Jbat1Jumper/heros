#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    Black,
    Grey,
    White,
    Red,
    Green,
    Blue,
    Yellow,
}

pub const CARD_EXAMPLE: &str = "
             ___________________________
            / (Necros)             (5) /
           / Varrick, the Necromancer /
          /       - - - - - -        /
         / E: Take a champion from  /
        /   your discard pile and  /
       /   put it on top of your  /
      /   deck.                  /
     / A: Draw a card.          /
    /____________________[ 3 ]_/
       ___________________________
      / Necros               (5) /
     / Varrick, the Necromancer /
    /____________________[ 3 ]_/__
      / Necros               (6) /
     / Life drain               /
    /__________________________/__
      / Guild                (5) /
     / Parov, the Enforcer      /
    /____________________[*5*]_/
       ___________________________
      / Wild                 (4) /
     / Nature's bounty          /
    /__.--------------------------
      / Necros               (5) /
     / Varrick, the Necromancer /
    /____________________[ 3 ]_/


   LIFE: 43        COMBAT: 3         GOLD: 6                                                  LIFE: 43        COMBAT: 3         GOLD: 6
  .------------------------ YOU ------------------------.                                    .--------------------- OPPONENT ----------------------.
             HAND                       FIELD                            SHOP                          FIELD                         HAND
  __________________________   __________________________     __________________________     __________________________   __________________________ 
 | <Necros>             (6) | | <Imperial>           (6) |   | <Wild>               (1) |   | <Wild>               (4) | | ???                      |
 | Life drain               | | Word of power            |   | Spark                    |   | Broelyn, Loreweaver      | '__________________________'
 '__________________________' '__________________________'   '__________________________'   '____________________[ 6 ]_' | ???                      |
 | <Necros>             (5) | | <Guild>              (5) |   | <Necros>             (3) |   | <Necros>             (5) | '__________________________'
 | Varrick, the Necromancer | | Rasmus, the Smuggler     |   | The rot                  |   | Varrick, the Necromancer | | ???                      |
 |  -  -  -  -  -  -  -  -  | '____________________[ 3 ]_'   '__________________________'   '____________________[ 3 ]_' '__________________________'
 | E: Take a champion from  | | Gold                     |   | <Necros>             (2) |                                | ???                      |
 |   your discard pile and  | '__________________________'   | Death cultist            |                                '__________________________'
 |   put it on top of your  | | Gold                     |   '____________________[*3*]_'                                | ???                      |
 |   deck.                  | '__________________________'   | <Imperial>           (7) |                                '__________________________'
 | A: Draw a card.          | | Gold                     |   | Domination               |                                                            
 '____________________[ 3 ]_' '__________________________'   '__________________________'                                                            
 | Fire Gem                 | | Fire Gem             (2) |   | <Imperial>           (4) |                                                            
 '__________________________' '__________________________'   | Darian, War Mage         |                                                            
 | <Guild>              (5) |                                '____________________[ 5 ]_'                                                            
 | Parov, the Enforcer      |                                | Fire Gem             (2) |                                                             
 '____________________[*5*]_'                                '__________________________'                                                            
 | <Wild>               (4) |                                                                                                                       
 | Nature's bounty          |                                                                                                                       
 '____________________[ 3 ]_'                                                                                                                       
  _________  _________   _________   _________  ________
 | <N> (6) || <I> (6) | | <G> (1) | | <W> (4) || ???    |
 | Life d..|| Word o..| | Rasmus..| | Broely..|'________'
 '_________''_________' '___[ 3 ]_' '_________'
";

#[derive(Clone, Debug, PartialEq)]
pub enum Draw {
    Clear,
    Print(usize, usize, String), // top, left, text
    //PrintHorizontalLine(XY, i32, char), // origin, width, content
    //PrintVerticalLine(XY, i32, char), // origin, height, content
    //PrintBox(XY, XY, char, char), // origin, size, inner content, outer content
    //WithFrontColor(Color, Vec<Draw>),
    //WithBackColor(Color, Vec<Draw>),
    WithOffset(usize, usize, Vec<Draw>), // top, left, more commands
    //WithClipping(XY, Vec<Draw>),
}

pub type Event = pc::Input;

pub trait Tui {
    fn on_event(&mut self, _event: Event) {}
    fn draw(&mut self, lines: usize, width: usize) -> Result<Vec<Draw>, ()>;
}

impl Tui for () {
    fn draw(&mut self, _w:usize, _h:usize) -> Result<Vec<Draw>, ()> {
        Ok(vec![])
    }
}

use pancurses as pc;

pub fn main<T>(mut app: T)
where
    T: Tui,
{
    let window = pc::initscr();
    pc::cbreak();
    window.refresh();
    window.keypad(true);
    let width = 130;
    let height = 40;

    // pc::resize_term(height, width);
    pc::noecho();
    loop {
        let w = window.get_max_x() as usize;
        let h = window.get_max_y() as usize;
        match app.draw(w, h) {
            Ok(draw) => draw_on_window(&window, draw, 0, 0),
            Err(_) => break,
        }
        match window.getch() {
            Some(pc::Input::KeyDC) => break,
            Some(i) => app.on_event(i),
            None => (),
        }
    }
    pc::endwin();
}

type Buffer = Vec<Vec<char>>;

pub fn draw_as_string(lines:usize, width:usize, commands: Vec<Draw>) -> String
{
    let mut buffer: Buffer =
        std::iter::repeat(std::iter::repeat(' ').take(width).collect())
            .take(lines)
            .collect();
    draw_over_buffer(&mut buffer, commands, 0, 0, lines, width);

    buffer
        .into_iter()
        .map(|row| row.into_iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

fn draw_over_buffer(buffer: &mut Buffer, commands: Vec<Draw>, top: usize, left: usize, bottom: usize, right: usize) {
    // ul = upper left, br = bottom right
    for dc in commands {
        match dc {
            Draw::Clear => {
                for i in top..bottom {
                    for j in left..right {
                        buffer[i as usize][j as usize] = ' ';
                    }
                }
            }
            Draw::Print(row, col, text) => {
                let text: Vec<char> = text.chars().collect();
                let row = row + top;
                let col = col + left;
                for i in 0..text.len() {
                    if col + i < right {
                        buffer[row][col + i] = text[i];
                    }
                }
            }
            Draw::WithOffset(t, l, cs) => {
                draw_over_buffer(buffer, cs, top + t, left + l, bottom, right);
            }
        }
    }
}

#[test]
fn test_draw_in_terminal() {
    let art = vec![
            Draw::WithOffset(1, 1, vec![Draw::Print(0, 0, "oh".into())]),
            Draw::Print(1, 4, "no!".into()),
        ];
    let result = draw_as_string(3, 8, art);
    assert_eq!(result, "        \n oh no! \n        ");
}

fn draw_on_window(window: &pc::Window, commands: Vec<Draw>, top: usize, left: usize) {
    for dc in commands {
        match dc {
            Draw::Clear => {
                window.erase();
            }
            Draw::Print(row, col, text) => {
                let row = top + row;
                let col = left + col;
                window.mvaddstr(col as i32, row as i32, text);
            }
            Draw::WithOffset(t, l, more_commands) => {
                draw_on_window(window, more_commands, top + t, left + l);
            }
        }
    }
}
