# FF - Media Conversion CLI Tool

A Rust CLI application that translates plain English commands into equivalent ffmpeg commands.

## Installation

1. Download the appropriate binary for your platform from the releases page
2. Make it executable: `chmod +x ff`
3. Place it in your PATH or run directly

## Usage

### Interactive Mode

Run `ff` to enter interactive mode:

```bash
ff
> convert video.mp4 to video.webm
```

### Direct Mode

Run the tool directly with your command:

```bash
ff "convert video.mp4 to video.webm"
```

### Dry Run

Test your command without executing it:

```bash
ff --dry-run "convert video.mp4 to video.webm"
```

### Specify Output Directory

```bash
ff --output /path/to/output "convert video.mp4 to video.webm"
```

## Examples

- Convert video format: `ff "convert myvideo.mp4 to myvideo.avi"`
- Change quality: `ff "convert myvideo.mp4 to high quality"`
- Extract audio: `ff "extract audio from myvideo.mp4 to myvideo.mp3"`

## Features

- **Plain English Interface**: Use natural language to describe media conversions
- **Interactive Mode**: Enter commands in a REPL-like environment
- **Direct Mode**: Execute commands directly from the command line
- **Dry Run**: Preview the generated ffmpeg command without executing it
- **Predictable Output Naming**: Output files are named predictably based on input files
- **Cross-Platform**: Works on Linux, macOS, and Windows
- **Offline Operation**: No internet connection required

## Supported Formats

Input and output formats include: MP4, AVI, MOV, WMV, MKV, WebM, MP3, WAV, FLAC, JPG, PNG, GIF

## Error Handling

The tool provides detailed error messages with specific guidance on how to fix issues.

## Exit Codes

- `0`: Success - command executed successfully
- `1`: User input error - invalid command or grammar
- `2`: FFmpeg execution failure - ffmpeg returned an error
- `>2`: Internal error - unexpected application error