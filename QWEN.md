# Qwen Code Context for ff

**Project**: ff - Media Conversion CLI Tool
**Last updated**: 2025-12-28

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
- 002-github-actions-release: Added GitHub Actions workflow for cross-platform binary releases
- 003-user-manual: Added comprehensive user manual for non-technical users

## GitHub Actions Release Implementation

### Summary
- Implemented GitHub Actions workflow for cross-platform binary releases
- Workflow automatically builds binaries for Linux, macOS, and Windows on tag creation
- Binaries are packaged and attached to GitHub releases automatically

### Technical Details
- Workflow file: `.github/workflows/release.yml`
- Build targets: x86_64-unknown-linux-musl, x86_64-apple-darwin, x86_64-pc-windows-msvc
- Uses `cross` crate for reliable cross-compilation
- Binaries are packaged with platform-specific names

### Files Added
- `.github/workflows/release.yml` - GitHub Actions workflow
- `specs/002-github-actions-release/` - Implementation plan documentation
  - `plan.md` - Implementation plan
  - `research.md` - Research findings
  - `data-model.md` - Data model
  - `quickstart.md` - Quickstart guide
  - `contracts/` - API contracts (empty directory)
  - `tasks.md` - Implementation tasks

### Compliance
- Follows all constitution requirements for CLI tools
- Maintains cross-platform compatibility
- Produces static binaries with no external dependencies
- Automates the release process

## User Manual Implementation

### Summary
- Created comprehensive user manual for non-technical users
- Manual explains installation, usage, and troubleshooting in simple language
- Covers all supported platforms (Windows, Mac, Linux)

### Technical Details
- Manual file: `docs/USER_MANUAL.md`
- Format: Markdown for easy reading and editing
- Target audience: Non-technical users

### Files Added
- `docs/USER_MANUAL.md` - User manual for non-technical users
- `specs/003-user-manual/` - Implementation plan documentation
  - `plan.md` - Implementation plan
  - `research.md` - Research findings
  - `data-model.md` - Data model
  - `quickstart.md` - Quickstart guide
  - `contracts/` - API contracts (empty directory)
  - `tasks.md` - Implementation tasks

### Compliance
- Follows all constitution requirements for CLI tools
- Maintains focus on plain English interface
- Provides accessible documentation for all users
- Supports cross-platform usage