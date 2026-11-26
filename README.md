# ffrs

[![CI](https://github.com/darkterminal/ffrs/actions/workflows/ci.yml/badge.svg)](https://github.com/darkterminal/ffrs/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/ffrs.svg)](https://crates.io/crates/ffrs)
[![docs.rs](https://docs.rs/ffrs/badge.svg)](https://docs.rs/ffrs)
[![license](https://img.shields.io/crates/l/ffrs.svg)](LICENSE)

`ffrs` turns plain English media commands into `ffmpeg` commands. Use it as a command-line tool or as a Rust library.

It began as a weekend answer to a comment: rewrite the idea in Rust and ship a single binary. The interface stays plain English, and the generated commands stay deterministic.

[Website and user manual](https://darkterminal.github.io/ffrs/) · [API reference](https://docs.rs/ffrs) · [crates.io](https://crates.io/crates/ffrs)

## Highlights

- Convert, resize, and transcode video, audio, and images without memorizing `ffmpeg` flags.
- Extract audio from a video with one readable command.
- Inspect the generated command with `--dry-run` before executing anything.
- Never overwrite an existing output by default; opt in with `--yes`.
- Use the same parser and builder from Rust through the library API.
- Runs on Linux, macOS, and Windows with a Rust 1.85+ toolchain.

## Installation

Actual conversion needs `ffmpeg` on your `PATH`. `--dry-run` only prints the generated command and does not need `ffmpeg`.

```bash
cargo install ffrs
```

From source:

```bash
git clone https://github.com/darkterminal/ffrs.git
cd ffrs
cargo install --path .
```

Verify the install:

```bash
ffrs --version
ffmpeg -version
```

Prebuilt release archives are not currently published. Use one of the Cargo methods above.

## Quick start

Inspect the command first. This example uses a file that already exists:

```bash
ffrs --dry-run "convert video.mp4 to video.avi"
```

```text
ffmpeg "-n" "-i" "video.mp4" "video.avi"
```

Remove `--dry-run` to run it. `ffrs` prints the command, then executes `ffmpeg` immediately. The default `-n` preserves an existing output file; pass `--yes` to send `-y` instead.

More examples:

```bash
# Extract audio
ffrs "extract audio from video.mp4 to audio.mp3"

# Resize to an explicit resolution
ffrs "resize video.mp4 to 1280x720 small.mp4"

# Choose a codec
ffrs "transcode video.mp4 to h265 out.mp4"

# File names with spaces: single quotes outside, double quotes inside
ffrs 'convert "my video.mp4" to "my video.avi"'

# Write into a directory that already exists
ffrs --output /path/to/output "convert video.mp4 to video.avi"
```

Interactive mode is useful when you want several commands in one session:

```bash
ffrs --interactive
```

Type commands at the `>` prompt. Use `quit`, `exit`, or end-of-input to leave.

## Command syntax

```text
[operation] <input> to [parameter] <output>
```

| Operation | Example | Notes |
|---|---|---|
| `convert` | `convert clip.mp4 to clip.avi` | Change the container or format |
| `extract audio` | `extract audio from clip.mp4 to song.mp3` | Shorter forms such as `extract clip.mp4 to song.mp3` also work |
| `resize` | `resize clip.mp4 to 1280x720 small.mp4` | Omit the parameter to use 1920x1080 |
| `transcode` | `transcode clip.mp4 to opus out.mp4` | Omit the parameter to use libx264 + aac |

Video codecs: `h264`, `libx264`, `h265`, `libx265`, `vp9`, `av1`, `mpeg4`. Audio codecs: `aac`, `mp3`, `flac`, `opus`, `vorbis`. A malformed resolution or unknown codec fails before `ffmpeg` runs.

Rules:

- The input must exist and be a file.
- The output needs a supported extension and must differ from the input.
- `--output` must point to an existing directory. Only the file name from the command is used there.
- Relative, absolute, quoted, and Windows-style paths are recognized.
- Output directories are not created automatically.

Supported formats:

- Video: MP4, AVI, MOV, WMV, MKV, WebM
- Audio: MP3, WAV, FLAC
- Images: JPG, PNG, GIF

Exit codes: `0` success, `1` input or CLI error, `2` `ffmpeg` failure, `3` internal error.

## Library usage

Add the crate:

```toml
[dependencies]
ffrs = "0.2.0"
```

Build a command without executing it:

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

## Limitations

- Commands execute as soon as they are printed. Use `--dry-run` to inspect first.
- Glob patterns and `~` are not expanded by `ffrs`; the shell may expand them first.
- Release binaries are not published yet.

## Documentation

- [Website and user manual](https://darkterminal.github.io/ffrs/)
- [API reference](https://docs.rs/ffrs)
- [Changelog](CHANGELOG.md)
- [Contributing guide](CONTRIBUTING.md)
- [Security policy](SECURITY.md)

## Contributing

Contributions are welcome. Read [CONTRIBUTING.md](CONTRIBUTING.md) for setup, verification commands, and the pull-request workflow. Run `cargo fmt`, `cargo clippy`, and `cargo test` before opening a PR.

This project follows the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). Please report unacceptable behavior as described in the security policy.

## License

MIT. See [LICENSE](LICENSE).
