# User Manual for FF CLI Tool

## Table of Contents
1. [Introduction](#introduction)
2. [Installation](#installation)
3. [Getting Started](#getting-started)
4. [Basic Usage](#basic-usage)
5. [Common Tasks](#common-tasks)
6. [Troubleshooting](#troubleshooting)
7. [Reference](#reference)

## Introduction

`ffrs` is a simple tool that helps you convert media files (like videos and audio) using plain English commands. Instead of remembering complex technical commands, you can just type what you want to do in simple English, and FF will handle the technical details for you.

For example, instead of typing a complex command, you can just say "convert video.mp4 to video.avi" and FF will do the conversion for you.

`ffrs` works on Windows, Mac, and Linux computers. It's designed to be simple to use but powerful enough to handle most media conversion tasks.

## Installation

### Windows

1. Download the latest `ffrs` release from the [releases page](https://github.com/darkterminal/ffrs/releases) (look for the file ending in `-pc-windows-msvc.zip`)
2. Extract the downloaded file
3. Copy the `ffrs.exe` file to a folder on your computer (like `C:\Program Files\ffrs\`)
4. Add that folder to your system PATH (search online for "how to add to PATH Windows" if you need help)
5. Open Command Prompt and type `ffrs --version` to verify the installation

### Mac

1. Download the latest ffrs release from the [releases page](https://github.com/darkterminal/ffrs/releases) (look for the file ending in `-apple-darwin.tar.gz`)
2. Extract the downloaded file
3. Copy the `ffrs` file to `/usr/local/bin/` (you may need administrator privileges)
4. Open Terminal and type `ff --version` to verify the installation

### Linux

1. Download the latest ffrs release from the [releases page](https://github.com/darkterminal/ffrs/releases) (look for the file ending in `-unknown-linux-musl.tar.gz`)
2. Extract the downloaded file
3. Copy the `ffrs` file to `/usr/local/bin/` (you may need sudo privileges)
4. Open Terminal and type `ffrs --version` to verify the installation

### Prerequisites

FF requires `ffmpeg` to be installed on your system. FFmpeg is the actual tool that does the media conversion, while FF just translates your English commands into the commands that ffmpeg understands.

To install ffmpeg:
- **Windows**: Download from https://ffmpeg.org/download.html or use a package manager like Chocolatey
- **Mac**: Install with Homebrew by running `brew install ffmpeg`
- **Linux**: Install with your package manager (e.g., `sudo apt install ffmpeg` on Ubuntu)

## Getting Started

ffrs can be used in two ways:

1. **Direct mode**: Type your command directly
2. **Interactive mode**: Start ffrs and then type your commands

### Direct Mode

In direct mode, you specify your command right after typing `ffrs`:

```bash
ffrs "convert myvideo.mp4 to myvideo.avi"
```

### Interactive Mode

In interactive mode, you start ffrs first, then type your commands:

```bash
ffrs --interactive
```

Then you'll see a prompt where you can type commands:

```
> convert myvideo.mp4 to myvideo.avi
```

To exit interactive mode, type `quit` or `exit`.

## Basic Usage

### Understanding ffrs Commands

ffrs commands follow a simple pattern:

```
[operation] [input file] to [output file]
```

Examples:
- `convert video.mp4 to video.avi`
- `convert myvideo.mp4 to .avi` (converts to the same name but different format)
- `extract audio from myvideo.mp4 to myaudio.mp3`

### What ffrs Does

When you run a command, ffrs does the following:
1. Shows you the technical command it will run
2. Asks for confirmation (unless you use `--yes`)
3. Runs the command using ffmpeg

For example, if you type:
```
ffrs "convert video.mp4 to video.avi"
```

ffrs will show:
```
ffmpeg -i "video.mp4" "video.avi"
```

Then it will run this command to perform the conversion.

### Dry Run

To see what ffrs would do without actually doing it, use the `--dry-run` option:

```bash
ffrs --dry-run "convert video.mp4 to video.avi"
```

This will show you the command ffrs would run but won't actually execute it.

## Common Tasks

### Converting Video Formats

**Convert MP4 to AVI:**
```
ffrs "convert myvideo.mp4 to myvideo.avi"
```

**Convert to different format with same name:**
```
ffrs "convert myvideo.mp4 to .avi"
```

### Converting Audio Formats

**Extract audio from video:**
```
ffrs "extract audio from myvideo.mp4 to myaudio.mp3"
```

**Convert audio file format:**
```
ffrs "convert mysong.wav to mysong.mp3"
```

### Converting Images

**Convert image formats:**
```
ffrs "convert photo.jpg to photo.png"
```

### Specifying Output Directory

To save the output file in a different directory:
```bash
ffrs --output /path/to/output "convert video.mp4 to video.avi"
```

## Troubleshooting

### Common Issues and Solutions

**Error: "ffmpeg is not available in PATH"**
- Make sure ffmpeg is installed on your system
- Check that ffmpeg is in your system PATH
- Try running `ffmpeg -version` in your terminal to verify

**Error: "No such file or directory"**
- Check that the file path is correct
- Make sure the file exists in the specified location
- Check for typos in the filename

**Error: "Unsupported format"**
- Make sure the input file is in a supported format
- ffrs supports: MP4, AVI, MOV, WMV, MKV, WebM, MP3, WAV, FLAC, JPG, PNG, GIF

**ffrs command not found**
- Make sure ffrs is properly installed and in your PATH
- On Windows, you might need to restart your command prompt after installation
- On Mac/Linux, you might need to log out and back in

### Getting Help

If you need help with FF, you can:
- Use `ffrs --help` to see available options
- Check the output ffrs shows you before running commands
- Look at the examples in this manual

## Reference

### Supported Operations

- `convert`: Convert a file from one format to another
- `extract audio`: Extract audio from a video file
- (More operations may be added in future versions)

### Supported Formats

**Video**: MP4, AVI, MOV, WMV, MKV, WebM
**Audio**: MP3, WAV, FLAC
**Images**: JPG, PNG, GIF

### Command Options

- `--interactive`: Start ffrs in interactive mode
- `--dry-run`: Show the command without executing it
- `--output [path]`: Specify output directory
- `--help`: Show help information
- `--version`: Show version information

### File Naming

ffrs follows predictable naming rules:
- Input: `video.mp4` → Output: `video.avi` (when converting to different format)
- Input: `path/to/video.mp4` → Output: `path/to/video.avi` (output in same directory)
- Input: `video.mp4` → Output: `video.avi` (when using format like `.avi`)

### Exit Codes

- `0`: Success
- `1`: User input error
- `2`: FFmpeg execution failure
- `>2`: Internal error