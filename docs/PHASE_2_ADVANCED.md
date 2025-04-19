# Phase 2: Advanced Features - Taking Your Rust Skills to the Next Level

Now that we've built a functional tic-tac-toe game, let's enhance it with more advanced Rust features. This phase will introduce you to concepts that are essential for building real-world Rust applications.

## What We'll Cover in Phase 2

1. **Code Organization and Modularity**
   - Proper module structure
   - Creating a library crate
   - Advanced error handling
   - Public vs. private APIs

2. **Serialization and Game State Management**
   - Using Serde for serialization
   - Working with JSON and other formats
   - Saving and loading game state
   - Custom serialization logic

3. **AI and Advanced Algorithms**
   - Implementing the minimax algorithm
   - Trait-based design for different player types
   - Generic programming in Rust
   - Performance optimization

4. **Asynchronous Programming**
   - Understanding async/await in Rust
   - Building a simple HTTP server
   - Handling concurrent games
   - Error handling in async code

## Prerequisites

Before starting Phase 2, make sure you:
- Have completed Phase 1 of the project
- Are comfortable with basic Rust syntax and concepts
- Have Rust and Cargo installed and working correctly

Let's begin by refactoring our code into a more maintainable structure!

## Commit 1: Code Organization and Modularity

### Module Structure in Rust

In our first commit of Phase 2, we've refactored our code into a proper module structure:

```rust
// lib.rs - The root of our library
pub mod game;
pub mod error;
pub mod player;
```

**Key concepts about Rust modules:**

- **Module Hierarchy**: Modules form a tree structure, starting from the crate root (lib.rs)
- **Visibility**: Use `pub` to make items accessible outside their module
- **Path Resolution**: Use `use` statements to bring items into scope
- **Re-exporting**: You can re-export items with `pub use` to simplify imports

### Library and Binary Crates

We've separated our code into a library and a binary:

```toml
# Cargo.toml
[lib]
name = "my_game_project"
path = "src/lib.rs"

[[bin]]
name = "tictactoe"
path = "src/main.rs"
```

**Benefits of this approach:**

- **Reusability**: The library can be used by multiple binaries
- **Testing**: Easier to test library code in isolation
- **API Design**: Forces you to think about your public interface
- **Documentation**: Better organization for documentation

### Error Handling with thiserror

We've improved our error handling using the `thiserror` crate:

```rust
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum GameError {
    #[error("Cell at position ({0}, {1}) is already occupied")]
    CellOccupied(usize, usize),

    // Other error variants...
}
```

**Advanced error handling concepts:**

- **Error Traits**: The `Error` trait provides a standard interface for errors
- **Error Propagation**: The `?` operator simplifies error handling
- **Custom Error Types**: Define domain-specific errors
- **Error Messages**: Provide clear, contextual error messages

### The ? Operator

We've used the `?` operator for concise error handling:

```rust
fn get_player_move() -> GameResult<(usize, usize)> {
    // ...
    io::stdout().flush().map_err(|e| {
        GameError::IoError(e.to_string())
    })?;
    // ...
}
```

**How the ? operator works:**

1. If the result is `Ok(value)`, it extracts the value
2. If the result is `Err(e)`, it returns early with the error
3. It can automatically convert between error types (via the `From` trait)
4. It's much more concise than using `match` for every operation

### Type Aliases for Results

We've created a type alias for our Result type:

```rust
pub type GameResult<T> = Result<T, GameError>;
```

**Benefits of type aliases:**

- **Readability**: Makes function signatures clearer
- **Consistency**: Ensures consistent error types
- **Flexibility**: Makes it easier to change the error type later
- **Documentation**: Communicates the purpose of the Result

## Commit 2: Game History and Serialization

### Serialization with Serde

In our second commit of Phase 2, we've added serialization support using the Serde framework:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    pub id: Uuid,
    pub board: [[Cell; 3]; 3],
    pub current_turn: Player,
    pub status: GameStatus,
}
```

**Key concepts about Serde:**

- **Derive Macros**: `#[derive(Serialize, Deserialize)]` automatically implements serialization
- **Format Agnostic**: Works with JSON, YAML, TOML, and other formats
- **Custom Serialization**: Can be customized for complex types
- **Zero-Copy Deserialization**: Efficient parsing of data

### Working with JSON

We've implemented JSON serialization and deserialization:

```rust
pub fn save_to_file(&self, filename: &str) -> GameResult<()> {
    let json = serde_json::to_string_pretty(self)
        .map_err(|e| GameError::SerializationError(e.to_string()))?;

    std::fs::write(filename, json)
        .map_err(|e| GameError::IoError(e.to_string()))?;

    Ok(())
}
```

**Working with JSON in Rust:**

- **serde_json**: Provides JSON-specific functionality
- **to_string_pretty**: Creates formatted JSON with indentation
- **from_str**: Parses JSON strings into Rust types
- **Error Handling**: Proper error handling for parsing failures

### Game History Tracking

We've implemented a history system to track game moves:

```rust
pub struct GameHistory {
    pub game_id: Uuid,
    pub moves: Vec<GameMove>,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub final_status: Option<GameStatus>,
}
```

**Game history concepts:**

- **Move Recording**: Tracking each move with timestamps
- **Game Reconstruction**: Ability to replay a game from history
- **Persistence**: Saving and loading game history
- **Metadata**: Tracking game start/end times and results

