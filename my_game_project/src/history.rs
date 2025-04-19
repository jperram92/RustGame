use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{GameError, GameResult};
use crate::game::{GameState, GameStatus};
use crate::player::Player;

/// Represents a single move in the game
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameMove {
    /// The player who made the move
    pub player: Player,
    /// The row where the move was made (0-2)
    pub row: usize,
    /// The column where the move was made (0-2)
    pub col: usize,
    /// The timestamp when the move was made
    pub timestamp: DateTime<Utc>,
}

impl GameMove {
    /// Creates a new game move
    pub fn new(player: Player, row: usize, col: usize) -> Self {
        Self {
            player,
            row,
            col,
            timestamp: Utc::now(),
        }
    }
}

/// Represents the complete history of a game
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameHistory {
    /// The unique identifier of the game
    pub game_id: Uuid,
    /// The list of moves in chronological order
    pub moves: Vec<GameMove>,
    /// The timestamp when the game started
    pub started_at: DateTime<Utc>,
    /// The timestamp when the game ended (if it has ended)
    pub ended_at: Option<DateTime<Utc>>,
    /// The final status of the game (if it has ended)
    pub final_status: Option<GameStatus>,
}

impl GameHistory {
    /// Creates a new game history
    pub fn new(game_id: Uuid) -> Self {
        Self {
            game_id,
            moves: Vec::new(),
            started_at: Utc::now(),
            ended_at: None,
            final_status: None,
        }
    }

    /// Adds a move to the history
    pub fn add_move(&mut self, player: Player, row: usize, col: usize) {
        let game_move = GameMove::new(player, row, col);
        self.moves.push(game_move);
    }

    /// Marks the game as finished
    pub fn finish(&mut self, status: GameStatus) {
        self.ended_at = Some(Utc::now());
        self.final_status = Some(status);
    }

    /// Saves the game history to a file
    pub fn save_to_file(&self, filename: &str) -> GameResult<()> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| GameError::SerializationError(e.to_string()))?;
        
        std::fs::write(filename, json)
            .map_err(|e| GameError::IoError(e.to_string()))?;
        
        Ok(())
    }

    /// Loads a game history from a file
    pub fn load_from_file(filename: &str) -> GameResult<Self> {
        let json = std::fs::read_to_string(filename)
            .map_err(|e| GameError::IoError(e.to_string()))?;
        
        let history = serde_json::from_str(&json)
            .map_err(|e| GameError::DeserializationError(e.to_string()))?;
        
        Ok(history)
    }

    /// Reconstructs a game state from the history
    pub fn reconstruct_game(&self) -> GameResult<GameState> {
        let mut game = GameState::new_with_id(self.game_id);
        
        for game_move in &self.moves {
            // Verify that it's the correct player's turn
            if game.current_turn != game_move.player {
                return Err(GameError::NotPlayerTurn);
            }
            
            // Apply the move
            game.make_move(game_move.row, game_move.col)?;
        }
        
        Ok(game)
    }
}
