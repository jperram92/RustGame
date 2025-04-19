use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;
use uuid::Uuid;

use crate::game::GameState;

/// Shared application state for the HTTP server
#[derive(Debug, Clone)]
pub struct AppState {
    /// Map of game ID to game state
    pub games: Arc<RwLock<HashMap<Uuid, GameState>>>,
}

impl AppState {
    /// Create a new application state
    pub fn new() -> Self {
        Self {
            games: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}
