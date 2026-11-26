---
layout: manual
title: Usage
description: "Direct mode, interactive mode, and command execution rules in ffrs."
---

## Usage

`ffrs` has two modes.

### Direct mode

Pass the command as an argument:

```bash
ffrs "convert myvideo.mp4 to myvideo.avi"
```

### Interactive mode

Start `ffrs`, then type commands one at a time:

```bash
ffrs --interactive
```

You will see a `>` prompt:

```text
> convert myvideo.mp4 to myvideo.avi
```

Type `quit` or `exit` to leave. End-of-input also exits: Ctrl-D on Unix, or Ctrl-Z then Enter on Windows.

### What happens when you run a command

1. `ffrs` parses the command and validates the input file.
2. It prints the `ffmpeg` command it will run.
3. It runs `ffmpeg` immediately.

There is no confirmation prompt after the command is printed. By default `ffrs` passes `-n`, so `ffmpeg` will not overwrite an existing output file. Pass `--yes` to send `-y` instead. To see the command without running it, use `--dry-run`:

```bash
ffrs --dry-run "convert video.mp4 to video.avi"
```

```text
ffmpeg "-n" "-i" "video.mp4" "video.avi"
```

With `--yes`, the same command prints `ffmpeg "-y" "-i" "video.mp4" "video.avi"`.

### Paths with spaces

Wrap the file names in double quotes inside the command, and the whole command in single quotes in your shell:

```bash
ffrs 'convert "my video.mp4" to "my video.avi"'
```

Relative paths (`sub/clip.mp4`, `./clip.mp4`, `../clip.mp4`), absolute paths, and Windows paths such as `C:\Users\me\clip.mp4` are supported. Shell expansion of globs and `~` is left to the shell; `ffrs` does not expand them itself.

### Output rules

- The output must use a [supported extension]({{ '/manual/reference/' | relative_url }}); `out.txt` is rejected.
- The output must be a different file from the input.
- The input file must exist and be a file.
- `--output` must name a directory that already exists. `ffrs` uses only the output file name from the command and places it in that directory. It does not create directories.

```bash
# Same directory as the input
ffrs "convert video.mp4 to video.avi"

# Into an existing directory
ffrs --output /path/to/output "convert video.mp4 to video.avi"
```
