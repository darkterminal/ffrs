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

### Changed

- Output paths without a supported media extension now fail fast instead of reaching ffmpeg.
- CLI usage errors now exit with code `1`; `--help` and `--version` still exit with `0`.
- `Cargo.lock` is now tracked, and non-crate files are excluded from package archives.

### Fixed

- Reject output paths that resolve to the input file, including through `--output`.
- Validate the output format from the file extension, so directories containing dots are accepted.
- Require `--output` to be an existing directory and report a clear error when it is missing.
- Interactive mode now exits on EOF instead of looping forever printing prompts.
- Interactive mode reports I/O errors instead of panicking when stdout cannot be written.
- Tokenizer recognizes relative, quoted, and Windows paths instead of rejecting them.
- Tokenizer always advances past unexpected characters instead of looping forever.

### Removed

- Unused `regex` dependency.
