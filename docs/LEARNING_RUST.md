# Learning Rust: A Developer's Journal

## Commit 1: Basic Game Structure

### Enums in Rust

In our first commit, we introduced several enums:

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    X,
    O,
}
```

**What's special about Rust enums?**
- Unlike enums in many languages, Rust enums are full-fledged algebraic data types
- They can contain data (like `Occupied(Player)` in our `Cell` enum)
- They're type-safe - the compiler ensures you handle all possible variants
- They work beautifully with pattern matching

**The `#[derive(...)]` attribute:**
- This is a form of metaprogramming in Rust
- It automatically implements traits for our types
- `Debug`: Allows printing with `{:?}` format specifier
- `Clone`, `Copy`: Enables copying the value instead of moving it
- `PartialEq`: Allows comparison with `==`

### Structs and Initialization

We defined our game state with a struct:

```rust
pub struct GameState {
    pub id: Uuid,
    pub board: [[Cell; 3]; 3],
    pub current_turn: Player,
    pub status: GameStatus,
}
```

**Key points about Rust structs:**
- Fields are private by default (we made them public with `pub`)
- No inheritance like in OOP languages
- No null values - every field must be initialized
- The `[[Cell; 3]; 3]` syntax creates a fixed-size 2D array (3×3 grid)

### Methods and Self

We implemented a method for our `GameState`:

```rust
impl GameState {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            board: [[Cell::Empty; 3]; 3],
            current_turn: Player::X,
            status: GameStatus::InProgress,
        }
    }
}
```

**What's happening here:**
- `impl` blocks are where we define methods for our types
- `Self` is a shorthand for the type name (`GameState` in this case)
- `new()` is a convention for constructors (not a language feature)
- The `->` syntax indicates the return type

### Pattern Matching

We used pattern matching in the `opponent` method:

```rust
pub fn opponent(&self) -> Self {
    match self {
        Player::X => Player::O,
        Player::O => Player::X,
    }
}
```

**Pattern matching in Rust:**
- The `match` expression is like a switch statement on steroids
- It's exhaustive - you must handle all possible cases
- It can destructure complex data
- The compiler ensures you don't miss any cases

### External Dependencies

We added our first external dependency:

```toml
[dependencies]
uuid = { version = "1.4", features = ["v4"] }
```

**Cargo.toml explained:**
- Similar to package.json in Node.js or requirements.txt in Python
- Specifies dependencies with semantic versioning
- Can enable specific features of a crate (Rust packages are called "crates")
- The `v4` feature enables UUID generation

## Commit 2: Game Logic

### Error Handling with Result

In our second commit, we implemented error handling using Rust's `Result` type:

```rust
pub fn make_move(&mut self, row: usize, col: usize) -> Result<(), GameError> {
    // Implementation...
}
```

**Key points about Rust's error handling:**
- No exceptions like in Python or JavaScript
- `Result<T, E>` is an enum with two variants: `Ok(T)` and `Err(E)`
- Forces you to handle errors explicitly
- Works beautifully with pattern matching

### Custom Error Types

We created a custom error type for our game:

```rust
pub enum GameError {
    CellOccupied(usize, usize),
    InvalidPosition(usize, usize),
    GameAlreadyFinished,
}
```

**Benefits of custom error types:**
- More specific than generic errors
- Can include context (like coordinates)
- Makes error handling more expressive
- Compiler ensures you handle all error cases

### Implementing Traits

We implemented the `Display` trait for our error type:

```rust
impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameError::CellOccupied(row, col) => {
                write!(f, "Cell at position ({}, {}) is already occupied", row, col)
            }
            // Other cases...
        }
    }
}
```

**What's happening here:**
- Traits are Rust's version of interfaces
- `Display` is a standard trait for formatting values as strings
- Implementing it allows using our type with `println!("{}", error)`
- The `write!` macro is similar to `format!` but writes to a formatter

### Pattern Matching with Guards

We used advanced pattern matching to check win conditions:

```rust
if (0..3).all(|col| matches!(self.board[last_row][col], Cell::Occupied(p) if p == player)) {
    self.status = GameStatus::Won(player);
    return;
}
```

**Advanced pattern matching features:**
- `matches!` macro tests if a value matches a pattern
- Pattern guards (`if p == player`) add extra conditions
- Combined with iterators like `all()` for powerful expressions

### Mutable References

We used mutable references to modify the game state:

```rust
pub fn make_move(&mut self, row: usize, col: usize) -> Result<(), GameError> {
    // Implementation...
}
```

**Understanding mutable references:**
- `&mut self` means "I need to modify this object"
- Only one mutable reference can exist at a time (prevents data races)
- The compiler enforces these rules at compile time
- No need for locks or synchronization in single-threaded code

### Iterators and Functional Programming

We used iterators and functional programming concepts:

```rust
if self.board.iter().all(|row| row.iter().all(|cell| !matches!(cell, Cell::Empty))) {
    self.status = GameStatus::Draw;
    return;
}
```

**Functional programming in Rust:**
- Iterators are lazy and efficient
- Methods like `all()`, `any()`, `map()`, `filter()` work like in Python
- Closures (`|x| expression`) are like lambda functions
- No garbage collection overhead

## Commit 3: Interactive CLI Interface

### User Input and Standard I/O

In our third commit, we implemented a command-line interface for our game:

```rust
fn get_player_move() -> (usize, usize) {
    loop {
        print!("Enter your move as 'row col' (0-2): ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        // Parse the input...
    }
}
```

