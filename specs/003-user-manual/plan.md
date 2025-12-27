## Implementation Plan: User Manual for FF CLI Tool

**Branch**: `user-manual` | **Date**: 2025-12-28 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification for user manual targeting non-technical users

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

### Summary

Create a comprehensive user manual (`docs/USER_MANUAL.md`) that explains the FF CLI tool in simple, non-technical language. The manual will cover installation, basic usage, common tasks, troubleshooting, and examples that help non-technical users understand and use the tool effectively.

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
**Documentation Target**: Non-technical users
**Documentation Format**: Markdown file in docs/ directory
**Documentation Style**: Plain English, step-by-step instructions, visual examples

### Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

Based on the constitution:
- ✅ CLI-First, No GUI Assumptions: Manual explains CLI usage appropriately
- ✅ Plain English Is the Interface: Manual uses simple language for non-technical users
- ✅ Deterministic and Inspectable: Manual explains how commands are translated to ffmpeg
- ✅ Offline, Zero State: Manual covers offline operation
- ✅ Fail Fast, Fail Loud: Manual includes troubleshooting section for common errors
- ✅ Technology Stack Requirements: Manual doesn't affect stack
- ✅ Binary Constraints: Manual doesn't affect binary
- ✅ Dependency Policy: Manual doesn't add dependencies
- ✅ Platform Support: Manual covers all platforms (Linux, macOS, Windows)
- ✅ Grammar-Driven Development: Manual explains the grammar approach
- ✅ Intent-Centric Architecture: Manual doesn't expose internal architecture
- ✅ Testing Discipline: Manual doesn't affect testing
- ✅ CLI UX Contract: Manual covers both interactive and direct modes
- ✅ Build & Release Policy: Manual doesn't affect build process
- ✅ Cross-Compilation: Manual covers cross-platform usage

### Project Structure

#### Documentation (this feature)

```text
specs/003-user-manual/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

#### Source Code (repository root)

```text
docs/
└── USER_MANUAL.md       # User manual for non-technical users
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

**Structure Decision**: Single user manual document in the docs/ directory. The manual will be written in Markdown format for easy reading and editing. It will be structured with clear sections for different aspects of using the tool, with examples that non-technical users can follow easily.

### Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| [e.g., 4th project] | [current need] | [why 3 projects insufficient] |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient] |