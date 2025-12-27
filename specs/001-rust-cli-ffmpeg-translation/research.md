# Research: Media Conversion CLI Tool

## Decision: Rust CLI Framework Selection
**Rationale**: For the Rust CLI application, I've selected clap as the primary CLI framework because it's the most widely used and well-maintained CLI framework in the Rust ecosystem. It provides excellent support for both simple and complex command-line interfaces.
**Alternatives considered**: 
- structopt (now deprecated in favor of clap)
- argh (lightweight but less feature-rich)
- gumdrop (less actively maintained)

## Decision: Grammar Parsing Approach
**Rationale**: For grammar parsing, I've chosen to implement a custom tokenizer and parser following the constitution's requirement that "Parser implemented manually or with lightweight crates". This ensures the CLI behavior remains readable from main.rs and avoids heavy meta-frameworks that obscure flow.
**Alternatives considered**:
- Using a full parser generator like pest or nom (would be overkill for English command parsing)
- Regular expressions for simple pattern matching (insufficient for complex grammar)
- Natural language processing libraries (violates "no AI/heuristics" requirement)

## Decision: Command Execution Method
**Rationale**: Using Rust's std::process::Command for executing ffmpeg commands provides safe, synchronous execution with proper output streaming as required by the constitution's output rules. This approach allows the generated ffmpeg command to be printed before execution.
**Alternatives considered**:
- subprocess crate (similar functionality but less standard)
- Direct system() calls (less safe and controllable)

## Decision: Cross-Platform Binary Distribution
**Rationale**: Using cargo and cross for cross-compilation aligns with the constitution's requirements for single binary output and cross-compilation support. This approach creates one binary per OS/arch as specified.
**Alternatives considered**:
- Docker containers (violates single binary requirement)
- Multiple installation methods (violates "Download → chmod → run" principle)

## Decision: Testing Strategy
**Rationale**: Following the constitution's testing discipline with unit tests for parsing, snapshot tests for command generation, and avoiding real media files in CI. This ensures media correctness remains ffmpeg's responsibility while verifying our command generation.
**Alternatives considered**:
- Integration tests with real media files (violates constitution's "No reliance on real media files in CI")
- Property-based testing (unnecessary complexity for this use case)