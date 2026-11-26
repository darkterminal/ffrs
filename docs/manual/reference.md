---
layout: manual
title: Reference
description: "ffrs operations, formats, options, exit codes, and library API."
---

## Reference

### Supported operations

- `convert`: convert a file to another supported format
- `extract audio`: extract audio from a video file
- `resize`: scale a video to `<width>x<height>`
- `transcode`: choose a video or audio codec

### Supported formats

- **Video**: MP4, AVI, MOV, WMV, MKV, WebM
- **Audio**: MP3, WAV, FLAC
- **Images**: JPG, PNG, GIF

### Command options

- `--interactive`: start interactive mode
- `--dry-run`: print the command without executing it
- `--yes`: overwrite an existing output file (`-y`); the default is no overwrite (`-n`)
- `--output <dir>`: write the output file into an existing directory
- `--help`: show help
- `--version`: show version

### Exit codes

- `0`: success
- `1`: user input or CLI usage error
- `2`: `ffmpeg` execution failure
- `3`: internal error

### Library API

The tokenizer, parser, and command builder are also available as a Rust library:

```rust
use ffrs::{CommandBuilder, Parser, Tokenizer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tokens = Tokenizer::new("convert video.mp4 to video.avi").tokenize();
    let intent = Parser::new(tokens).parse()?;
    let command = CommandBuilder::new().build_command(&intent)?;

    println!("{}", command.to_display_string());
    Ok(())
}
```

This prints the command without executing it. See the [API reference](https://docs.rs/ffrs) for the full types and errors.
