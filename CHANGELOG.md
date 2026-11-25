# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Changelog tracking for upcoming changes.

### Changed

- Output paths without a supported media extension now fail fast instead of reaching ffmpeg.

### Fixed

- Reject output paths that resolve to the input file, including through `--output`.
- Validate the output format from the file extension, so directories containing dots are accepted.
- Require `--output` to be an existing directory and report a clear error when it is missing.

### Removed