### Working with Dates and Times

We've used the Chrono crate for date and time handling:

```rust
pub struct GameMove {
    pub player: Player,
    pub row: usize,
    pub col: usize,
    pub timestamp: DateTime<Utc>,
}
```

**Chrono concepts:**

- **UTC Time**: Using coordinated universal time for consistency
- **DateTime**: Type-safe date and time representation
- **Serialization**: Integrates with Serde for date serialization
- **Current Time**: `Utc::now()` for getting the current time

### File I/O with Error Handling

We've implemented robust file I/O with proper error handling:

```rust
fn load_game(filename: &str) -> GameResult<GameState> {
    if !Path::new(filename).exists() {
        return Err(GameError::IoError(format!("File '{}' not found", filename)));
    }

    println!("Loading game from {}...", filename);
    let game = GameState::load_from_file(filename)?;
    println!("Game loaded successfully!");

    Ok(game)
}
```

**File I/O concepts:**

- **Path Handling**: Using the `Path` type for file paths
- **File Existence Checking**: Verifying files exist before reading
- **Error Propagation**: Using the `?` operator for clean error handling
- **User Feedback**: Providing clear messages about operations

## Commit 3: AI Opponent with Minimax

### Trait-Based Design

In our third commit of Phase 2, we've implemented a trait-based design for players:

```rust
pub trait GamePlayer {
    fn get_move(&self, game: &GameState) -> GameResult<(usize, usize)>;
    fn get_player_type(&self) -> Player;
    fn get_name(&self) -> String;
}
```

**Key concepts about traits:**

- **Interface Definition**: Traits define a set of methods that types must implement
- **Polymorphism**: Different types can implement the same trait
- **Dynamic Dispatch**: `Box<dyn GamePlayer>` allows runtime polymorphism
- **Trait Bounds**: Can constrain generic types to implement specific traits

### The Minimax Algorithm

We've implemented the minimax algorithm for our AI opponent:

```rust
fn minimax(&self, game: &GameState, depth: usize, max_depth: usize, is_maximizing: bool) -> i32 {
    // Base cases: terminal state or maximum depth reached
    if game.status != GameStatus::InProgress || depth == max_depth {
        return self.evaluate(game) - depth as i32; // Prefer shorter paths to victory
    }

    if is_maximizing {
        // Maximizing player (AI)
        let mut best_score = i32::MIN;
        // Try each empty cell...
        best_score
    } else {
        // Minimizing player (opponent)
        let mut best_score = i32::MAX;
        // Try each empty cell...
        best_score
    }
}
```

**Minimax algorithm concepts:**

- **Recursive Search**: Explores possible future game states
- **Alternating Minimizing/Maximizing**: Models two opposing players
- **Evaluation Function**: Assigns scores to game states
- **Depth-Limited Search**: Controls how far ahead the AI looks

### Dynamic Dispatch with Trait Objects

We've used trait objects for runtime polymorphism:

```rust
fn create_players(mode: GameMode) -> GameResult<(Box<dyn GamePlayer>, Box<dyn GamePlayer>)> {
    match mode {
        GameMode::HumanVsHuman => {
            let player1 = Box::new(HumanPlayer::new(Player::X, "Player 1".to_string()));
            let player2 = Box::new(HumanPlayer::new(Player::O, "Player 2".to_string()));
            Ok((player1, player2))
        },
        GameMode::HumanVsAIHard => {
            let player1 = Box::new(HumanPlayer::new(Player::X, "Player".to_string()));
            let player2 = Box::new(MinimaxAI::new(Player::O, Difficulty::Hard));
            Ok((player1, player2))
        },
        // Other modes...
    }
}
```

**Dynamic dispatch concepts:**

- **Trait Objects**: `Box<dyn Trait>` represents any type implementing the trait
- **Virtual Method Table**: Runtime lookup of method implementations
- **Heap Allocation**: `Box` allocates the object on the heap
- **Runtime Polymorphism**: Different implementations chosen at runtime

### Module Organization

We've organized our code into a more complex module structure:

```
src/
├── ai/
│   ├── mod.rs
│   └── minimax.rs
├── player/
│   ├── mod.rs
│   ├── player_type.rs
│   └── human_player.rs
├── game.rs
├── error.rs
├── history.rs
└── lib.rs
```

**Module organization concepts:**

- **Hierarchical Modules**: Nested modules for better organization
- **Re-exports**: Using `pub use` to simplify imports
- **Visibility Control**: Controlling what's public vs. private
- **Module Files**: Using `mod.rs` for directory modules

### Difficulty Levels and Strategy

We've implemented different difficulty levels for the AI:

```rust
pub enum Difficulty {
    Easy,   // Makes random valid moves
    Medium, // Uses minimax but with limited depth
    Hard,   // Uses full minimax algorithm
}

fn get_max_depth(&self) -> usize {
    match self.difficulty {
        Difficulty::Easy => 1,
        Difficulty::Medium => 3,
        Difficulty::Hard => 9, // Full search for tic-tac-toe
    }
}
```

**Game AI concepts:**

- **Difficulty Scaling**: Different levels of challenge
- **Search Depth**: Controlling how far ahead the AI looks
- **Randomness**: Adding unpredictability at lower difficulties
- **Perfect Play**: Unbeatable AI at the highest difficulty

### Next Steps

In our next commit, we'll implement a simple HTTP server using async Rust.
