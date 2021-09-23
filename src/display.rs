use crate::game::{Game, GameStatus, Player};
use crossterm::{execute, Result};
use crossterm::cursor::MoveTo;
use crossterm::style::{Color, Print, SetForegroundColor, ResetColor};
use std::ops::{Deref, DerefMut};

//--------------------------------------------------------------------------------------
//-- Character constants for various display items
//--------------------------------------------------------------------------------------

const BIG_X: [&'static str; 10] = [
    "      ●●●●●●●●●      ●●●      ",
    "       ●●●●●●●●●    ●●●       ",
    "        ●●●●●●●●●  ●●●        ",
    "         ●●●●●●●●●●●●         ",
    "          ●●●●●●●●●●          ",
    "          ●●●●●●●●●●          ",
    "         ●●●●●●●●●●●●         ",
    "        ●●●  ●●●●●●●●●        ",
    "       ●●●    ●●●●●●●●●       ",
    "      ●●●      ●●●●●●●●●      "
];

const BIG_O: [&'static str; 10] = [
    "          ●●●●●●●●●●          ",
    "       ●●●●●●      ●●●        ",
    "      ●●●●●●        ●●●●      ",
    "      ●●●●●          ●●●●     ",
    "      ●●●●●           ●●●     ",
    "      ●●●●●           ●●●     ",
    "      ●●●●●          ●●●●     ",
    "      ●●●●●●        ●●●●      ",
    "       ●●●●●●      ●●●        ",
    "          ●●●●●●●●●●          "
];

const BIG_EMPTY: [&'static str; 10] = [
    "                              ",
    "                              ",
    "                              ",
    "                              ",
    "                              ",
    "                              ",
    "                              ",
    "                              ",
    "                              ",
    "                              "
];

const BIG_GRID: [&'static str; 38] = [
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
];

