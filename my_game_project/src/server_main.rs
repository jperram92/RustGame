use std::net::SocketAddr;

use axum::Server;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

use my_game_project::server::{create_router, AppState};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Create the application state
    let state = AppState::new();

    // Create the router
    let app = create_router().with_state(state);

    // Get the port from the environment variable or use 3000 as default
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("PORT must be a number");

    // Define the address to listen on all interfaces
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    // Start the server
    info!("Starting server on {}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
