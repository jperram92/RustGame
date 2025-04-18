# Phase 1: Foundation Setup - A Python/TypeScript Developer's Guide to Rust

## 1. Welcome to Rust! 
Coming from Python/TypeScript, you'll notice some differences. Don't worry - we'll explain everything!

### 1.1 Key Differences from Python/TypeScript

#### Types are Explicit (and Important!)
In Python, you might write:
```python
# Python
player = "X"  # Type is implicit
game_board = [[None] * 3 for _ in range(3)]  # Flexible types
```

In TypeScript:
```typescript
// TypeScript
type Player = "X" | "O";
let player: Player = "X";
let gameBoard: (Player | null)[][] = Array(3).fill(null).map(() => Array(3).fill(null));
```

In Rust, we're more explicit and strict:
```rust
// Rust
pub enum Player { // Like a stricter version of TypeScript's union types
    X,  // These are "variants", not strings!
    O,
}

// This is like a TypeScript type with only two possible values
pub enum Cell {
    Empty,
    Occupied(Player)  // Can hold a Player value (like a tuple)
}

// Fixed-size array with exact type - no null or undefined!
pub board: [[Cell; 3]; 3]
```

#### Memory Management: The Big Difference
In Python/TypeScript, you don't think about memory much:
```python
# Python - memory is managed for you
def update_game(game):
    game.board[0][0] = "X"  # Just works!
    return game
```

In Rust, you need to be explicit about ownership:
```rust
// Rust - you need to specify if you're borrowing or taking ownership
pub fn update_game(game: &mut GameState) {  // &mut means "I want to borrow and modify"
    game.board[0][0] = Cell::Occupied(Player::X);
}
```

### 1.2 Understanding Rust's Special Features

#### Enums (They're Not Like Python Enums!)
```rust
// This is like a super-powered version of TypeScript's union types
pub enum GameStatus {
    InProgress,           // Just a variant
    Won(Player),         // Variant with data (like a tuple)
    Draw,                // Another variant
}
```

Why this is cool:
- No more string comparisons like `if status === "in_progress"`
- Can't have invalid states (unlike Python where any string could be used)
- Can hold data (unlike TypeScript's union types)

#### Traits (Like TypeScript Interfaces, but More Powerful)
When you see this:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
```
Think of it like automatically implementing interfaces in TypeScript:
```typescript
// TypeScript equivalent (sort of)
interface Debuggable {
    toString(): string;
}
interface Cloneable {
    clone(): this;
}
```

#### Result Type (Better Than Try/Catch)
Instead of:
```python
# Python
try:
    make_move(x, y)
except InvalidMoveError as e:
    print(f"Error: {e}")
```

Rust uses:
```rust
match game.make_move(x, y) {
    Ok(()) => println!("Move successful!"),
    Err(e) => println!("Error: {}", e),
}
```

### 1.3 The Game State Structure

Here's our main game structure:
```rust
pub struct GameState {
    pub id: Uuid,                // Like a unique string ID
    pub board: [[Cell; 3]; 3],   // 3x3 grid of Cell enums
    pub current_turn: Player,    // Whose turn is it?
    pub status: GameStatus,      // Current game status
}
```

Think of this like a TypeScript interface:
```typescript
interface GameState {
    id: string;
    board: Array<Array<Cell>>;
    currentTurn: Player;
    status: GameStatus;
}
```

But Rust's version:
- Can't have null/undefined (more reliable!)
- Has fixed-size arrays (better performance!)
- Guarantees type safety (no runtime surprises!)

### 1.4 Error Handling (No More Try/Catch!)

Instead of throwing errors like in Python/TypeScript:
```rust
pub enum GameError {
    #[error("Invalid move: Cell at position ({0}, {1}) is already occupied")]
    CellOccupied(usize, usize),  // usize is like a positive number
    
    #[error("Invalid position: ({0}, {1}) is out of bounds")]
    InvalidPosition(usize, usize),
    
    #[error("Game is already finished")]
    GameAlreadyFinished,
    
    #[error("Not player's turn")]
    NotPlayerTurn,
}
```

This is like having a custom error class, but:
- More structured than Python's exceptions
- More powerful than TypeScript's union types
- Compiler ensures you handle all error cases!

## 2. Common Patterns You'll Use

### 2.1 Creating New Instances
Instead of a constructor:
```python
# Python
class GameState:
    def __init__(self):
        self.board = [[None] * 3 for _ in range(3)]
```

In Rust we use the `new()` convention:
```rust
impl GameState {
    pub fn new() -> Self {  // -> Self means "returns a GameState"
        Self {
            id: Uuid::new_v4(),  // Generate a unique ID
            board: [[Cell::Empty; 3]; 3],  // Create empty board
            current_turn: Player::X,       // X goes first
            status: GameStatus::InProgress,
        }
    }
}
```

### 2.2 Pattern Matching (Better Than Switch/Match)
```rust
match game.status {
    GameStatus::InProgress => println!("Game is still going!"),
    GameStatus::Won(player) => println!("Player {:?} won!", player),
    GameStatus::Draw => println!("It's a draw!"),
}
```
This is like switch/case but:
- Must handle all cases (compiler enforced!)
- Can extract data from variants
- No fall-through bugs!

## Next Steps

Now that you understand the basics, would you like to:
1. See how to implement game moves?
2. Learn more about any specific concept?
3. See how testing works in Rust?

Remember: Rust might feel strict at first, but these restrictions prevent bugs that plague Python/TypeScript code in production!