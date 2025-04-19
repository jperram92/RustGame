# Tic-Tac-Toe Game

A Rust implementation of Tic-Tac-Toe with an AI opponent and online multiplayer functionality.

## Features

- Command-line interface for local play
- AI opponent with multiple difficulty levels
- RESTful API for online play
- Game state serialization and history tracking
- Web client for easy access

## Local Development

### Prerequisites

- Rust and Cargo (https://rustup.rs/)
- Web browser for the client

### Running the CLI Game

```bash
cargo run --bin tictactoe
```

### Running the Server

```bash
cargo run --bin server
```

The server will start on http://localhost:3000 by default.

### Playing the Game

1. Start the server
2. Open `client/index.html` in your web browser
3. Click "New Game" to start a new game
4. Make moves by clicking on the board
5. Use the "AI Move" button to have the AI make a move

## Deployment

### Deploying to Render

1. Fork this repository to your GitHub account
2. Sign up for a Render account at https://render.com/
3. In your Render dashboard, click "New" and select "Blueprint"
4. Connect your GitHub repository
5. Render will detect the `render.yaml` file and set up the service
6. Click "Apply" to deploy

### Updating the Client

After deployment, update the API_URL in `client/index.html`:

```javascript
const API_URL = 'https://your-app-name.onrender.com';
```

### Hosting the Client

You can host the client on:

1. **GitHub Pages**:
   - Create a repository for the client
   - Push the contents of the `client` folder
   - Enable GitHub Pages in the repository settings

2. **Netlify**:
   - Sign up at https://www.netlify.com/
   - Drag and drop the `client` folder to deploy

## License

MIT

## Acknowledgements

- Built with Rust, Tokio, and Axum
- AI uses the minimax algorithm
