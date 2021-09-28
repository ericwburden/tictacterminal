//! Structs that implement core game functionality, representing the game and
//! game board.

use crate::display::{
    BIG_X,
    BIG_O,
    BIG_GRID,
    BIG_PLAYER,
    BIG_TRY_AGAIN,
    BIG_WINS,
    ROW_HEIGHT,
    COL_WIDTH,
    Draw,
    DrawWithColor,
};
use crate::error::{GameError, Result};

use crossterm::execute;
use crossterm::cursor::MoveTo;
use crossterm::style::{Color, SetForegroundColor, ResetColor};
use itertools::Itertools; 
use std::collections::HashMap;


//--------------------------------------------------------------------------------------
//-- Game Status
//--------------------------------------------------------------------------------------

/// The three possible game states
/// - Winner: A player has won. Encapsulates the identity of the winner.
/// - Pending: The game has not yet concluded.
/// - Draw: The game has concluded in a draw. No more moves possible.
#[derive(Debug, PartialEq)]
pub(crate) enum GameStatus {
    Winner(Player),
    Pending(Player),
    Draw,
}

impl Draw for GameStatus {
    fn draw(&self, term_row: u16, term_col: u16) -> crossterm::Result<()> {
        match self {
            GameStatus::Winner(player) => {
                player.draw(term_row, term_col)?;
                BIG_WINS.draw_with_color(term_row, term_col + 30, Color::DarkGreen)
            },
            GameStatus::Pending(player) => {
                BIG_PLAYER.draw(term_row, term_col)?;
                player.draw(term_row, term_col + 55)
            },
            GameStatus::Draw => {
                BIG_TRY_AGAIN.draw_with_color(term_row, term_col, Color::DarkRed)
            },
        }
    }
}


//--------------------------------------------------------------------------------------
//-- Player
//--------------------------------------------------------------------------------------

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) enum Player { X, O }

impl Player {
    pub(crate) fn toggle(&mut self) {
        *self = match self {
            Player::X => Player::O,
            Player::O => Player::X
        };
    }
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O",)
        }
    }
}

impl Draw for Player {
    fn draw(&self, term_row: u16, term_col: u16) -> crossterm::Result<()> {
        match self { 
            Player::X => BIG_X.draw_with_color(term_row, term_col, Color::DarkCyan),
            Player::O => BIG_O.draw_with_color(term_row, term_col, Color::DarkMagenta),
        }
    }
}


//--------------------------------------------------------------------------------------
//-- Game Space Coordinate
//--------------------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub(crate) struct Coordinate {
    row: usize,
    col: usize,
}

impl Coordinate {
    pub(crate) fn new(row: usize, col: usize) -> Self {
        Coordinate { row, col }
    }

    pub(crate) fn get_tuple(&self) -> (usize, usize) {
        (self.row, self.col)
    }
}

impl From<Coordinate> for (usize, usize) {
    fn from(coord: Coordinate) -> Self {
        coord.get_tuple()
    }
}

impl From<(usize, usize)> for Coordinate {
    fn from(t: (usize, usize)) -> Self {
        Coordinate::new(t.0, t.1)
    }
}


//--------------------------------------------------------------------------------------
//-- Game Space
//--------------------------------------------------------------------------------------

/// Represents a single space on the game board
#[derive(Debug)]
pub(crate) struct GameSpace {
    mark: Option<Player>,     // Corresponds to which player marked that space
    value: u8,                // The 'magic square' value for this space
    coordinate: Coordinate,   // The coordinate in the game board containing this space
}

impl GameSpace {
    pub(crate) fn new(value: u8, row: usize, col: usize) -> Self { 
        let coordinate = Coordinate::new(row, col);
        GameSpace { mark: None, value, coordinate } 
    }

    pub(crate) fn get_mark(&self) -> Option<Player> {
        self.mark
    }

    pub(crate) fn get_coordinate(&self) -> Coordinate {
        self.coordinate
    }
}

impl Draw for GameSpace {
    fn draw(&self, term_row: u16, term_col: u16) -> crossterm::Result<()> {
        let (row, col) = self.coordinate.into();
        let out_row = term_row + (row as u16 * ROW_HEIGHT);
        let out_col = term_col + (col as u16 * COL_WIDTH);
        if let Some(player) = self.mark { return player.draw(out_row, out_col); }
        Ok(())
    }
}


