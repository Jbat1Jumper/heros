pub struct XY {
    x: i32,
    y: i32,
}

impl XY {
    pub fn add(&self, other: &XY) -> XY {
        XY {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
    pub fn zero() -> XY {
        XY { x: 0, y: 0 }
    }
}

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

pub enum Draw {
    Clear,
    Print(XY, String), // origin, text
    //PrintHorizontalLine(XY, i32, char), // origin, width, content
    //PrintVerticalLine(XY, i32, char), // origin, height, content
    //PrintBox(XY, XY, char, char), // origin, size, inner content, outer content
    //WithFrontColor(Color, Vec<Draw>),
    //WithBackColor(Color, Vec<Draw>),
    WithOffset(XY, Vec<Draw>),
    //WithClipping(XY, Vec<Draw>),
}

pub type Event = pc::Input;

pub trait Tui {
    fn on_event(&mut self, _event: Event) {}
    fn draw(&mut self, size: XY) -> Result<Vec<Draw>, ()>;
}

impl Tui for () {
    fn draw(&mut self, _size: XY) -> Result<Vec<Draw>, ()> {
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

    pc::resize_term(height, width);
    pc::noecho();
    loop {
        let size = XY {
            x: window.get_max_y(),
            y: window.get_max_x(),
        };
        match app.draw(size) {
            Ok(draw) => draw_on_window(&window, draw, XY::zero()),
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

fn draw_on_window(window: &pc::Window, commands: Vec<Draw>, offset: XY) {
    for dc in commands {
        match dc {
            Draw::Clear => {
                window.erase();
            }
            Draw::Print(origin, text) => {
                let pos = offset.add(&origin);
                window.mvaddstr(pos.x, pos.y, text);
            }
            Draw::WithOffset(more_offset, more_commands) => {
                draw_on_window(window, more_commands, offset.add(&more_offset));
            }
        }
    }
}
