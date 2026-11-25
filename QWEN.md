# Qwen Code Context for ffrs

**Project**: ffrs — Media Conversion CLI Tool  
**Last updated**: 2026-11-26

## Active Technologies

- Rust stable, edition 2021
- clap 4 (derive feature) for CLI parsing
- std::process::Command for ffmpeg execution
- tempfile (dev dependency) for test fixtures

## Project Structure

```text
src/
├── lib.rs
├── main.rs
├── errors.rs
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

tests/  (top-level Cargo integration test binaries)
├── cli_contract.rs
├── cli_output.rs
├── command_generation.rs
├── deterministic_behavior.rs
├── extract_audio.rs
├── grammar.rs
├── path_tokenization.rs
├── quickstart_validation.rs
└── workflow.rs
```

Nested directories under `tests/` are not discovered by Cargo; keep integration tests at the top level. The binary in `src/main.rs` consumes the `ffrs` library crate and must not re-declare its modules.

## Commands

```bash
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
cargo build --locked --all-targets
cargo test --locked --all-targets
cargo test --locked --doc
```

Tests must not require ffmpeg. Use `--dry-run`, tokenizer/parser tests, and temporary files instead.

## Conventions

- Conventional Commits for commit messages.
- `CHANGELOG.md` has only an `[Unreleased]` section during maintenance; no version bump, tag, or GitHub release.
- Every maintenance item has one issue and one pull request; review before squash merge.
- Internal planning notes live in `.internal/`, excluded via `.git/info/exclude`.

## CI

CI is defined in `.github/workflows/ci.yml`: a lint job (format, clippy, rustdoc) and a locked build/test matrix on Ubuntu, macOS, and Windows. There is no release workflow and no published release assets; install from crates.io or source.
