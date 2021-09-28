mod cursor;
mod display;
mod error;
mod game;

use crate::cursor::{Cursor, Direction};
use crate::display::Draw;
use crate::game::{Game, GameStatus};

use crossterm::{execute, Result};
use crossterm::cursor::MoveDown;
use crossterm::event::{read, Event, KeyCode};
use crossterm::style::Print;
use crossterm::terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType};

const TOP: u16 = 10;
const LEFT: u16 = 10;

fn main() -> Result<()> {
    // Setup
    let mut stdout = std::io::stdout();
    let mut game = Game::new();
    enable_raw_mode()?;

    // Draw the game in an alternate screen
    execute!(stdout, EnterAlternateScreen)?;
    'game: while let GameStatus::Pending(_) = game.status() {

        // While the game status is pending, there must be at least one available space
        let mut gc = Cursor::first_available(&game).unwrap();
        'control: loop {
            execute!(stdout, Clear(ClearType::All))?;
            gc.draw(TOP, LEFT)?;
            game.draw(TOP, LEFT)?;
            if let Event::Key(event) = read()? {
                match event.code {
                    KeyCode::Esc => break 'game,
                    KeyCode::Char('h') | KeyCode::Left  => gc.shift(Direction::Left),
                    KeyCode::Char('k') | KeyCode::Up    => gc.shift(Direction::Up),
                    KeyCode::Char('j') | KeyCode::Down  => gc.shift(Direction::Down),
                    KeyCode::Char('l') | KeyCode::Right => gc.shift(Direction::Right),
                    KeyCode::Enter => {
                        if let Err(e) = game.add_move(gc.get_coordinate()) {
                            println!("{}, please try again!", e);
                        }
                        continue 'game;
                    },
                    _ => continue 'control,
                }
            }
        }
    }

    // Cleanup
    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen)?;
    execute!(stdout, Clear(ClearType::All))?;
    game.draw(TOP, LEFT)?;
    execute!(stdout, MoveDown(40), Print("\n"))?;
    Ok(())
}
