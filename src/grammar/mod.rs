//! Grammar parsing module for the FF CLI tool.
//!
//! This module provides tokenization and parsing functionality to convert
//! plain English commands into structured intents.

pub mod tokenizer;
pub mod parser;

pub use tokenizer::*;
pub use parser::*;