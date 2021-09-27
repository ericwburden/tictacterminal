use crossterm::{execute, Result};
use crossterm::cursor::MoveTo;
use crossterm::style::{Color, Print, SetForegroundColor, ResetColor};
use std::ops::{Deref, DerefMut};


//--------------------------------------------------------------------------------------
//-- Constant row and column offsets
//--------------------------------------------------------------------------------------

pub(crate) const ROW_HEIGHT: u16 = 14;  // The height of a game board row, including grid space
pub(crate) const COL_WIDTH: u16 = 33;   // The width of a game board column, including grid space

//--------------------------------------------------------------------------------------
//-- Character constants for various display items
//--------------------------------------------------------------------------------------

pub(crate) const BIG_X: [&'static str; 12] = [
    "                              ",
    "      ●●●●●●●●●      ●●●      ",
    "       ●●●●●●●●●    ●●●       ",
    "        ●●●●●●●●●  ●●●        ",
    "         ●●●●●●●●●●●●         ",
    "          ●●●●●●●●●●          ",
    "          ●●●●●●●●●●          ",
    "         ●●●●●●●●●●●●         ",
    "        ●●●  ●●●●●●●●●        ",
    "       ●●●    ●●●●●●●●●       ",
    "      ●●●      ●●●●●●●●●      ",
    "                              "
];

pub(crate) const BIG_O: [&'static str; 12] = [
    "                              ",
    "          ●●●●●●●●●●          ",
    "       ●●●●●●      ●●●        ",
    "      ●●●●●●        ●●●●      ",
    "      ●●●●●          ●●●●     ",
    "      ●●●●●           ●●●     ",
    "      ●●●●●           ●●●     ",
    "      ●●●●●          ●●●●     ",
    "      ●●●●●●        ●●●●      ",
    "       ●●●●●●      ●●●        ",
    "          ●●●●●●●●●●          ",
    "                              ",
];

pub(crate) const BIG_CURSOR: [&'static str; 12] = [
    " ╔══════════════════════════╗ ",
    " ║                          ║ ",
    " ║                          ║ ",
    " ║                          ║ ",
    " ║                          ║ ",
    " ║                          ║ ",
    " ║                          ║ ",
    " ║                          ║ ",
    " ║                          ║ ",
    " ║                          ║ ",
    " ║                          ║ ",
    " ╚══════════════════════════╝ ",
];

pub(crate) const BIG_GRID: [&'static str; 40] = [
    "                              ┃ ┃                              ┃ ┃                               ",
    "                              ┃ ┃                              ┃ ┃                               ",
    "                              ┃ ┃                              ┃ ┃                               ",
    "                              ┃ ┃                              ┃ ┃                               ",
    "                              ┃ ┃                              ┃ ┃                               ",
    "                              ┃ ┃                              ┃ ┃                               ",
    "                              ┃ ┃                              ┃ ┃                               ",
    "                              ┃ ┃                              ┃ ┃                               ",
    "                              ┃ ┃                              ┃ ┃                               ",
    "                              ┃ ┃                              ┃ ┃                               ",
    "                              ┃ ┃                              ┃ ┃                               ",
    "                              ┃ ┃                              ┃ ┃                               ",
    "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━",
    "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━",
    "                              ┃ ┃                              ┃ ┃                               ",
    "                              ┃ ┃                              ┃ ┃                               ",
    "                              ┃ ┃                              ┃ ┃                               ",
    "                              ┃ ┃                              ┃ ┃                               ",
    "                              ┃ ┃                              ┃ ┃                               ",
    "                              ┃ ┃                              ┃ ┃                               ",
    "                              ┃ ┃                              ┃ ┃                               ",
    "                              ┃ ┃                              ┃ ┃                               ",
    "                              ┃ ┃                              ┃ ┃                               ",
    "                              ┃ ┃                              ┃ ┃                               ",
    "                              ┃ ┃                              ┃ ┃                               ",
    "                              ┃ ┃                              ┃ ┃                               ",
    "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛ ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━",
    "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓ ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━",
    "                              ┃ ┃                              ┃ ┃                               ",
    "                              ┃ ┃                              ┃ ┃                               ",
    "                              ┃ ┃                              ┃ ┃                               ",
    "                              ┃ ┃                              ┃ ┃                               ",
    "                              ┃ ┃                              ┃ ┃                               ",
    "                              ┃ ┃                              ┃ ┃                               ",
    "                              ┃ ┃                              ┃ ┃                               ",
    "                              ┃ ┃                              ┃ ┃                               ",
    "                              ┃ ┃                              ┃ ┃                               ",
    "                              ┃ ┃                              ┃ ┃                               ",
    "                              ┃ ┃                              ┃ ┃                               ",
    "                              ┃ ┃                              ┃ ┃                               ",
];

pub(crate) const BIG_WINS: [&'static str; 11] = [
    "                                          ",
    "                                          ",
    "  ●●●       ●●● ●●●                   ●●● ",
    "  ●●●   ●   ●●● ●●●                   ●●● ",
    "  ●●●  ●●●  ●●●                       ●●● ",
    "  ●●● ●●●●● ●●● ●●● ●●●●●●●  ●●●●●●●  ●●● ",
    "  ●●●●●●●●●●●●● ●●● ●●● ●●●● ●●●      ●●● ",
    "  ●●●●●● ●●●●●● ●●● ●●●  ●●● ●●●●●●●  ●●● ",
    "  ●●●●●   ●●●●● ●●● ●●●  ●●●  ●●●●●●●  ●  ",
    "  ●●●●     ●●●● ●●● ●●●  ●●●      ●●● ●●● ",
    "  ●●●       ●●● ●●● ●●●  ●●●  ●●●●●●● ●●● ",
];

