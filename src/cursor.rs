use crate::display::{BIG_CURSOR, BIG_X, BIG_O, ROW_HEIGHT, COL_WIDTH, Draw, DrawWithColor};
use crate::game::{Coordinate, Game, Player};

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
    coordinate: Coordinate,
}

impl Cursor {
    pub(crate) fn new(player: Player, row: usize, col: usize) -> Self {
        let coordinate = Coordinate::new(row, col);
        Cursor { player, coordinate }
    }

    pub(crate) fn first_available(game: &Game) -> Option<Self> {
        let player = game.current_player();
        for space in game.iter() {
            if space.get_mark().is_none() { 
                let (row, col) = space.get_coordinate().into();
                return Some(Cursor::new(player, row, col));
            }
        }
        None
    }

    pub(crate) fn shift(&mut self, direction: Direction) {
        let (mut row, mut col) = self.coordinate.into(); 
        match direction {
            Direction::Up    => if row == 0 { row = 2 } else { row -= 1 },
            Direction::Left  => if col == 0 { col = 2 } else { col -= 1 },
            Direction::Down  => if row == 2 { row = 0 } else { row += 1 },
            Direction::Right => if col == 2 { col = 0 } else { col += 1 },
        };
        self.coordinate = Coordinate::new(row, col);
    }

    pub(crate) fn get_coordinate(&self) -> Coordinate {
        self.coordinate
    }
}

impl Draw for Cursor {
    fn draw(&self, term_row: u16, term_col: u16) -> Result<()> {
        let img = match self.player { Player::X => BIG_X, Player::O => BIG_O, };
        let (row, col) = self.coordinate.into();
        let out_row = term_row + (row as u16 * ROW_HEIGHT);
        let out_col = term_col + (col as u16 * COL_WIDTH);
        img.draw_with_color(out_row, out_col, Color::DarkYellow)?;
        BIG_CURSOR.draw_with_color(out_row, out_col, Color::DarkYellow)
    }
}
