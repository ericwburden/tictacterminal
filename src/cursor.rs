use crate::display::{BIG_CURSOR, BIG_X, BIG_O, ROW_HEIGHT, COL_WIDTH, Draw, DrawWithColor};
use crate::game::{Game, Player};

use crossterm::Result;
use crossterm::style::Color;

pub(crate) enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub(crate) struct Cursor {
    player: Player,
    pub(crate) row: usize,
    pub(crate) col: usize,
}

impl Cursor {
    pub fn new(player: Player, row: usize, col: usize) -> Self {
        Cursor { player, row, col }
    }

    pub fn first_available(game: &Game) -> Option<Self> {
        let player = game.current_player();
        for space in game.iter() {
            if space.mark.is_none() { 
                return Some(Cursor::new(player, space.row, space.col));
            }
        }
        None
    }

    pub fn shift(&mut self, direction: Direction) {
        match direction {
            Direction::Up    => if self.row == 0 { self.row = 2 } else { self.row -= 1 },
            Direction::Left  => if self.col == 0 { self.col = 2 } else { self.col -= 1 },
            Direction::Down  => if self.row == 2 { self.row = 0 } else { self.row += 1 },
            Direction::Right => if self.col == 2 { self.col = 0 } else { self.col += 1 },
        }
    }
}

impl Draw for Cursor {
    fn draw(&self, term_row: u16, term_col: u16) -> Result<()> {
        let img = match self.player { Player::X => BIG_X, Player::O => BIG_O, };
        let out_row = term_row + (self.row as u16 * ROW_HEIGHT);
        let out_col = term_col + (self.col as u16 * COL_WIDTH);
        img.draw_with_color(out_row, out_col, Color::DarkYellow)?;
        BIG_CURSOR.draw_with_color(out_row, out_col, Color::DarkYellow)
    }
}
