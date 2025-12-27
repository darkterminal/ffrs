//! # FF - Media Conversion CLI Tool
//!
//! FF is a CLI tool that translates plain English commands into ffmpeg commands.
//! It provides both a library API and a command-line interface for media conversion tasks.
//!
//! ## Features
//!
//! - Translate plain English commands to ffmpeg commands
//! - Support for multiple media formats
//! - Interactive and direct command modes
//! - Dry-run functionality
//! - Deterministic and inspectable command generation

pub mod grammar;
pub mod intent;
pub mod command_builder;
pub mod executor;
pub mod utils;

pub use grammar::{Tokenizer, Parser};
pub use intent::types::{Intent, OperationType};
pub use command_builder::CommandBuilder;
pub use executor::runner::Runner;
pub use utils::file_utils;