//! Structs that implement core game functionality, representing the game and
//! game board.

use crate::error::{GameError, Result};
use itertools::Itertools; 
use std::collections::HashMap;

/// The three possible game states
/// - Winner: A player has won. Encapsulates the identity of the winner.
/// - Pending: The game has not yet concluded.
/// - Draw: The game has concluded in a draw. No more moves possible.
#[derive(Debug, PartialEq)]
pub enum GameStatus {
    Winner(Player),
    Pending,
    Draw,
}


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Player {
    X,
    O,
}

impl Player {
    pub fn toggle(&mut self) {
        *self = match self {
            Player::X => Player::O,
            Player::O => Player::X
        };
    }

    pub fn to_char(&self) -> char {
        match self {
            Player::X => 'X',
            Player::O => 'O',
        }
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

/// Represents a single space on the game board
#[derive(Debug)]
pub struct GameSpace {
    pub mark: Option<Player>,     // Corresponds to which player marked that space
    pub value: u8,                // The 'magic square' value for this space
    pub row: usize,               // The row containing this space
    pub col: usize,               // The column containing this space
}

impl GameSpace {
    pub fn new(value: u8, row: usize, col: usize) -> Self { 
        GameSpace { mark: None, value, row, col } 
    }

    pub fn get_mark(&self) -> Option<Player> {
        self.mark
    }
}

/// Represents a Tic Tac Toe game
#[derive(Debug)]
pub struct Game {
    board: [[GameSpace; 3]; 3],  // The game board represented by game spaces
    player: Player               // The current player, Player::X or Player::O
}

impl<'a> Game {
    /// Create a new, empty game board
    pub fn new() -> Self { 
        let board = [
            [GameSpace::new(2, 0, 0), GameSpace::new(7, 0, 1), GameSpace::new(6, 0, 2)],
            [GameSpace::new(9, 1, 0), GameSpace::new(5, 1, 1), GameSpace::new(1, 1, 2)],
            [GameSpace::new(4, 2, 0), GameSpace::new(3, 2, 1), GameSpace::new(8, 2, 2)]
        ];
        Game { board, player: Player::X } 
    }

    /// Add a 'move' to the game board, marking a space according to the current player.
    /// Returns an error if the space indicated by 'move' is currently occupied.
    pub fn add_move(&mut self, mv: (usize, usize)) -> Result<()> {
        if let Some(_) = self.board[mv.0][mv.1].mark {
            Err(GameError::SpaceOccupied)
        } else {
            self.board[mv.0][mv.1].mark = Some(self.player);
            self.player.toggle();
            Ok(())
        }
    }

    /// Create a new Game from a series of 'moves'
    /// If any of the moves is invalid (space already occupied), return an error.
    pub fn from(moves: &[(usize, usize)]) -> Result<Self> {
        let mut game = Self::new();
        for mv in moves { game.add_move(*mv)?; }
        Ok(game)
    }

    /// Add a move by numeric space. Spaces are numbered 1-9 from left to right,
    /// top to bottom.
    pub fn play_on_space(&mut self, space: u8) -> Result<()> {
        match space {
            0 => self.add_move((0, 0))?,
            1 => self.add_move((0, 1))?,
            2 => self.add_move((0, 2))?,
            3 => self.add_move((1, 0))?,
            4 => self.add_move((1, 1))?,
            5 => self.add_move((1, 2))?,
            6 => self.add_move((2, 0))?,
            7 => self.add_move((2, 1))?,
            8 => self.add_move((2, 2))?,
            _ => return Err(GameError::InvalidSpace)
        }
        Ok(())
    }

    /// Return the current player
    pub fn current_player(&self) -> Player {
        self.player
    }

    /// Return an iterator that yields references to the individual game spaces, in
    /// order from left to right, top to bottom.
    pub fn iter(&'a self) -> GameIterator<'a> {
        GameIterator { game: &self, row: 0, col: 0 }
    }
    
    /// Return a reference to a game space given by its row/col index
    pub fn get_space(&self, row: usize, col: usize) -> &GameSpace {
       &self.board[row][col]
    }

    /// Return a mapping of player to the values of the spaces occupied by that player.
    /// The values are derived from a 3x3 magic square where each horizontal, vertical,
    /// and diagonal line sums to 15.
    pub fn get_player_scores(&self) -> HashMap<Player, Vec<u8>> {
        let mut player_scores = HashMap::new();
        for space in self.iter() {
            let (row_idx, col_idx) = (space.row, space.col);
            if let Some(p) = self.board[row_idx][col_idx].mark {
                let scores = player_scores.entry(p).or_insert(Vec::with_capacity(5));
                scores.push(space.value);
            }
        }
        player_scores
    }

    /// Determines the winner of the game, as it stands, if there is one. Returns None
    /// if there is no winner. A winner is declared if any three of the values of the
    /// spaces occupied by that player sum to 15.
    pub fn get_winner(&self) -> Option<Player> {
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
    pub fn count_occupied_spaces(&self) -> u8 {
        let mut occupied_spaces = 0;
        for space in self.iter() {
            if space.mark.is_some() { occupied_spaces += 1; }
        }
        occupied_spaces
    }

    /// Determine and return the current status of the Game, as it currently stands
    pub fn status(&self) -> GameStatus {
        if let Some(player) = self.get_winner() { return GameStatus::Winner(player) }
        if self.count_occupied_spaces() == 9 { return GameStatus::Draw }
        GameStatus::Pending
    }

    pub fn is_ongoing(&self) -> bool {
        self.status() == GameStatus::Pending
    }

    pub fn winner(&self) -> Option<Player> {
        if let GameStatus::Winner(winner) = self.status() {
            Some(winner)
        } else {
            None
        }
    }
}

// Implement Display for the Game, for pretty printing the current state of the board
impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in 0..3usize {
            for col in 0..3usize {
                let space_number = row*3 + col;
                let space_number_char = match space_number {
                    0 => '0', 1 => '1', 2 => '2', 3 => '3',
                    4 => '4', 5 => '5', 6 => '6', 7 => '7',
                    8 => '8', _ => ' ',
                };
                let value = match self.board[row][col].mark {
                    Some(v) => v.to_char(), 
                    None => space_number_char, 
                };
                write!(f, " {} ", value)?;
                if col < 2 { write!(f, "│")?; } else { write!(f, "\n")?; }
            }
            if row < 2 { writeln!(f, "───┼───┼───")?; }
        }
        writeln!(f, "")
    }
}

//--------------------------------------------------------------------------------------
//-- Iteration over game board positions, from left to right, top to bottom
//--------------------------------------------------------------------------------------

/// Iterator for a Game, used to iterate through game spaces
pub struct GameIterator<'a> {
    game: &'a Game,
    row: usize,
    col: usize,
}

