use serde::{Deserialize, Serialize};

/// Represents a player in the game (X or O)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Player {
    /// The X player (usually goes first)
    X,
    /// The O player
    O,
}

impl Player {
    /// Returns the opponent of the current player
    /// 
    /// # Examples
    /// 
    /// ```
    /// use my_game_project::player::Player;
    /// 
    /// let player = Player::X;
    /// assert_eq!(player.opponent(), Player::O);
    /// ```
    pub fn opponent(&self) -> Self {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}