**Working with I/O in Rust:**
- `io::stdin()` and `io::stdout()` provide access to standard input/output
- `print!` doesn't automatically flush output (unlike `println!`)
- `read_line` takes a mutable reference to a string and appends input to it
- The `expect` method is a shorthand for handling `Result` when you just want to panic on error

### String Manipulation

We parsed user input using string manipulation:

```rust
let coords: Vec<&str> = input.trim().split_whitespace().collect();
```

**String operations in Rust:**
- Strings are UTF-8 encoded and more complex than in other languages
- `trim()` removes whitespace from both ends (like Python's `strip()`)
- `split_whitespace()` returns an iterator over substrings
- `collect()` transforms an iterator into a collection (here, a `Vec<&str>`)

### Result Handling with Pattern Matching

We used pattern matching to handle multiple `Result` values at once:

```rust
match (coords[0].parse::<usize>(), coords[1].parse::<usize>()) {
    (Ok(row), Ok(col)) => {
        if row <= 2 && col <= 2 {
            return (row, col);
        } else {
            println!("Row and column must be between 0 and 2.");
        }
    }
    _ => println!("Invalid input. Please enter numbers."),
}
```

**Advanced pattern matching:**
- Tuples of `Result` values can be matched together
- The `_` pattern is a catch-all for any other case
- Type annotations (`::<usize>`) tell the compiler what type to parse into

### Loop Control Flow

We used different types of loops for different purposes:

```rust
// Infinite loop with explicit returns
loop {
    // Get input and validate...
    return (row, col); // Exit when valid input is received
}

// Main game loop with conditional breaks
loop {
    // Game logic...
    match game.status {
        GameStatus::Won(_) | GameStatus::Draw => break,
        _ => continue,
    }
}
```

**Loops in Rust:**
- `loop` creates an infinite loop (clearer intent than `while true`)
- `break` exits the loop
- `continue` skips to the next iteration
- Loops can return values: `let x = loop { break 42; };`

## Commit 4: Testing and Documentation

### Documentation Comments

In our fourth commit, we added documentation comments to our code:

```rust
/// Represents a player in the game (X or O)
pub enum Player {
    /// The X player (usually goes first)
    X,
    /// The O player
    O,
}
```

**Documentation in Rust:**
- `///` creates documentation comments (similar to JavaDoc or JSDoc)
- Documentation can include Markdown formatting
- Documentation is compiled into HTML when you run `cargo doc`
- Code examples in documentation can be tested with `cargo test`

### Unit Testing

We added unit tests for our game logic:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game() {
        let game = GameState::new();
        assert_eq!(game.current_turn, Player::X);
        // More assertions...
    }

    // More tests...
}
```

**Testing in Rust:**
- Tests are typically in a module annotated with `#[cfg(test)]`
- Individual test functions are annotated with `#[test]`
- `use super::*;` imports all items from the parent module
- Tests are run with `cargo test`
- Failed assertions show detailed error messages

### Conditional Compilation

We used conditional compilation for tests:

```rust
#[cfg(test)]
mod tests {
    // Test code here...
}
```

**Conditional compilation in Rust:**
- `#[cfg(test)]` means "only compile this when testing"
- Test code doesn't bloat your production binary
- You can use `#[cfg(...)]` for other conditions too (e.g., platform-specific code)

### Unwrap and Expect

We used `unwrap()` and `expect()` in our tests:

```rust
game.make_move(0, 0).unwrap(); // Panics if the Result is an Err
io::stdout().flush().expect("Failed to flush stdout");
```

**Unwrap and Expect:**
- `unwrap()` extracts the value from `Ok` or panics if it's an `Err`
- `expect()` is like `unwrap()` but with a custom error message
- Useful for tests and quick prototyping
- Generally avoided in production code in favor of proper error handling

### Assertions

We used various assertion macros in our tests:

```rust
assert_eq!(game.current_turn, Player::X);
assert!(game.make_move(0, 0).is_ok());
```

**Assertion macros in Rust:**
- `assert!` checks if a boolean expression is true
- `assert_eq!` checks if two expressions are equal
- `assert_ne!` checks if two expressions are not equal
- Failed assertions show the values being compared

### Pattern Matching in Tests

We used pattern matching to test error types:

```rust
match result {
    Err(GameError::GameAlreadyFinished) => {}, // Expected
    _ => panic!("Expected GameAlreadyFinished error"),
}
```

**Pattern matching in tests:**
- Can verify that specific error variants are returned
- `panic!` macro causes a test to fail with a custom message
- Underscore (`_`) pattern catches all other cases

## Project Complete!

Congratulations! We've built a complete tic-tac-toe game in Rust, covering:

1. **Basic Types and Structures**
   - Enums, structs, and methods
   - Ownership and borrowing

2. **Game Logic**
   - Error handling with Result
   - Pattern matching
   - Functional programming with iterators

3. **User Interface**
   - Command-line I/O
   - String parsing
   - Loop control flow

4. **Testing and Documentation**
   - Unit tests
   - Documentation comments
   - Conditional compilation

This project has introduced you to many of Rust's core concepts. As you continue your Rust journey, you might want to explore:

- **Modules and Crates**: Organizing larger projects
- **Traits**: More advanced interfaces and generics
- **Concurrency**: Rust's fearless concurrency with threads and async/await
- **Error Handling**: Using the `?` operator and custom error types
- **Advanced Pattern Matching**: Destructuring complex data

Happy coding in Rust!
