use std::time::Instant;

use serde::{Deserialize, Serialize};

use crate::error::{GameError, GameResult};
use crate::game::{Cell, GameState, GameStatus};
use crate::player::{GamePlayer, Player};

/// Difficulty levels for the AI
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Difficulty {
    /// Easy difficulty - makes random valid moves
    Easy,
    /// Medium difficulty - uses minimax but with limited depth
    Medium,
    /// Hard difficulty - uses full minimax algorithm
    Hard,
}

/// An AI player that uses the minimax algorithm
pub struct MinimaxAI {
    /// The player type (X or O)
    player_type: Player,
    /// The difficulty level
    difficulty: Difficulty,
}

impl MinimaxAI {
    /// Create a new AI player
    pub fn new(player_type: Player, difficulty: Difficulty) -> Self {
        Self { player_type, difficulty }
    }

    /// Get the maximum search depth based on difficulty
    fn get_max_depth(&self) -> usize {
        match self.difficulty {
            Difficulty::Easy => 1,
            Difficulty::Medium => 3,
            Difficulty::Hard => 9, // Full search for tic-tac-toe
        }
    }

    /// Evaluate the board state for the minimax algorithm
    fn evaluate(&self, game: &GameState) -> i32 {
        match game.status {
            GameStatus::Won(player) => {
                if player == self.player_type {
                    10 // AI wins
                } else {
                    -10 // AI loses
                }
            }
            GameStatus::Draw => 0, // Draw
            GameStatus::InProgress => 0, // Game still in progress
        }
    }

    /// Find the best move using the minimax algorithm
    fn find_best_move(&self, game: &GameState) -> GameResult<(usize, usize)> {
        // If it's easy difficulty, just make a random valid move
        if self.difficulty == Difficulty::Easy {
            return self.find_random_move(game);
        }

        let max_depth = self.get_max_depth();
        let mut best_score = i32::MIN;
        let mut best_move = None;

        // Try each empty cell
        for row in 0..3 {
            for col in 0..3 {
                if let Cell::Empty = game.board[row][col] {
                    // Make a temporary move
                    let mut game_copy = game.clone();
                    game_copy.make_move(row, col)?;

                    // Calculate score for this move
                    let score = self.minimax(&game_copy, 0, max_depth, false);

                    // Update best move if this is better
                    if score > best_score {
                        best_score = score;
                        best_move = Some((row, col));
                    }
                }
            }
        }

        best_move.ok_or_else(|| GameError::NoValidMoves)
    }

    /// Find a random valid move
    fn find_random_move(&self, game: &GameState) -> GameResult<(usize, usize)> {
        let mut empty_cells = Vec::new();

        // Find all empty cells
        for row in 0..3 {
            for col in 0..3 {
                if let Cell::Empty = game.board[row][col] {
                    empty_cells.push((row, col));
                }
            }
        }

        // Pick a random empty cell
        if empty_cells.is_empty() {
            return Err(GameError::NoValidMoves);
        }

        let random_index = (Instant::now().elapsed().as_nanos() % empty_cells.len() as u128) as usize;
        Ok(empty_cells[random_index])
    }

    /// The minimax algorithm implementation
    fn minimax(&self, game: &GameState, depth: usize, max_depth: usize, is_maximizing: bool) -> i32 {
        // Base cases: terminal state or maximum depth reached
        if game.status != GameStatus::InProgress || depth == max_depth {
            return self.evaluate(game) - depth as i32; // Prefer shorter paths to victory
        }

        if is_maximizing {
            // Maximizing player (AI)
            let mut best_score = i32::MIN;

            // Try each empty cell
            for row in 0..3 {
                for col in 0..3 {
                    if let Cell::Empty = game.board[row][col] {
                        // Make a temporary move
                        let mut game_copy = game.clone();
                        if game_copy.make_move(row, col).is_ok() {
                            // Calculate score for this move
                            let score = self.minimax(&game_copy, depth + 1, max_depth, false);
                            best_score = best_score.max(score);
                        }
                    }
                }
            }

            best_score
        } else {
            // Minimizing player (opponent)
            let mut best_score = i32::MAX;

            // Try each empty cell
            for row in 0..3 {
                for col in 0..3 {
                    if let Cell::Empty = game.board[row][col] {
                        // Make a temporary move
                        let mut game_copy = game.clone();
                        if game_copy.make_move(row, col).is_ok() {
                            // Calculate score for this move
                            let score = self.minimax(&game_copy, depth + 1, max_depth, true);
                            best_score = best_score.min(score);
                        }
                    }
                }
            }

            best_score
        }
    }
}

impl GamePlayer for MinimaxAI {
    fn get_move(&self, game: &GameState) -> GameResult<(usize, usize)> {
        println!("AI is thinking...");
        let start = Instant::now();

        let result = self.find_best_move(game);

        let duration = start.elapsed();
        println!("AI decided in {:.2?}", duration);

        result
    }

    fn get_player_type(&self) -> Player {
        self.player_type
    }

    fn get_name(&self) -> String {
        format!("AI ({:?})", self.difficulty)
    }
}
