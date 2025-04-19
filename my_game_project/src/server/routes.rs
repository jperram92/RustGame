use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

use crate::server::handlers;
use crate::server::state::AppState;

/// Create the router for the HTTP server
pub fn create_router() -> Router<AppState> {
    // Create a CORS layer
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Create the router
    Router::new()
        // Game routes
        .route("/games", get(handlers::list_games))
        .route("/games", post(handlers::create_game))
        .route("/games/:id", get(handlers::get_game))
        .route("/games/:id/move", post(handlers::make_move))
        .route("/games/:id/ai-move", post(handlers::make_ai_move))
        // Add the CORS layer
        .layer(cors)
}