//--------------------------------------------------------------------------------------
//-- Game
//--------------------------------------------------------------------------------------

/// Represents a Tic Tac Toe game
#[derive(Debug)]
pub(crate) struct Game {
    board: [[GameSpace; 3]; 3],  // The game board represented by game spaces
    player: Player               // The current player, Player::X or Player::O
}

impl<'a> Game {
    /// Create a new, empty game board
    pub(crate) fn new() -> Self { 
        let board = [
            [GameSpace::new(2, 0, 0), GameSpace::new(7, 0, 1), GameSpace::new(6, 0, 2)],
            [GameSpace::new(9, 1, 0), GameSpace::new(5, 1, 1), GameSpace::new(1, 1, 2)],
            [GameSpace::new(4, 2, 0), GameSpace::new(3, 2, 1), GameSpace::new(8, 2, 2)]
        ];
        Game { board, player: Player::X } 
    }

    /// Add a 'move' to the game board, marking a space according to the current player.
    /// Returns an error if the space indicated by 'move' is currently occupied.
    pub(crate) fn add_move(&mut self, coord: Coordinate) -> Result<()> {
        let (row, col) = coord.into();
        if let Some(_) = self.board[row][col].mark {
            Err(GameError::SpaceOccupied)
        } else {
            self.board[row][col].mark = Some(self.player);
            self.player.toggle();
            Ok(())
        }
    }

    /// Create a new Game from a series of 'moves'
    /// If any of the moves is invalid (space already occupied), return an error.
    /// Used primarily for testing
    #[allow(dead_code)]
    pub(crate) fn from(moves: &[Coordinate]) -> Result<Self> {
        let mut game = Self::new();
        for mv in moves { game.add_move(*mv)?; }
        Ok(game)
    }

    /// Return the current player
    pub(crate) fn current_player(&self) -> Player {
        self.player
    }

    /// Return an iterator that yields references to the individual game spaces, in
    /// order from left to right, top to bottom.
    pub(crate) fn iter(&'a self) -> GameIterator<'a> {
        GameIterator { game: &self, row: 0, col: 0 }
    }
    
    /// Return a reference to a game space given by its row/col index
    pub(crate) fn get_space(&self, coord: Coordinate) -> &GameSpace {
        let (row, col) = coord.into();
        &self.board[row][col]
    }

    /// Return a mapping of player to the values of the spaces occupied by that player.
    /// The values are derived from a 3x3 magic square where each horizontal, vertical,
    /// and diagonal line sums to 15.
    pub(crate) fn get_player_scores(&self) -> HashMap<Player, Vec<u8>> {
        let mut player_scores = HashMap::new();
        for space in self.iter() {
            let (row, col) = space.coordinate.into();
            if let Some(p) = self.board[row][col].mark {
                let scores = player_scores.entry(p).or_insert(Vec::with_capacity(5));
                scores.push(space.value);
            }
        }
        player_scores
    }

    /// Determines the winner of the game, as it stands, if there is one. Returns None
    /// if there is no winner. A winner is declared if any three of the values of the
    /// spaces occupied by that player sum to 15.
    pub(crate) fn get_winner(&self) -> Option<Player> {
        let player_scores = self.get_player_scores();
        for (player, scores) in player_scores.iter() {
            // Do any unique three-space combinations sum to 15?
            let player_wins = scores.iter()
                .combinations(3)
                .any(|scores| scores.iter().map(|i| *i).sum::<u8>() == 15);
            if player_wins { return Some(*player) }
        }
        None
    }

    /// Count the number of occupied spaces on the game board
    pub(crate) fn count_occupied_spaces(&self) -> u8 {
        let mut occupied_spaces = 0;
        for space in self.iter() {
            if space.mark.is_some() { occupied_spaces += 1; }
        }
        occupied_spaces
    }

    /// Determine and return the current status of the Game, as it currently stands
    pub(crate) fn status(&self) -> GameStatus {
        if let Some(player) = self.get_winner() { return GameStatus::Winner(player) }
        if self.count_occupied_spaces() == 9 { return GameStatus::Draw }
        GameStatus::Pending(self.player)
    }
}

