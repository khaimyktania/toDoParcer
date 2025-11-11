
/// Library module for To-Do list parsing.
/// 
/// Contains the main parser and related data structures.
/// Crate entry for **to_do_parcer** — a parser and CLI for a lightweight
pub mod parser;

/// Re-exports core types and parser for easy access.
pub use parser::{ParseError, Priority, Project, Task, TaskStatus, ToDoParser};
