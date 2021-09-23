pub mod display;
pub mod engine;
pub mod error;
pub mod game;

use crate::game::{Game, GameStatus};
use crate::error::GameError;
use crate::display::PrintToConsole;
use std::io::{stdout, Write};
use crossterm::{execute, Result};
use crossterm::cursor::MoveDown;
use crossterm::style::Color;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType};
use text_io::try_read;

fn main() -> Result<()> {
    // Setup
    let mut stdout = stdout();
    let mut game = Game::new();
    let mut game_status = GameStatus::Pending;

    // Draw the game in an alternate screen
    execute!(stdout, EnterAlternateScreen)?;
    while game_status == GameStatus::Pending {
        execute!(stdout, Clear(ClearType::All))?;
        game.print_to_console(10, 10, Color::Grey)?;
        print!("Please enter your chosen space: ");
        let _ = stdout.flush();
        let s: u8 =  match try_read!("{}\r\n").map_err(|_| GameError::InvalidSpace) {
            Ok(v) => v,
            Err(e) => { 
                print!("{}, please try again!", e);
                continue;
            }
        };
        game_status = match game.play_on_space(s) {
            Ok(_) => game.status(),
            Err(e) => {
                print!("{}, please try again!", e);
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