impl Draw for Game {
    fn draw(&self, term_row: u16, term_col: u16) -> crossterm::Result<()> {
        let mut stdout = std::io::stdout();

        // Print the game grid (#)
        execute!(stdout, SetForegroundColor(Color::Grey))?;
        BIG_GRID.draw(term_row, term_col)?;

        // Print out the game spaces
        for space in self.iter() { space.draw(term_row, term_col)?; }

        // Print a status message to the right of the game grid
        self.status().draw(term_row, term_col + 104)?;

        // Reset the color and move the cursor underneath the message
        execute!(stdout, ResetColor, MoveTo(term_col + 104, term_row + 15))?;
        Ok(())
    }       
}


//--------------------------------------------------------------------------------------
//-- Iteration over game board positions, from left to right, top to bottom
//--------------------------------------------------------------------------------------

/// Iterator for a Game, used to iterate through game spaces
pub(crate) struct GameIterator<'a> {
    game: &'a Game,
    row: usize,
    col: usize,
}

impl<'a> Iterator for GameIterator<'a> {
    type Item = &'a GameSpace;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row > 2 { return None; }
        let coord: Coordinate = (self.row, self.col).into();
        let out = self.game.get_space(coord);
        if self.col == 2 { 
            self.row += 1;
            self.col = 0;
        } else {
            self.col += 1;
        }
        Some(out)
    }
}


//--------------------------------------------------------------------------------------
//-- Tests to ensure it works!
//--------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn to_coord_vec(arr: &[(usize, usize)]) -> Vec<Coordinate> {
        arr.iter().map(|x| Coordinate::from(*x)).collect()
    }

    #[test]
    fn test_one() {
        // Detects a 'downhill-diagonal' win
        let moves: Vec<_> = to_coord_vec(&[(0, 0), (2, 0), (1, 1), (2, 1), (2, 2)]);
        let game = Game::from(&moves).expect("Failed to create game.");
        assert_eq!(game.status(), GameStatus::Winner(Player::X));
    }

    #[test]
    fn test_two() {
        // Detects an 'uphill-diagonal' win
        let moves = to_coord_vec(&[(0, 0), (1, 1), (0, 1), (0, 2), (1, 0), (2, 0)]);
        let game = Game::from(&moves).expect("Failed to create game.");
        assert_eq!(game.status(), GameStatus::Winner(Player::O));
    }

    #[test]
    fn test_three() {
        // Returns 'Draw' when there is no winner and no moves remaining
        let moves = to_coord_vec(&[
            (0, 0), (1, 1), (2, 0),
            (1, 0), (1, 2), (2, 1),
            (0, 1), (0, 2), (2, 2)
        ]);
        let game = Game::from(&moves).expect("Failed to create game.");
        assert_eq!(game.status(), GameStatus::Draw);
    }

    #[test]
    fn test_four() {
        // Returns 'Pending' even if there are enough moves remaining to change 
        // the outcome later in the game
        let moves = to_coord_vec(&[(0, 0), (1, 1)]);
        let game = Game::from(&moves).expect("Failed to create game.");
        assert_eq!(game.status(), GameStatus::Pending(Player::X));
    }

    #[test]
    fn test_five() {
        // Returns 'Pending' even if there are NOT enough moves remaining to 
        // change the outcome later in the game
        let moves = to_coord_vec(&[(1, 1), (0, 0), (1, 2), (1, 0), (2, 0), (0, 2)]);
        let game = Game::from(&moves).expect("Failed to create game.");
        assert_eq!(game.status(), GameStatus::Pending(Player::X));
    }

    #[test]
    fn test_six() {
        // Detects a 'horizontal' win
        let moves = to_coord_vec(&[(1, 1), (0, 0), (1, 0), (0, 1), (1, 2)]);
        let game = Game::from(&moves).expect("Failed to create game.");
        assert_eq!(game.status(), GameStatus::Winner(Player::X));
    }

    #[test]
    fn test_seven() {
        // Detects a 'vertical' win
        let moves = to_coord_vec(&[(1, 1), (0, 2), (0, 0), (2, 2), (1, 0), (1, 2)]);
        let game = Game::from(&moves).expect("Failed to create game.");
        assert_eq!(game.status(), GameStatus::Winner(Player::O));
    }

    #[test]
    fn test_eight() {
        // Returns an error when attempting to add a duplicate move
        let moves = to_coord_vec(&[(2, 2), (2, 2)]);
        let game = Game::from(&moves).expect_err("Expected to receive an error");
        assert_eq!(game, GameError::SpaceOccupied)
    }
}

