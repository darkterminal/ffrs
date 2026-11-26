# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Changelog tracking for upcoming changes.
- `--version` flag, which reports the `ffrs` crate version.
- `ffrs` identity for clap so `--help` and usage output match the binary name.
- Documented `extract audio from <input> to <output>` syntax, plus shorter `extract audio <input> ...` and `extract <input> ...` forms.
- `--yes` flag to overwrite an existing output file; the default never overwrites.
- Grammar parameters for `resize` (`<width>x<height>`) and `transcode` (codec), with fail-fast validation.
- Tag-triggered release workflow that builds Linux, macOS, and Windows binaries and attaches them to the GitHub release.

### Changed

- `CommandBuilder` now returns a concrete `BuildError` instead of `Box<dyn Error>`; the error is exported and converts into `FfError`.
- User-facing guidance now lives on `FfError` instead of being derived from message text in the CLI.
- Declared Rust 1.85 as the minimum supported version and added an MSRV CI job.
- CI now audits dependencies for advisories on pull requests and pushes to `main`.
- Output paths without a supported media extension now fail fast instead of reaching ffmpeg.
- CLI usage errors now exit with code `1`; `--help` and `--version` still exit with `0`.
- The binary now consumes the library crate instead of compiling a second copy of its modules.
- CI now enforces formatting, clippy, and rustdoc, with locked builds and current action versions.
- `Cargo.lock` is now tracked, and non-crate files are excluded from package archives.
- Rustfmt configuration now matches the crate's 2021 edition, and all sources are formatted.
- Generated ffmpeg commands now include an explicit overwrite flag (`-n` by default, `-y` with `--yes`).
- README, user manual, and Qwen context now describe the implemented CLI; stale release-asset instructions were removed.
- Contributor guide, issue/PR templates, security policy, and Dependabot configuration are now included.

### Fixed

- Reject output paths that resolve to the input file, including through `--output`.
- Validate the output format from the file extension, so directories containing dots are accepted.
- Require `--output` to be an existing directory and report a clear error when it is missing.
- Interactive mode now exits on EOF instead of looping forever printing prompts.
- Interactive mode reports I/O errors instead of panicking when stdout cannot be written.
- Tokenizer recognizes relative, quoted, and Windows paths instead of rejecting them.
- Tokenizer always advances past unexpected characters instead of looping forever.
- Integration tests nested under `tests/` are now discovered and compiled by Cargo.

### Removed

- Unused `regex` dependency.
- Stale `specs/`, `.specify/`, and `.qwen/` scaffolding; the content remains in git history.
