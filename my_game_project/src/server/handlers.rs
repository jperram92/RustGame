use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ai::{Difficulty, MinimaxAI};
use crate::game::{GameState, GameStatus};
use crate::player::{GamePlayer, Player};
use crate::server::state::AppState;

/// Response for listing games
#[derive(Debug, Serialize)]
pub struct GamesListResponse {
    /// List of game IDs
    pub games: Vec<GameSummary>,
}

/// Summary of a game
#[derive(Debug, Serialize)]
pub struct GameSummary {
    /// Game ID
    pub id: Uuid,
    /// Current status of the game
    pub status: GameStatus,
    /// Current turn
    pub current_turn: Player,
}

/// Request for creating a game
#[derive(Debug, Deserialize)]
pub struct CreateGameRequest {
    /// Optional player to start (defaults to X)
    pub starting_player: Option<Player>,
}

/// Request for making a move
#[derive(Debug, Deserialize)]
pub struct MakeMoveRequest {
    /// Row index (0-2)
    pub row: usize,
    /// Column index (0-2)
    pub col: usize,
    /// Player making the move
    pub player: Player,
}

/// Request for making an AI move
#[derive(Debug, Deserialize)]
pub struct MakeAIMoveRequest {
    /// Difficulty level for the AI
    pub difficulty: Difficulty,
}

/// List all games
pub async fn list_games(
    State(state): State<AppState>,
) -> Result<Json<GamesListResponse>, StatusCode> {
    let games = state.games.read().await;
    
    let game_summaries = games
        .iter()
        .map(|(id, game)| GameSummary {
            id: *id,
            status: game.status,
            current_turn: game.current_turn,
        })
        .collect();
    
    Ok(Json(GamesListResponse {
        games: game_summaries,
    }))
}

/// Create a new game
pub async fn create_game(
    State(state): State<AppState>,
    Json(request): Json<CreateGameRequest>,
) -> Result<Json<GameState>, StatusCode> {
    let mut game = GameState::new();
    
    // Set the starting player if specified
    if let Some(starting_player) = request.starting_player {
        game.current_turn = starting_player;
    }
    
    // Add the game to the state
    let game_id = game.id;
    state.games.write().await.insert(game_id, game.clone());
    
    Ok(Json(game))
}

/// Get a game by ID
pub async fn get_game(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<GameState>, StatusCode> {
    let games = state.games.read().await;
    
    let game = games.get(&id).ok_or(StatusCode::NOT_FOUND)?;
    
    Ok(Json(game.clone()))
}

/// Make a move in a game
pub async fn make_move(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(request): Json<MakeMoveRequest>,
) -> Result<Json<GameState>, StatusCode> {
    // Get the game
    let mut games = state.games.write().await;
    let game = games.get_mut(&id).ok_or(StatusCode::NOT_FOUND)?;
    
    // Verify it's the correct player's turn
    if game.current_turn != request.player {
        return Err(StatusCode::BAD_REQUEST);
    }
    
    // Make the move
    game.make_move(request.row, request.col)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    Ok(Json(game.clone()))
}

/// Make an AI move in a game
pub async fn make_ai_move(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(request): Json<MakeAIMoveRequest>,
) -> Result<Json<GameState>, StatusCode> {
    // Get the game
    let mut games = state.games.write().await;
    let game = games.get_mut(&id).ok_or(StatusCode::NOT_FOUND)?;
    
    // Create an AI player
    let ai = MinimaxAI::new(game.current_turn, request.difficulty);
    
    // Get the AI's move
    let (row, col) = ai.get_move(game)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    // Make the move
    game.make_move(row, col)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    
    Ok(Json(game.clone()))
}
