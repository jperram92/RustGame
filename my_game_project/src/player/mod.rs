mod player_type;
mod human_player;

pub use player_type::Player;
pub use human_player::HumanPlayer;

use crate::error::GameResult;
use crate::game::GameState;

/// A trait for entities that can make moves in the game
pub trait GamePlayer {
    /// Get the next move from this player
    fn get_move(&self, game: &GameState) -> GameResult<(usize, usize)>;

    /// Get the player type (X or O)
    fn get_player_type(&self) -> Player;

    /// Get a descriptive name for this player
    fn get_name(&self) -> String;
}
