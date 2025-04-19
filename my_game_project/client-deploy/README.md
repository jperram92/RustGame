# Tic-Tac-Toe Online Client

This is the web client for the Tic-Tac-Toe Online game. It connects to a Rust-based backend server to provide a multiplayer tic-tac-toe experience.

## How to Play

1. Visit the hosted version at [your-github-username.github.io/tic-tac-toe-client](https://your-github-username.github.io/tic-tac-toe-client)
2. Click "New Game" to start a new game
3. Make moves by clicking on the board
4. Use the "AI Move" button to have the AI make a move
5. Select different difficulty levels from the dropdown

## Local Development

To run this client locally:

1. Clone this repository
2. Open `index.html` in your web browser
3. Make sure the backend server is running

## Connecting to Your Own Server

To connect this client to your own server:

1. Edit the `API_URL` variable in `index.html`:
   ```javascript
   const API_URL = 'https://your-server-url.com';
   ```

2. Deploy the updated client

## Deployment

This client can be easily deployed to GitHub Pages:

1. Fork this repository
2. Go to the repository settings
3. Navigate to the "Pages" section
4. Select the main branch as the source
5. Click "Save"

Your client will be available at `https://your-github-username.github.io/tic-tac-toe-client`

## Technologies Used

- HTML
- CSS
- JavaScript (vanilla)
- Fetch API for server communication

## Backend

The backend for this client is built with Rust using:
- Tokio for async runtime
- Axum for the web framework
- Serde for serialization

The backend code is available at [your-github-username/tic-tac-toe-server](https://github.com/your-github-username/tic-tac-toe-server)
