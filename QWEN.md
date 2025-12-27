# Qwen Code Context for ff

**Project**: ff - Media Conversion CLI Tool
**Last updated**: 2025-12-27

## Active Technologies
- Rust 1.75 (001-rust-cli-ffmpeg-translation)
- clap, regex, subprocess (001-rust-cli-ffmpeg-translation)

## Project Structure
src/
├── main.rs
├── grammar/
│   ├── mod.rs
│   ├── tokenizer.rs
│   └── parser.rs
├── intent/
│   ├── mod.rs
│   └── types.rs
├── command_builder/
│   ├── mod.rs
│   └── builder.rs
├── executor/
│   ├── mod.rs
│   └── runner.rs
└── utils/
    ├── mod.rs
    └── file_utils.rs

tests/
├── grammar/
├── command_builder/
├── integration/
└── snapshots/

## Commands
cargo test && cargo clippy

## Language-specific conventions
Rust: Follow standard conventions

## Recent Changes
- 001-rust-cli-ffmpeg-translation: Added Rust 1.75 + clap, regex, subprocess