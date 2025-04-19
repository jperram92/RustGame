use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{GameError, GameResult};
use crate::history::GameHistory;
use crate::player::Player;

/// Represents a cell on the game board
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Cell {
    /// An empty cell
    Empty,
    /// A cell occupied by a player
    Occupied(Player),
}

/// Represents the current status of the game
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum GameStatus {
    /// The game is still in progress
    InProgress,
    /// The game has been won by the specified player
    Won(Player),
    /// The game ended in a draw
    Draw,
}

/// Represents the complete state of a tic-tac-toe game
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    /// Unique identifier for the game
    pub id: Uuid,
    /// The 3x3 game board
    pub board: [[Cell; 3]; 3],
    /// The player whose turn it is
    pub current_turn: Player,
    /// The current status of the game
    pub status: GameStatus,
}

impl GameState {
    /// Creates a new game with an empty board and a new UUID
    ///
    /// # Examples
    ///
    /// ```
    /// use my_game_project::game::GameState;
    /// use my_game_project::player::Player;
    /// use my_game_project::game::GameStatus;
    ///
    /// let game = GameState::new();
    /// assert_eq!(game.current_turn, Player::X);
    /// assert_eq!(game.status, GameStatus::InProgress);
    /// ```
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            board: [[Cell::Empty; 3]; 3],
            current_turn: Player::X,
            status: GameStatus::InProgress,
        }
    }

    /// Creates a new game with a specific UUID
    ///
    /// This is useful for reconstructing games from history
    pub fn new_with_id(id: Uuid) -> Self {
        Self {
            id,
            board: [[Cell::Empty; 3]; 3],
            current_turn: Player::X,
            status: GameStatus::InProgress,
        }
    }

    /// Makes a move at the specified position
    ///
    /// # Arguments
    ///
    /// * `row` - The row index (0-2)
    /// * `col` - The column index (0-2)
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the move was successful
    /// * `Err(GameError)` if the move was invalid
    ///
    /// # Examples
    ///
    /// ```
    /// use my_game_project::game::GameState;
    ///
    /// let mut game = GameState::new();
    /// assert!(game.make_move(0, 0).is_ok());
    /// assert!(game.make_move(0, 0).is_err()); // Cell already occupied
    /// ```
    pub fn make_move(&mut self, row: usize, col: usize) -> GameResult<()> {
        // Check if the game is already finished
        if self.status != GameStatus::InProgress {
            return Err(GameError::GameAlreadyFinished);
        }

        // Check if the position is valid
        if row >= 3 || col >= 3 {
            return Err(GameError::InvalidPosition(row, col));
        }

        // Check if the cell is empty
        match self.board[row][col] {
            Cell::Empty => {
                // Make the move
                self.board[row][col] = Cell::Occupied(self.current_turn);

                // Check for win or draw
                self.update_game_status(row, col);

                // Switch turns if the game is still in progress
                if self.status == GameStatus::InProgress {
                    self.current_turn = self.current_turn.opponent();
                }

                Ok(())
            }
            Cell::Occupied(_) => Err(GameError::CellOccupied(row, col)),
        }
    }

    /// Updates the game status after a move
    ///
    /// This method checks if the last move resulted in a win or a draw
    /// and updates the game status accordingly.
    ///
    /// # Arguments
    ///
    /// * `last_row` - The row of the last move
    /// * `last_col` - The column of the last move
    fn update_game_status(&mut self, last_row: usize, last_col: usize) {
        // Get the player who just made a move
        let player = self.current_turn;

        // Check row
        if (0..3).all(|col| matches!(self.board[last_row][col], Cell::Occupied(p) if p == player)) {
            self.status = GameStatus::Won(player);
            return;
        }

        // Check column
        if (0..3).all(|row| matches!(self.board[row][last_col], Cell::Occupied(p) if p == player)) {
            self.status = GameStatus::Won(player);
            return;
        }

        // Check diagonal (top-left to bottom-right)
        if last_row == last_col &&
           (0..3).all(|i| matches!(self.board[i][i], Cell::Occupied(p) if p == player)) {
            self.status = GameStatus::Won(player);
            return;
        }

        // Check diagonal (top-right to bottom-left)
        if last_row + last_col == 2 &&
           (0..3).all(|i| matches!(self.board[i][2-i], Cell::Occupied(p) if p == player)) {
            self.status = GameStatus::Won(player);
            return;
        }

        // Check for draw (all cells filled)
        if self.board.iter().all(|row| row.iter().all(|cell| !matches!(cell, Cell::Empty))) {
            self.status = GameStatus::Draw;
            return;
        }
    }

    /// Prints the current board state to the console
    pub fn print_board(&self) {
        println!("Current board:");
        println!("-------------");

        for row in &self.board {
            print!("|");
            for cell in row {
                match cell {
                    Cell::Empty => print!("   |"),
                    Cell::Occupied(Player::X) => print!(" X |"),
                    Cell::Occupied(Player::O) => print!(" O |"),
                }
            }
            println!("");
            println!("-------------");
        }
    }

    /// Creates a new game history for this game
    pub fn create_history(&self) -> GameHistory {
        GameHistory::new(self.id)
    }

    /// Saves the game state to a file in JSON format
    pub fn save_to_file(&self, filename: &str) -> GameResult<()> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| GameError::SerializationError(e.to_string()))?;

        std::fs::write(filename, json)
            .map_err(|e| GameError::IoError(e.to_string()))?;

        Ok(())
    }

    /// Loads a game state from a file
    pub fn load_from_file(filename: &str) -> GameResult<Self> {
        let json = std::fs::read_to_string(filename)
            .map_err(|e| GameError::IoError(e.to_string()))?;

        let game = serde_json::from_str(&json)
            .map_err(|e| GameError::DeserializationError(e.to_string()))?;

        Ok(game)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game() {
        let game = GameState::new();
        assert_eq!(game.current_turn, Player::X);
        assert_eq!(game.status, GameStatus::InProgress);

        // Check that all cells are empty
        for row in 0..3 {
            for col in 0..3 {
                assert_eq!(game.board[row][col], Cell::Empty);
            }
        }
    }

    #[test]
    fn test_make_move() {
        let mut game = GameState::new();

        // Make a valid move
        assert!(game.make_move(0, 0).is_ok());
        assert_eq!(game.board[0][0], Cell::Occupied(Player::X));
        assert_eq!(game.current_turn, Player::O); // Turn should switch

        // Try to make a move on an occupied cell
        assert!(game.make_move(0, 0).is_err());

        // Try to make a move out of bounds
        assert!(game.make_move(3, 0).is_err());
    }

    #[test]
    fn test_win_conditions() {
        // Test row win
        let mut game = GameState::new();
        game.make_move(0, 0).unwrap(); // X at (0,0)
        game.make_move(1, 0).unwrap(); // O at (1,0)
        game.make_move(0, 1).unwrap(); // X at (0,1)
        game.make_move(1, 1).unwrap(); // O at (1,1)
        game.make_move(0, 2).unwrap(); // X at (0,2)
        assert_eq!(game.status, GameStatus::Won(Player::X));

        // Test column win
        let mut game = GameState::new();
        game.make_move(0, 0).unwrap(); // X at (0,0)
        game.make_move(0, 1).unwrap(); // O at (0,1)
        game.make_move(1, 0).unwrap(); // X at (1,0)
        game.make_move(1, 1).unwrap(); // O at (1,1)
        game.make_move(2, 0).unwrap(); // X at (2,0)
        assert_eq!(game.status, GameStatus::Won(Player::X));

        // Test diagonal win (top-left to bottom-right)
        let mut game = GameState::new();
        game.make_move(0, 0).unwrap(); // X at (0,0)
        game.make_move(0, 1).unwrap(); // O at (0,1)
        game.make_move(1, 1).unwrap(); // X at (1,1)
        game.make_move(0, 2).unwrap(); // O at (0,2)
        game.make_move(2, 2).unwrap(); // X at (2,2)
        assert_eq!(game.status, GameStatus::Won(Player::X));

        // Test diagonal win (top-right to bottom-left)
        let mut game = GameState::new();
        game.make_move(0, 2).unwrap(); // X at (0,2)
        game.make_move(0, 0).unwrap(); // O at (0,0)
        game.make_move(1, 1).unwrap(); // X at (1,1)
        game.make_move(0, 1).unwrap(); // O at (0,1)
        game.make_move(2, 0).unwrap(); // X at (2,0)
        assert_eq!(game.status, GameStatus::Won(Player::X));
    }

    #[test]
    fn test_draw() {
        let mut game = GameState::new();
        // Fill the board in a way that doesn't result in a win
        // X | O | X
        // O | O | X
        // X | X | O
        game.make_move(0, 0).unwrap(); // X at (0,0)
        game.make_move(0, 1).unwrap(); // O at (0,1)
        game.make_move(0, 2).unwrap(); // X at (0,2)
        game.make_move(1, 0).unwrap(); // O at (1,0)
        game.make_move(1, 2).unwrap(); // X at (1,2)
        game.make_move(1, 1).unwrap(); // O at (1,1)
        game.make_move(2, 0).unwrap(); // X at (2,0)
        game.make_move(2, 2).unwrap(); // O at (2,2)
        game.make_move(2, 1).unwrap(); // X at (2,1)
        assert_eq!(game.status, GameStatus::Draw);
    }

    #[test]
    fn test_game_already_finished() {
        let mut game = GameState::new();
        // Create a winning condition
        game.make_move(0, 0).unwrap();
        game.make_move(1, 0).unwrap();
        game.make_move(0, 1).unwrap();
        game.make_move(1, 1).unwrap();
        game.make_move(0, 2).unwrap();
        // Game should be won by X now
        assert_eq!(game.status, GameStatus::Won(Player::X));

        // Try to make another move
        let result = game.make_move(2, 2);
        assert!(result.is_err());
        match result {
            Err(GameError::GameAlreadyFinished) => {}, // Expected
            _ => panic!("Expected GameAlreadyFinished error"),
        }
    }
}
