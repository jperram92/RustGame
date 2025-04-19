use std::io::{self, Write};

use crate::error::{GameError, GameResult};
use crate::game::GameState;
use crate::player::{GamePlayer, Player};

/// A human player that gets moves from stdin
pub struct HumanPlayer {
    /// The player type (X or O)
    player_type: Player,
    /// The player's name
    name: String,
}

impl HumanPlayer {
    /// Create a new human player
    pub fn new(player_type: Player, name: String) -> Self {
        Self { player_type, name }
    }
    
    /// Get a move from the user via stdin
    fn get_player_move_from_stdin(&self) -> GameResult<(usize, usize)> {
        loop {
            print!("Enter your move as 'row col' (0-2): ");
            io::stdout().flush().map_err(|e| {
                GameError::IoError(e.to_string())
            })?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).map_err(|e| {
                GameError::IoError(e.to_string())
            })?;
            
            // Parse the input
            let coords: Vec<&str> = input.trim().split_whitespace().collect();
            
            if coords.len() != 2 {
                println!("Please enter exactly two numbers separated by a space.");
                continue;
            }
            
            // Try to parse the coordinates
            match (coords[0].parse::<usize>(), coords[1].parse::<usize>()) {
                (Ok(row), Ok(col)) => {
                    if row <= 2 && col <= 2 {
                        return Ok((row, col));
                    } else {
                        println!("Row and column must be between 0 and 2.");
                    }
                }
                _ => println!("Invalid input. Please enter numbers."),
            }
        }
    }
}

impl GamePlayer for HumanPlayer {
    fn get_move(&self, _game: &GameState) -> GameResult<(usize, usize)> {
        self.get_player_move_from_stdin()
    }
    
    fn get_player_type(&self) -> Player {
        self.player_type
    }
    
    fn get_name(&self) -> String {
        format!("{} (Human)", self.name)
    }
}
