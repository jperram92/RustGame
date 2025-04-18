use thiserror::Error;

#[derive(Error, Debug)]
pub enum GameError {
    #[error("Invalid move: Cell at position ({0}, {1}) is already occupied")]
    CellOccupied(usize, usize),
    
    #[error("Invalid position: ({0}, {1}) is out of bounds")]
    InvalidPosition(usize, usize),
    
    #[error("Game is already finished")]
    GameAlreadyFinished,
    
    #[error("Not player's turn")]
    NotPlayerTurn,
}