impl<'a> Iterator for GameIterator<'a> {
    type Item = &'a GameSpace;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row > 2 { return None; }
        let out = self.game.get_space(self.row, self.col);
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

    #[test]
    fn test_one() {
        // Detects a 'downhill-diagonal' win
        let moves = [(0, 0), (2, 0), (1, 1), (2, 1), (2, 2)];
        let game = Game::from(&moves).expect("Failed to create game.");
        assert_eq!(game.status(), GameStatus::Winner(Player::X));
    }

    #[test]
    fn test_two() {
        // Detects an 'uphill-diagonal' win
        let moves = [(0, 0), (1, 1), (0, 1), (0, 2), (1, 0), (2, 0)];
        let game = Game::from(&moves).expect("Failed to create game.");
        assert_eq!(game.status(), GameStatus::Winner(Player::O));
    }

    #[test]
    fn test_three() {
        // Returns 'Draw' when there is no winner and no moves remaining
        let moves = [
            (0, 0), (1, 1), (2, 0),
            (1, 0), (1, 2), (2, 1),
            (0, 1), (0, 2), (2, 2)
        ];
        let game = Game::from(&moves).expect("Failed to create game.");
        assert_eq!(game.status(), GameStatus::Draw);
    }

    #[test]
    fn test_four() {
        // Returns 'Pending' even if there are enough moves remaining to change 
        // the outcome later in the game
        let moves = [(0, 0), (1, 1)];
        let game = Game::from(&moves).expect("Failed to create game.");
        assert_eq!(game.status(), GameStatus::Pending);
    }

    #[test]
    fn test_five() {
        // Returns 'Pending' even if there are NOT enough moves remaining to 
        // change the outcome later in the game
        let moves = [(1, 1), (0, 0), (1, 2), (1, 0), (2, 0), (0, 2)];
        let game = Game::from(&moves).expect("Failed to create game.");
        assert_eq!(game.status(), GameStatus::Pending);
    }

    #[test]
    fn test_six() {
        // Detects a 'horizontal' win
        let moves = [(1, 1), (0, 0), (1, 0), (0, 1), (1, 2)];
        let game = Game::from(&moves).expect("Failed to create game.");
        assert_eq!(game.status(), GameStatus::Winner(Player::X));
    }

    #[test]
    fn test_seven() {
        // Detects a 'vertical' win
        let moves = [(1, 1), (0, 2), (0, 0), (2, 2), (1, 0), (1, 2)];
        let game = Game::from(&moves).expect("Failed to create game.");
        assert_eq!(game.status(), GameStatus::Winner(Player::O));
    }

    #[test]
    fn test_eight() {
        // Returns an error when attempting to add a duplicate move
        let moves = [(2, 2), (2, 2)];
        let game = Game::from(&moves).expect_err("Expected to receive an error");
        assert_eq!(game, GameError::SpaceOccupied)
    }
}

