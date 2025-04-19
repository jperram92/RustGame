//! Tic-tac-toe game library
//!
//! This library provides the core functionality for a tic-tac-toe game,
//! including game state management, move validation, and win condition checking.
//! It also supports game history tracking, serialization, AI opponents, and a REST API.

pub mod game;
pub mod error;
pub mod player;
pub mod history;
pub mod ai;
pub mod server;
