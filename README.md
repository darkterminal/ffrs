# FF - Media Conversion CLI Tool

FF is a CLI tool that translates plain English commands into ffmpeg commands. It provides both a library API and a command-line interface for media conversion tasks.

## The Story

I’m a simple guy.

I watch a YouTube livestream about vibe coding.
That curiosity pulls me into a search. I look up **spec-kit**—and there it is. Found.

Then I drift.
I scroll my Facebook timeline.
Scroll.
Scroll again.
Refresh.

Suddenly, nixCraft posts about the **ff CLI tool**.
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
- Support for multiple media formats
- Interactive and direct command modes
- Dry-run functionality
- Deterministic and inspectable command generation

## Installation

### As a Binary

You can install FF using Cargo:

```bash
cargo install ff
```

### From Source

```bash
git clone https://github.com/darkterminal/ff-rs.git
cd ff-rs
cargo install --path .
```

## Usage

### Command Line Interface

```bash
# Convert a video file
ff "convert video.mp4 to video.avi"

# Interactive mode
ff --interactive

# Dry run (shows command without executing)
ff --dry-run "convert video.mp4 to video.avi"

# Specify output directory
ff --output /path/to/output "convert video.mp4 to video.avi"
```

### As a Library

Add to your `Cargo.toml`:

```toml
[dependencies]
ff = "0.1.0"
```

Use in your code:

```rust
use ff::{Tokenizer, Parser, CommandBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Tokenize the command
    let mut tokenizer = Tokenizer::new("convert video.mp4 to video.avi");
    let tokens = tokenizer.tokenize();

    // Parse the tokens into an intent
    let mut parser = Parser::new(tokens);
    let intent = parser.parse()?;

    // Build the ffmpeg command
    let cmd_builder = CommandBuilder::new();
    let ffmpeg_cmd = cmd_builder.build_command(&intent)?;

    println!("Generated command: {}", ffmpeg_cmd);

    Ok(())
}
```

## Supported Formats

- Video: MP4, AVI, MOV, WMV, MKV, WebM
- Audio: MP3, WAV, FLAC
- Images: JPG, PNG, GIF

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.