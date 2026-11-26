---
layout: manual
title: Troubleshooting
description: "Common ffrs errors and how to fix them."
---

## Troubleshooting

### `ffmpeg is not available in PATH`

Install `ffmpeg` and make sure it is accessible from your terminal. Check with `ffmpeg -version`. See [Installation]({{ '/manual/installation/' | relative_url }}).

### `Input file does not exist`

Check the spelling and path of the input file. Run the command from the directory that contains it, or use an absolute path.

### `output path must use a supported format`

Give the output a supported extension such as `.mp4`, `.avi`, `.mp3`, or `.png`.

### `Output path must differ from input path`

Choose a different output name or format. `ffrs` will not overwrite the input file.

### `Output directory does not exist or is not a directory`

Create the directory first, or pass a different existing directory to `--output`.

### `Output file already exists`

By default `ffrs` passes `-n` to `ffmpeg`, so an existing output is preserved and `ffmpeg` exits with an error. Rerun with `--yes` if overwriting is intended.

### `Unsupported format`

The input extension is not supported. Check the [format list]({{ '/manual/reference/' | relative_url }}).

### `ffrs: command not found`

Make sure `cargo install ffrs` completed and the Cargo bin directory is in your `PATH`. You may need to restart your terminal.

### Getting help

- `ffrs --help` lists the available options.
- `ffrs --version` prints the installed version.
- `ffrs --dry-run "..."` shows the generated command without running it.

If something still fails, open an issue on [GitHub](https://github.com/darkterminal/ffrs/issues) with the command, the generated output, and your OS.
