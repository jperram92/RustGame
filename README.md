# Rust Game - Tic-Tac-Toe Online

A multiplayer Tic-Tac-Toe game built with Rust on the backend and HTML/JavaScript on the frontend.

## Play Online

You can play the game online at: [https://jperram92.github.io/RustGame/](https://jperram92.github.io/RustGame/)

## Features

- Play Tic-Tac-Toe against an AI opponent
- Multiple AI difficulty levels
- RESTful API built with Rust and Axum
- Responsive web interface

## Project Structure

- `my_game_project/` - The Rust backend server
  - Built with Axum, Tokio, and Serde
  - Implements game logic and AI
- `client-deploy/` - The web client for GitHub Pages
- `docs/` - Documentation and learning resources

## Development

### Prerequisites

- Rust and Cargo
- Visual C++ build tools (for Windows)

### Running the Server Locally

```bash
cd my_game_project
cargo run --bin server
```

### Running the Client Locally

Open `client/index.html` in your web browser.

## Deployment

The game is deployed in two parts:
1. Frontend: GitHub Pages
2. Backend: Render.com

## Learning Resources

This project includes extensive documentation in the `docs/` folder about:
- Rust programming concepts
- Game development patterns
- API design
- Deployment strategies

## License

MIT
