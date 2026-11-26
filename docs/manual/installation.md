---
layout: manual
title: Installation
description: "Install ffrs and ffmpeg on Linux, macOS, and Windows."
---

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

Make sure the Cargo bin directory is in your `PATH`: usually `~/.cargo/bin`, or `%USERPROFILE%\.cargo\bin` on Windows. You may need to restart the terminal after installing.

### Prerequisites

`ffrs` generates commands but does not include `ffmpeg`. Install it separately and confirm `ffmpeg -version` works in your terminal:

- **Windows**: download from [ffmpeg.org](https://ffmpeg.org/download.html) or use a package manager such as Chocolatey.
- **macOS**: `brew install ffmpeg`
- **Linux**: use your package manager, for example `sudo apt install ffmpeg` on Ubuntu.

`--dry-run` does not need `ffmpeg` because it only prints the command. A Rust 1.85+ toolchain is required to build from source.
