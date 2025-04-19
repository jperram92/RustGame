use thiserror::Error;

/// Represents errors that can occur during game operations
#[derive(Error, Debug, Clone)]
pub enum GameError {
    /// The cell at the specified position is already occupied
    #[error("Cell at position ({0}, {1}) is already occupied")]
    CellOccupied(usize, usize),

    /// The specified position is outside the board boundaries
    #[error("Invalid position: ({0}, {1}) is out of bounds")]
    InvalidPosition(usize, usize),

    /// The game is already finished (won or drawn)
    #[error("Game is already finished")]
    GameAlreadyFinished,

    /// Not the player's turn
    #[error("Not player's turn")]
    NotPlayerTurn,

    /// IO error occurred
    #[error("IO error: {0}")]
    IoError(String),

    /// Error during serialization
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Error during deserialization
    #[error("Deserialization error: {0}")]
    DeserializationError(String),

    /// Game not found
    #[error("Game with ID {0} not found")]
    GameNotFound(String),

    /// No valid moves available
    #[error("No valid moves available")]
    NoValidMoves,

    /// Invalid player type
    #[error("Invalid player type: {0}")]
    InvalidPlayerType(String),
}

/// A specialized Result type for game operations
pub type GameResult<T> = Result<T, GameError>;
