use std::io::{self, Write};
use std::path::Path;

use my_game_project::ai::{MinimaxAI, Difficulty};
use my_game_project::game::{GameState, GameStatus};
use my_game_project::error::{GameError, GameResult};
use my_game_project::history::GameHistory;
use my_game_project::player::{GamePlayer, HumanPlayer, Player};

fn main() -> GameResult<()> {
    println!("Welcome to Tic-Tac-Toe in Rust!");
    println!("==========================");

    // Check if we should load a saved game
    let mut game = if let Some(filename) = get_load_game_option()? {
        load_game(&filename)?
    } else {
        // Create a new game
        let game = GameState::new();
        println!("Game created with ID: {}", game.id);
        println!("Player X goes first\n");
        game
    };

    // Create a history object
    let mut history = game.create_history();

    // Set up players
    let game_mode = get_game_mode()?;
    let (player1, player2) = create_players(game_mode)?;

    println!("\nPlayer 1: {}", player1.get_name());
    println!("Player 2: {}\n", player2.get_name());

    // Main game loop
    loop {
        // Display the current board
        game.print_board();

        // Check if the game is over
        match game.status {
            GameStatus::Won(player) => {
                println!("Player {:?} wins!", player);
                history.finish(game.status);
                break;
            }
            GameStatus::Draw => {
                println!("It's a draw!");
                history.finish(game.status);
                break;
            }
            GameStatus::InProgress => {
                println!("Player {:?}'s turn", game.current_turn);
            }
        }

        // Get the current player
        let current_player = if game.current_turn == Player::X {
            &player1
        } else {
            &player2
        };

        println!("{}'s turn", current_player.get_name());

        // Get the player's move
        let (row, col) = current_player.get_move(&game)?;

        // Make the move
        match game.make_move(row, col) {
            Ok(()) => {
                // Record the move in history
                history.add_move(game.current_turn.opponent(), row, col);
                println!("Move successful!\n");

                // Save the game after each move
                save_game_option(&game, &history)?;
            }
            Err(e) => {
                println!("Error: {}\nPlease try again.\n", e);
                continue;
            }
        }
    }

    // Final board state
    println!("\nFinal board state:");
    game.print_board();

    // Save the final game state and history
    save_game_option(&game, &history)?;

    println!("Thanks for playing!");

    Ok(())
}

/// Game modes for the tic-tac-toe game
#[derive(Debug, Clone, Copy, PartialEq)]
enum GameMode {
    /// Human vs Human
    HumanVsHuman,
    /// Human vs AI (Easy)
    HumanVsAIEasy,
    /// Human vs AI (Medium)
    HumanVsAIMedium,
    /// Human vs AI (Hard)
    HumanVsAIHard,
}

/// Get the game mode from the user
fn get_game_mode() -> GameResult<GameMode> {
    println!("Select game mode:");
    println!("1. Human vs Human");
    println!("2. Human vs AI (Easy)");
    println!("3. Human vs AI (Medium)");
    println!("4. Human vs AI (Hard)");

    print!("Enter your choice (1-4): ");
    io::stdout().flush().map_err(|e| {
        GameError::IoError(e.to_string())
    })?;

    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|e| {
        GameError::IoError(e.to_string())
    })?;

    match input.trim() {
        "1" => Ok(GameMode::HumanVsHuman),
        "2" => Ok(GameMode::HumanVsAIEasy),
        "3" => Ok(GameMode::HumanVsAIMedium),
        "4" => Ok(GameMode::HumanVsAIHard),
        _ => {
            println!("Invalid choice. Defaulting to Human vs Human.");
            Ok(GameMode::HumanVsHuman)
        }
    }
}

/// Create players based on the selected game mode
fn create_players(mode: GameMode) -> GameResult<(Box<dyn GamePlayer>, Box<dyn GamePlayer>)> {

    match mode {
        GameMode::HumanVsHuman => {
            let player1 = Box::new(HumanPlayer::new(Player::X, "Player 1".to_string()));
            let player2 = Box::new(HumanPlayer::new(Player::O, "Player 2".to_string()));
            Ok((player1, player2))
        },
        GameMode::HumanVsAIEasy => {
            let player1 = Box::new(HumanPlayer::new(Player::X, "Player".to_string()));
            let player2 = Box::new(MinimaxAI::new(Player::O, Difficulty::Easy));
            Ok((player1, player2))
        },
        GameMode::HumanVsAIMedium => {
            let player1 = Box::new(HumanPlayer::new(Player::X, "Player".to_string()));
            let player2 = Box::new(MinimaxAI::new(Player::O, Difficulty::Medium));
            Ok((player1, player2))
        },
        GameMode::HumanVsAIHard => {
            let player1 = Box::new(HumanPlayer::new(Player::X, "Player".to_string()));
            let player2 = Box::new(MinimaxAI::new(Player::O, Difficulty::Hard));
            Ok((player1, player2))
        },
    }
}

/// Asks the user if they want to load a saved game
fn get_load_game_option() -> GameResult<Option<String>> {
    print!("Do you want to load a saved game? (y/n): ");
    io::stdout().flush().map_err(|e| {
        GameError::IoError(e.to_string())
    })?;

    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|e| {
        GameError::IoError(e.to_string())
    })?;

    if input.trim().to_lowercase() == "y" {
        print!("Enter the filename to load: ");
        io::stdout().flush().map_err(|e| {
            GameError::IoError(e.to_string())
        })?;

        let mut filename = String::new();
        io::stdin().read_line(&mut filename).map_err(|e| {
            GameError::IoError(e.to_string())
        })?;

        Ok(Some(filename.trim().to_string()))
    } else {
        Ok(None)
    }
}

/// Loads a game from a file
fn load_game(filename: &str) -> GameResult<GameState> {
    if !Path::new(filename).exists() {
        return Err(GameError::IoError(format!("File '{}' not found", filename)));
    }

    println!("Loading game from {}...", filename);
    let game = GameState::load_from_file(filename)?;
    println!("Game loaded successfully!");

    Ok(game)
}

/// Asks the user if they want to save the game
fn save_game_option(game: &GameState, history: &GameHistory) -> GameResult<()> {
    // Auto-save the game state
    let game_filename = format!("game_{}.json", game.id);
    game.save_to_file(&game_filename)?;

    // Auto-save the history
    let history_filename = format!("history_{}.json", game.id);
    history.save_to_file(&history_filename)?;

    Ok(())
}