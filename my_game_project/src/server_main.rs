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
    
    // Define the address to listen on
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    
    // Start the server
    info!("Starting server on {}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
