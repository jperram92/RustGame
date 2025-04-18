use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Player {
    X,
    O,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Cell {
    Empty,
    Occupied(Player),
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum GameStatus {
    InProgress,
    Won(Player),
    Draw,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub id: Uuid,
    pub board: [[Cell; 3]; 3],
    pub current_turn: Player,
    pub status: GameStatus,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            board: [[Cell::Empty; 3]; 3],
            current_turn: Player::X,
            status: GameStatus::InProgress,
        }
    }
}