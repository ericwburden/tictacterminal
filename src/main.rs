pub mod display;
pub mod error;
pub mod game;

use crate::game::{Game, GameStatus};
use crate::error::GameError;
use crate::display::PrintToConsole;
use std::io::{stdout, Write};
use crossterm::{execute, Result};
use crossterm::cursor::{MoveDown, SavePosition, RestorePosition};
use crossterm::style::{Color, Print, SetForegroundColor, ResetColor};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType};
use text_io::try_read;

fn main() -> Result<()> {
    // Setup
    let mut stdout = stdout();
    let mut game = Game::new();
    let mut game_status = GameStatus::Pending;
    let mut err_msg = String::new();

    // Draw the game in an alternate screen
    execute!(stdout, EnterAlternateScreen)?;
    while game_status == GameStatus::Pending {
        execute!(stdout, Clear(ClearType::All))?;
        game.print_to_console(10, 10, Color::Grey)?;
        if err_msg.len() > 0 {
            execute!(
                stdout, 
                SavePosition, 
                MoveDown(2),
                SetForegroundColor(Color::DarkRed),
                Print(&err_msg), 
                RestorePosition,
                ResetColor,
            )?;
            err_msg = String::new();
        }
        print!("Please enter your chosen space: ");
        let _ = stdout.flush();
        let s: u8 =  match try_read!("{}\n").map_err(|_| GameError::InvalidSpace) {
            Ok(v) => v,
            Err(e) => { 
                err_msg = format!("{}, please try again!", e);
                continue;
            }
        };
        game_status = match game.play_on_space(s) {
            Ok(_) => game.status(),
            Err(e) => {
                err_msg = format!("{}, please try again!", e);
                GameStatus::Pending
            }
        }
    }
    execute!(stdout, LeaveAlternateScreen)?;
    execute!(stdout, Clear(ClearType::All))?;
    game.print_to_console(10, 10, Color::Grey)?;
    execute!(stdout, MoveDown(40))?;
    println!("");
    Ok(())
}
