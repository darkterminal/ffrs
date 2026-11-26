# User Manual for ffrs CLI Tool

## Table of Contents

1. [Introduction](#introduction)
2. [Installation](#installation)
3. [Getting Started](#getting-started)
4. [Basic Usage](#basic-usage)
5. [Common Tasks](#common-tasks)
6. [Troubleshooting](#troubleshooting)
7. [Reference](#reference)

## Introduction

`ffrs` is a tool that converts media files using plain English commands. Instead of remembering complex ffmpeg options, you type something like `convert video.mp4 to video.avi` and `ffrs` builds the ffmpeg command for you.

`ffrs` runs on Windows, macOS, and Linux.

## Installation

Release archives are not currently published, so install with Cargo.

### With Cargo (all platforms)

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

Make sure the Cargo bin directory is in your `PATH` (usually `~/.cargo/bin`, or `%USERPROFILE%\.cargo\bin` on Windows).

### Prerequisites

`ffrs` generates commands but does not include ffmpeg. Install ffmpeg separately and make sure `ffmpeg -version` works in your terminal:

- **Windows**: download from https://ffmpeg.org/download.html or use a package manager such as Chocolatey.
- **macOS**: `brew install ffmpeg`
- **Linux**: use your package manager, for example `sudo apt install ffmpeg` on Ubuntu.

`--dry-run` does not need ffmpeg because it only prints the command.

## Getting Started

`ffrs` has two modes:

1. **Direct mode**: pass the command as an argument.
2. **Interactive mode**: start `ffrs`, then type commands one at a time.

### Direct mode

```bash
ffrs "convert myvideo.mp4 to myvideo.avi"
```

### Interactive mode

```bash
ffrs --interactive
```

You will see a `>` prompt where you can type commands:

```text
> convert myvideo.mp4 to myvideo.avi
```

Type `quit` or `exit` to leave. Pressing Ctrl-D (or Ctrl-Z then Enter on Windows) also exits.

## Basic Usage

### Command pattern

```text
[operation] <input file> to <output file>
```

Examples:

- `convert video.mp4 to video.avi`
- `convert myvideo.mp4 to .avi` (same name, different format)
- `extract audio from myvideo.mp4 to myaudio.mp3`

### What ffrs does

When you run a command, `ffrs`:

1. Parses and validates the command and input file.
2. Prints the ffmpeg command it will run.
3. Runs ffmpeg immediately.

Commands run immediately after being printed. By default `ffrs` adds `-n`, so ffmpeg will not overwrite an existing output file. If you want to allow overwriting, pass `--yes`, which adds `-y`. There is no interactive confirmation prompt. If you only want to see the command, use `--dry-run`.

For example:

```bash
ffrs --dry-run "convert video.mp4 to video.avi"
```

prints:

```text
ffmpeg "-n" "-i" "video.mp4" "video.avi"
```

With `--yes`, the same command prints `ffmpeg "-y" "-i" "video.mp4" "video.avi"`.

### Paths with spaces

Wrap the file names in double quotes inside the command, and wrap the whole command in single quotes in your shell:

```bash
ffrs 'convert "my video.mp4" to "my video.avi"'
```

Relative paths (`sub/clip.mp4`, `./clip.mp4`, `../clip.mp4`), absolute paths, and Windows paths (`C:\Users\me\clip.mp4`) are supported.

### Output rules

- The output must use a supported extension; `out.txt` is rejected.
- The output must be a different file from the input.
- The input file must exist and be a file.
- `--output` must name a directory that already exists. `ffrs` uses only the output file name from the command and places it in that directory. It does not create directories.

Examples:

```bash
# Same directory as the input
ffrs "convert video.mp4 to video.avi"

# Into an existing directory
ffrs --output /path/to/output "convert video.mp4 to video.avi"
```

## Common Tasks

### Converting video formats

```bash
ffrs "convert myvideo.mp4 to myvideo.avi"
ffrs "convert myvideo.mp4 to .avi"
```

### Extracting audio

```bash
ffrs "extract audio from myvideo.mp4 to myaudio.mp3"
```

Shorter forms work too:

```bash
ffrs "extract audio myvideo.mp4 to myaudio.mp3"
ffrs "extract myvideo.mp4 to myaudio.mp3"
```

### Converting audio formats

```bash
ffrs "convert mysong.wav to mysong.mp3"
```

### Converting images

```bash
ffrs "convert photo.jpg to photo.png"
```

## Troubleshooting

### "ffmpeg is not available in PATH"

Install ffmpeg and make sure it is accessible from your terminal. Check with `ffmpeg -version`.

### "Input file does not exist"

Check the spelling and path of the input file. Run the command from the directory that contains it, or use an absolute path.

### "output path must use a supported format"

Give the output a supported extension such as `.mp4`, `.avi`, `.mp3`, or `.png`.

### "Output path must differ from input path"

Choose a different output name or format; `ffrs` will not overwrite the input file.

### "Output directory does not exist or is not a directory"

Create the directory first, or pass a different existing directory to `--output`.

### "Output file already exists"

By default `ffrs` passes `-n` to ffmpeg, so an existing output is preserved and ffmpeg exits with an error. Rerun with `--yes` if overwriting is intended.

### "Unsupported format"

The input extension is not supported. See the supported formats list below.

### `ffrs: command not found`

Make sure `cargo install ffrs` completed and the Cargo bin directory is in your `PATH`. You may need to restart your terminal.

### Getting help

- `ffrs --help` lists the available options.
- `ffrs --version` prints the installed version.
- `ffrs --dry-run "..."` shows the generated command without running it.

## Reference

### Supported operations

- `convert`: convert a file to another supported format
- `extract audio`: extract audio from a video file

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

### File naming

- `video.mp4` → `video.avi` when converting to a different format
- `path/to/video.mp4` → `path/to/video.avi` (same directory as the input)
- `convert video.mp4 to .avi` → `video.avi`

### Exit codes

- `0`: success
- `1`: user input or CLI usage error
- `2`: ffmpeg execution failure
- `3`: internal error
