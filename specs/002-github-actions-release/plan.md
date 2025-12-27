## Implementation Plan: GitHub Actions for Cross-Platform Binary Releases

**Branch**: `github-actions-release` | **Date**: 2025-12-28 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification for cross-platform binary releases via GitHub Actions

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

### Summary

Implement GitHub Actions workflow to automatically build and release cross-platform binaries for the Rust CLI tool on tag creation. The workflow will build binaries for Linux, macOS, and Windows, package them appropriately, and create GitHub releases with the binaries attached.

### Technical Context

**Language/Version**: Rust 1.75
**Primary Dependencies**: clap (CLI parsing), regex (pattern matching), subprocess (command execution)
**Storage**: N/A (no persistent storage, operates on files)
**Testing**: cargo test
**Target Platform**: Linux, macOS, Windows
**Project Type**: Single CLI application
**Performance Goals**: Under 1 second startup time and 10% slower than direct ffmpeg execution
**Constraints**: Cross-platform compatibility, single static binary, offline operation, zero configuration
**Scale/Scope**: Individual user media conversion tasks
**CI/CD Tool**: GitHub Actions
**Release Process**: Automated on tag creation
**Build Targets**: x86_64-unknown-linux-musl, x86_64-apple-darwin, x86_64-pc-windows-msvc

### Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

Based on the constitution:
- ✅ CLI-First, No GUI Assumptions: This is a CLI tool with no GUI
- ✅ Plain English Is the Interface: Tool translates plain English to ffmpeg commands
- ✅ Deterministic and Inspectable: Always prints the ffmpeg command before execution
- ✅ Offline, Zero State: No persistent state or configuration files
- ✅ Fail Fast, Fail Loud: Will exit with non-zero code on invalid input
- ✅ Technology Stack Requirements: Using Rust as specified
- ✅ Binary Constraints: Single binary output with no dynamic Rust dependencies
- ✅ Dependency Policy: Using lightweight crates as specified (clap, regex, subprocess)
- ✅ Platform Support: Supporting Linux, macOS, Windows as specified
- ✅ Grammar-Driven Development: Following tokenizer → parser → intent → command builder → executor flow
- ✅ Intent-Centric Architecture: Following the required architecture flow
- ✅ Testing Discipline: Unit tests for parsing, snapshot tests for command generation
- ✅ CLI UX Contract: Supporting both interactive and direct modes, proper output rules, correct exit codes
- ✅ Build & Release Policy: Using Rust cross-compilation for single binaries per platform
- ✅ Cross-Compilation: Using GitHub Actions for cross-compilation to create binaries for all platforms

### Project Structure

#### Documentation (this feature)

```text
specs/002-github-actions-release/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

#### Source Code (repository root)

```text
.github/
└── workflows/
    └── release.yml      # GitHub Actions workflow for releases
src/
├── main.rs              # CLI entry point and command parsing
├── grammar/             # Grammar rules and parsing logic
│   ├── mod.rs
│   ├── tokenizer.rs     # Tokenizes English commands
│   └── parser.rs        # Parses tokens into intent structs
├── intent/              # Intent struct definitions
│   ├── mod.rs
│   └── types.rs         # Various intent types (convert, resize, etc.)
├── command_builder/     # Converts intents to ffmpeg commands
│   ├── mod.rs
│   └── builder.rs
├── executor/            # Executes ffmpeg commands
│   ├── mod.rs
│   └── runner.rs
└── utils/               # Utility functions
    ├── mod.rs
    └── file_utils.rs    # File handling utilities

tests/
├── grammar/             # Grammar parsing tests
├── command_builder/     # Command generation tests
├── integration/         # Integration tests
└── snapshots/           # Snapshot tests for command generation
```

**Structure Decision**: Single project structure selected with modular organization by functionality. The code is organized into modules for grammar parsing, intent representation, command building, and execution, with appropriate submodules for each concern. The GitHub Actions workflow will be added to the `.github/workflows/` directory to handle cross-platform binary releases.

### Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |