# ffrs - Media Conversion CLI Tool

[![CI](https://github.com/darkterminal/ffrs/actions/workflows/ci.yml/badge.svg)](https://github.com/darkterminal/ffrs/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/ffrs.svg)](https://crates.io/crates/ffrs)
[![docs.rs](https://docs.rs/ffrs/badge.svg)](https://docs.rs/ffrs)
[![license](https://img.shields.io/crates/l/ffrs.svg)](LICENSE)

`ffrs` translates plain English commands into `ffmpeg` commands. It works as both a command-line tool and a Rust library.

## The Story

I’m a simple guy.

I watch a YouTube livestream about vibe coding.
That curiosity pulls me into a search. I look up **spec-kit**—and there it is. Found.

Then I drift.
I scroll my Facebook timeline.
Scroll.
Scroll again.
Refresh.

Suddenly, nixCraft posts about the **ffrs CLI tool**.
Yeah, it’s written in JavaScript—and the comment section is exactly what you’d expect.

Then someone drops the line that changes everything:

> “This is my only complaint. It’s a great idea—just rewrite it in Rust or Go and ship it as a single binary.”

Me?
**Why not.**

No overthinking. No debate.
Just momentum.

**Vibe it. Build it. Ship it.**

## Features

- Translate plain English commands to ffmpeg commands
- Convert video, audio, and image formats
- Extract audio from a video file
- Interactive and direct command modes
- Dry-run to inspect the generated command before running it
- Deterministic, structured command generation

## Installation

Actual conversion requires `ffmpeg` in your `PATH`. `--dry-run` only prints the command and does not need ffmpeg.

### From crates.io

```bash
cargo install ffrs
```

### From source

```bash
git clone https://github.com/darkterminal/ffrs.git
cd ffrs
cargo install --path .
```

Verify the installation:

```bash
ffrs --version
```

Prebuilt release archives are not currently published. Use one of the Cargo methods above.

## Usage

### Direct mode

```bash
# Convert a video file
ffrs "convert video.mp4 to video.avi"

# Inspect the command without running it
ffrs --dry-run "convert video.mp4 to video.avi"

# Allow overwriting an existing output file
ffrs --yes "convert video.mp4 to video.avi"

# Extract audio
ffrs "extract audio from video.mp4 to audio.mp3"

# Paths with spaces
ffrs 'convert "my video.mp4" to "my video.avi"'

# Write the output into an existing directory
ffrs --output /path/to/output "convert video.mp4 to video.avi"

# Version and help
ffrs --version
ffrs --help
```

The command is printed before execution. By default `ffrs` passes `-n` to ffmpeg, so an existing output file is never overwritten. Pass `--yes` to send `-y` instead. Use `--dry-run` when you only want to inspect the command.

### Interactive mode

```bash
ffrs --interactive
```

Type commands at the `>` prompt. Type `quit` or `exit` to leave; end-of-input (for example Ctrl-D) also exits.

### As a library

Add to your `Cargo.toml`:

```toml
[dependencies]
ffrs = "0.1.0"
```

Use in your code:

```rust
use ffrs::{CommandBuilder, Parser, Tokenizer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut tokenizer = Tokenizer::new("convert video.mp4 to video.avi");
    let tokens = tokenizer.tokenize();

    let mut parser = Parser::new(tokens);
    let intent = parser.parse()?;

    let cmd_builder = CommandBuilder::new();
    let ffmpeg_cmd = cmd_builder.build_command(&intent)?;

    println!("Generated command: {}", ffmpeg_cmd.to_display_string());

    Ok(())
}
```

## Command syntax

```text
[operation] <input> to <output>
```

Supported operations:

- `convert` — convert a file to another supported format
- `extract audio from <input> to <output>` — extract audio (shorter forms `extract audio <input> ...`, `extract <input> ...`, and `extractaudio <input> ...` also work)

Rules:

- The input file must exist and be a file.
- The output must have a supported extension and must differ from the input.
- `--output` must point to a directory that already exists; only the output file name is taken from the command.
- Existing output files are preserved by default; `--yes` opts in to overwriting them.
- Relative, absolute, quoted, and Windows-style paths are recognized.

## Supported formats

- Video: MP4, AVI, MOV, WMV, MKV, WebM
- Audio: MP3, WAV, FLAC
- Images: JPG, PNG, GIF

## Exit codes

- `0`: success
- `1`: user input error (including CLI usage errors)
- `2`: ffmpeg execution failure
- `3`: internal error

## Known limitations

- Commands execute immediately after being printed; use `--dry-run` to inspect first, and `--yes` only when overwriting is intended.
- Output directories are not created automatically.
- Glob patterns and `~` are not expanded by `ffrs`; the shell may expand them first.
- No release binaries are published at the moment.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for notable changes.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for setup and contribution guidelines.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
