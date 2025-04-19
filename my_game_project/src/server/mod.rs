//! HTTP server for the tic-tac-toe game
//!
//! This module provides a REST API for playing tic-tac-toe over HTTP.

mod routes;
mod state;
mod handlers;

pub use routes::create_router;
pub use state::AppState;