pub(crate) const BIG_PLAYER: [&'static str; 14] = [
    "                                                         ",
    "                                                         ",
    "                                                         ",
    "  ●●●●●●●●●  ●●●                                         ",
    "  ●●●   ●●●● ●●●                                         ",
    "  ●●●    ●●● ●●●                                         ",
    "  ●●●   ●●●● ●●●  ●●●●●●  ●●●  ●●●  ●●●●●●  ●●●●●●● ●●●  ",
    "  ●●●●●●●●●  ●●●     ●●●● ●●●  ●●● ●●●  ●●● ●●●●●   ●●●  ",
    "  ●●●        ●●● ●●●●●●●● ●●●  ●●● ●●●●●●●● ●●●          ",
    "  ●●●        ●●● ●●●  ●●● ●●●● ●●● ●●●●     ●●●     ●●●  ",
    "  ●●●        ●●● ●●●●●●●●  ●●●●●●●  ●●●●●●  ●●●     ●●●  ",
    "                               ●●●                       ",
    "                          ●●● ●●●●                       ",
    "                           ●●●●●●                        ",
];

pub(crate) const BIG_TRY_AGAIN: [&'static str; 10] = [
    "  ●●●●●●●●●●●                           ●●●●●                   ●●●          ●●●", 
    "      ●●●                              ●●●●●●                   ●●●          ●●●", 
    "      ●●●                             ●●●●●●●                                ●●●", 
    "      ●●●  ●●●●●●● ●●●  ●●●          ●●●● ●●●  ●●●●●●   ●●●●●●  ●●● ●●●●●●●  ●●●", 
    "      ●●●  ●●●●●   ●●●  ●●●         ●●●●  ●●● ●●●●●●●●     ●●●● ●●● ●●● ●●●● ●●●", 
    "      ●●●  ●●●     ●●●  ●●●        ●●●●   ●●● ●●●  ●●● ●●●●●●●● ●●● ●●●  ●●● ●●●", 
    "      ●●●  ●●●     ●●●● ●●●       ●●●●●●●●●●● ●●●● ●●● ●●●  ●●● ●●● ●●●  ●●●  ● ", 
    "      ●●●  ●●●      ●●●●●●●      ●●●●     ●●●  ●●●●●●● ●●●●●●●● ●●● ●●●  ●●● ●●●", 
    "                   ●●●  ●●●                   ●●●  ●●●                          ", 
    "                    ●●●●●●                     ●●●●●●                           ", 
];


//--------------------------------------------------------------------------------------
//-- Structs and traits for displaying the game
//--------------------------------------------------------------------------------------

pub struct CharMatrix(Vec<Vec<char>>); // Just a Vec<Vec<char>> to attach traits to

impl Default for CharMatrix {
    fn default() -> Self {
        CharMatrix(vec![vec![]])
    }
}

impl Deref for CharMatrix {
    type Target = Vec<Vec<char>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CharMatrix {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for CharMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.iter() {
            writeln!(f, "{}", row.iter().collect::<String>())?;
        }
        Ok(())
    }
}

//--------------------------------------------------------------------------------------
//-- ToCharMatrix trait
//--------------------------------------------------------------------------------------

pub(crate) trait ToCharMatrix {
    fn to_char_matrix(&self) -> CharMatrix;
}

// const generics to the rescue!
impl<const N: usize> ToCharMatrix for [&'static str; N] {
    fn to_char_matrix(&self) -> CharMatrix {
        let mut out = Vec::new();
        for line in self {
            let out_line = line.chars().collect::<Vec<_>>();
            out.push(out_line);
        }
        CharMatrix(out)
    }
}

impl<T> Draw for T where T: ToCharMatrix {
    fn draw(&self, term_row: u16, term_col: u16) -> Result<()> {
        self.to_char_matrix().draw(term_row, term_col)
    }
}

//--------------------------------------------------------------------------------------
//-- Drawing traits
//--------------------------------------------------------------------------------------

// For printing game elements to the console
pub(crate) trait Draw {
    fn draw(&self, term_row: u16, term_col: u16) -> Result<()>;
}

impl Draw for CharMatrix {
    fn draw(&self, term_row: u16, term_col: u16) -> Result<()> {
        for (row_idx, row) in self.iter().enumerate() {
            let print_row = (row_idx as u16) + term_row;
            for (col_idx, col) in row.iter().enumerate() {
                if col.is_ascii_whitespace() { continue; }
                let print_col = (col_idx as u16) + term_col;
                execute!(std::io::stdout(), MoveTo(print_col, print_row), Print(col))?;
            }
        }
        Ok(())
    }
}


pub(crate) trait DrawWithColor {
    fn draw_with_color(&self, term_row: u16, term_col: u16, color: Color) -> Result<()>;
}

impl<T: Draw> DrawWithColor for T {
    fn draw_with_color(&self, term_row: u16, term_col: u16, color: Color) -> Result<()> {
        let mut stdout = std::io::stdout();
        execute!(stdout, SetForegroundColor(color))?;
        self.draw(term_row, term_col)?;
        execute!(stdout, ResetColor)
    }
}


