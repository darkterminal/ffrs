//! Grammar parsing module for the FF CLI tool.
//!
//! This module provides tokenization and parsing functionality to convert
//! plain English commands into structured intents.

pub mod parser;
pub mod tokenizer;

pub use parser::*;
pub use tokenizer::*;
