<!--
SYNC IMPACT REPORT
Version change: 1.1.0 → 1.2.0
Modified principles: Core Principles, Technology Stack Requirements, Development Workflow
Added sections: CLI UX Contract, Build & Release Policy
Removed sections: N/A
Templates requiring updates: ❌ none
Follow-up TODOs: None
-->

# ff Constitution

## Core Principles

### CLI-First, No GUI Assumptions
ff is a command-line tool.
All design decisions prioritize terminal usage, scripting, pipes, and automation.

No GUI hooks. No background services.

### Plain English Is the Interface
The CLI accepts human-readable commands, not flags.

Flags exist only for:

* execution control (`--dry-run`, `--yes`)
* debugging

Business intent must never be expressed via flags.

### Deterministic and Inspectable
Every CLI invocation must:

* deterministically map to a single ffmpeg command
* print that command before execution
* allow execution to be skipped

The CLI never hides work.

### Offline, Zero State
The CLI must:

* work without internet
* store no state
* create no config files
* respect the current working directory only

Stateless by design.

### Fail Fast, Fail Loud
Invalid grammar, ambiguity, or unsupported intent must:

* exit non-zero
* explain the failure in one sentence
* suggest a valid example if possible

Silent behavior is forbidden.

## Technology Stack Requirements

### Language and Runtime

* Rust (stable)
* No nightly
* No runtime VM
* No scripting engines

Compile-time safety over runtime flexibility.

### Binary Constraints

* Single binary output
* No dynamic linking to Rust libs
* ffmpeg is the only external executable dependency

If it increases binary size without UX value, it is rejected.

### Dependency Policy

* Prefer `std`
* Parser implemented manually or with lightweight crates
* No CLI meta-frameworks that obscure flow

The CLI behavior must be readable from `main.rs`.

### Platform Support

Required:

* Linux
* macOS
* Windows

Unsupported platforms must fail at build time, not runtime.

## CLI UX Contract

### Invocation Modes

Two valid modes only:

1. Interactive: `ff`
2. Direct: `ff <plain-english-command>`

No hybrid modes.

### Output Rules

* Generated ffmpeg command is always printed
* Execution result streams directly from ffmpeg
* No spinners, no animations, no TUI

Text output only.

### Exit Codes

* `0` → success
* `1` → user input error
* `2` → ffmpeg execution failure
* `>2` → internal error

Scripts depend on this contract.

## Development Workflow

### Grammar-Driven Development

Every feature begins with:

1. A written English grammar rule
2. A parser implementation
3. Snapshot tests

No implementation without grammar.

### Intent-Centric Architecture

Internal flow is fixed:

```
Input string
→ Tokenizer
→ Intent struct
→ ffmpeg command builder
→ Executor
```

Skipping layers is not allowed.

### Testing Discipline

* Unit tests for parsing
* Snapshot tests for command generation
* No reliance on real media files in CI

Media correctness is ffmpeg's responsibility.

## Build & Release Policy

### Cross-Compilation

* Use Rust cross-compilation
* One binary per OS/arch
* No installers, no package managers required

Download → chmod → run.

### Versioning Rules

* Semantic Versioning
* Grammar changes are breaking
* Grammar is a public API

Breaking grammar without a major bump is prohibited.

## Governance

### Feature Acceptance Criteria

A feature is accepted only if:

* It fits the CLI-only model
* It can be parsed deterministically
* It does not add configuration state
* It does not require platform-specific branching

Complexity is grounds for rejection.

### Backward Compatibility

Existing commands must never change meaning.

Deprecation requires:

* warning output
* documented replacement
* at least one minor release cycle

### Maintainer Authority

Maintainers may veto:

* flag-heavy designs
* smart guessing
* "advanced mode" requests

ff is a translator, not a power-user cockpit.

**Version**: 1.2.0
**Ratified**: 2025-01-27
**Last Amended**: 2025-12-27
