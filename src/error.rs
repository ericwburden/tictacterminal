//! Custom error types to represent the sorts of errors that may occur during a game
//! `SpaceOccupied` - Tried to place a mark in a space already marked

/// A list specifying the categories of Game errors
#[derive(Debug, PartialEq)]
#[non_exhaustive]
pub(crate) enum GameError {
    /// Tried to place a mark in a space already marked
    SpaceOccupied,
}

pub(crate) type Result<T> = std::result::Result<T, GameError>;

impl std::fmt::Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let msg = match self {
            GameError::SpaceOccupied => "Cannot add a move to an already occupied space",
        };
        write!(f, "{}", msg)
    }
}

