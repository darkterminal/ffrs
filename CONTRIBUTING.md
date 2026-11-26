# Contributing to ffrs

Thanks for helping improve `ffrs`. This guide describes the setup and the workflow used in this repository.

## Setup

```bash
git clone https://github.com/darkterminal/ffrs.git
cd ffrs
cargo build --locked
```

Install Rust 1.85 or newer. That is the declared minimum supported version (MSRV) in `Cargo.toml`, and CI verifies it with a 1.85.0 job. Actual conversion needs `ffmpeg` in `PATH`, but the test suite does not execute ffmpeg and does not need it installed.

To run the CLI locally:

```bash
cargo run -- --dry-run "convert video.mp4 to video.avi"
```

## Verification

Run the same checks as CI before opening a pull request:

```bash
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
cargo build --locked --all-targets
cargo test --locked --all-targets
cargo test --locked --doc
```

Tests must not depend on a real ffmpeg installation or on release assets. Use `--dry-run`, tokenizer/parser tests, and temporary files.

If the 1.85.0 toolchain is installed locally, also run `cargo +1.85.0 check --locked --all-targets` to reproduce the MSRV job.

## Commit messages

Use [Conventional Commits](https://www.conventionalcommits.org/): `type(scope): summary`. Common types in this repository are `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `build`, and `ci`.

Examples:

```text
fix(grammar): recognize relative, quoted, and Windows paths
test: make all integration tests discoverable
ci: modernize workflow and add quality gates
```

Keep the summary imperative and lower-case, and put the rationale in the commit body when it is not obvious.

## Issue and pull request flow

1. Open an issue first and describe the problem or proposal. One work item gets one issue and one pull request.
2. Create a focused branch from `main`, for example `fix/path-tokenization` or `docs/manual-accuracy`.
3. Make the smallest change that solves the issue. Avoid unrelated refactors and formatting whole directories.
4. Add or update tests, then run the verification commands above.
5. Update the `[Unreleased]` section of `CHANGELOG.md`.
6. Push the branch and open a pull request that references the issue (`Closes #N`).
7. Address review feedback. A pull request is squash-merged only after checks pass and the review is complete.

Do not bump the version, create tags, or publish releases as part of a normal maintenance pull request. Release decisions are separate from code changes.

## Grammar changes

The plain-English grammar is implemented in `src/grammar/tokenizer.rs` and `src/grammar/parser.rs`. When changing it:

- Preserve existing command forms unless the issue explicitly asks to break them.
- Add unit tests in the grammar modules and, when CLI behavior changes, an integration test under `tests/`.
- Keep integration tests as top-level files in `tests/`; Cargo does not discover nested test directories.
- Document any new user-facing syntax in `README.md` and `docs/USER_MANUAL.md`.
- Never let the tokenizer loop without advancing, and validate output paths before reaching ffmpeg.

## Internal notes

Planning and tracking notes live in `.internal/`, which is excluded through `.git/info/exclude` rather than `.gitignore`. They are not part of pull requests. Items that need a human decision are recorded in `.internal/NEEDS-CONFIRMATION.md` and are not implemented until explicitly requested.
