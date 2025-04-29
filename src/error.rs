// src/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Rekt {
    #[error("Lexer error: {0}")]
    Lexer(String),
    
    #[error("Parser error: {0}")]
    Parser(String),
    
    #[error("Runtime error: {0}")]
    Runtime(String),

    #[error("Type error: {0}")]
    Type(String),  // Changed from Syntax to Type to match usage
}