const BIG_WINS: [&'static str; 10] = [
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

const BIG_PLAYER: [&'static str; 10] = [
    "  ●●●●●●●●●  ●●●                                         ",
    "  ●●●   ●●●● ●●●                                         ",
    "  ●●●    ●●● ●●●                                         ",
    "  ●●●   ●●●● ●●●  ●●●●●●  ●●●  ●●●  ●●●●●●  ●●●●●●● ●●●  ",
    "  ●●●●●●●●●  ●●●     ●●●● ●●●  ●●● ●●●  ●●● ●●●●●   ●●●  ",
    "  ●●●        ●●● ●●●●●●●● ●●●  ●●● ●●●●●●●● ●●●          ",
    "  ●●●        ●●● ●●●  ●●● ●●●● ●●● ●●●●     ●●●     ●●●  ",
    "  ●●●        ●●● ●●●●●●●●  ●●●●●●●  ●●●●●●  ●●●     ●●●  ",
    "                          ●●●  ●●●                       ",
    "                           ●●●●●●                        ",
];

const BIG_TRY_AGAIN: [&'static str; 10] = [
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

impl CharMatrix {
    /// Given a player, get the CharMatrix representing that player (X or O)
    fn get_player_mark(player: Player) -> CharMatrix {
        let player_repr = match player {
            Player::X => BIG_X,
            Player::O => BIG_O,
        };
        player_repr.to_char_matrix()
    }
}

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

// For printing game elements to the console
pub(crate) trait PrintToConsole {
    fn print_to_console(&self, term_row: u16, term_col: u16, color: Color) -> Result<()>;
}

impl PrintToConsole for CharMatrix {
    fn print_to_console(&self, term_row: u16, term_col: u16, color: Color) -> Result<()> {
        let mut stdout = std::io::stdout();
        execute!(stdout, SetForegroundColor(color))?;

        for (row_idx, row) in self.iter().enumerate() {
            let row_offset = row_idx as u16;
            let row_string = row.iter().collect::<String>();
            execute!(
                stdout, 
                MoveTo(term_col, term_row + row_offset),
                Print(row_string)
            )?;
        }

        execute!(stdout, ResetColor)?;
        Ok(())
    }
}

impl PrintToConsole for Game {
    fn print_to_console(&self, term_row: u16, term_col: u16, color: Color) -> Result<()> {
        let mut stdout = std::io::stdout();
        let grid = BIG_GRID.to_char_matrix();

        // Print the game grid (#)
        execute!(stdout, SetForegroundColor(color))?;
        grid.print_to_console(term_row, term_col, color)?;

        // Print out the game spaces
        for (space_idx, space) in self.iter().enumerate() {
            let cursor_row = term_row + (space.row * 14) as u16;
            let cursor_col = term_col + (space.col * 33) as u16;
            let (space_color, space_cell_repr) = match space.mark {
                Some(Player::X) => (Color::DarkCyan, BIG_X),
                Some(Player::O) => (Color::DarkMagenta, BIG_O),
                None => (color, BIG_EMPTY),
            };
            let space_char_matrix = space_cell_repr.to_indexed_char_matrix(space_idx as u32);
            space_char_matrix.print_to_console(cursor_row, cursor_col, space_color)?;
        }

        // Print a status message to the right of the game grid
        let msg_row = term_row;
        let msg_col = term_col + 104;
        match self.status() {
            GameStatus::Pending => {
                // Print out a current player message
                let current_player_mark = CharMatrix::get_player_mark(self.current_player());
                BIG_PLAYER.print_to_console(msg_row, msg_col, color)?;
                current_player_mark.print_to_console(msg_row, msg_col + 55, color)?;
            },
            GameStatus::Draw => {
                // Print a 'Try Again' message for a Draw
                BIG_TRY_AGAIN .print_to_console(msg_row, msg_col, Color::DarkRed)?;
            },
            GameStatus::Winner(winner) => {
                // Print a winner message if a player wins
                let winner_mark = CharMatrix::get_player_mark(winner);
                winner_mark.print_to_console(msg_row, msg_col, Color::DarkGreen)?;
                BIG_WINS.print_to_console(msg_row, msg_col + 35, Color::DarkGreen)?;
            }
        }

        // Reset the color and move the cursor underneath the message
        execute!(stdout, ResetColor, MoveTo(msg_col, term_row + 15))?;
        Ok(())
    }       
}


// Type aliases for the static display constants
type CellRepr = [&'static str; 10];
type GridRepr = [&'static str; 38];

pub(crate) trait ToCharMatrix {
    fn to_char_matrix(&self) -> CharMatrix;
}

// There's probably a good way not to need to re-write the ToCharMatrix implementation
// for both kinds of Repr
impl ToCharMatrix for CellRepr {
    fn to_char_matrix(&self) -> CharMatrix {
        let mut out = Vec::with_capacity(10);
        for line in self {
            let out_line = line.chars().collect::<Vec<_>>();
            out.push(out_line);
        }
        CharMatrix(out)
    }
}

impl ToCharMatrix for GridRepr {
    fn to_char_matrix(&self) -> CharMatrix {
        let mut out = Vec::with_capacity(38);
        for line in self {
            let out_line = line.chars().collect::<Vec<_>>();
            out.push(out_line);
        }
        CharMatrix(out)
    }
}

impl<T> PrintToConsole for T where T: ToCharMatrix {
    fn print_to_console(&self, term_row: u16, term_col: u16, color: Color) -> Result<()> {
        self.to_char_matrix().print_to_console(term_row, term_col, color)
    }
}

trait ToIndexedCharMatrix {
    fn to_indexed_char_matrix(&self, index: u32) -> CharMatrix;
}

impl ToIndexedCharMatrix for CellRepr {
    fn to_indexed_char_matrix(&self, index: u32) -> CharMatrix {
        let mut char_matrix = self.to_char_matrix();
        char_matrix[0][2] = char::from_digit(index, 10).unwrap();
        char_matrix
    }
}

