---
layout: manual
title: Manual
description: "User manual for the ffrs command-line tool."
---

## User manual

`ffrs` converts media files from plain English commands. Instead of remembering `ffmpeg` options, you type something like `convert video.mp4 to video.avi` and `ffrs` builds the command for you. It runs on Linux, macOS, and Windows.

> Commands execute immediately after being printed. Start with `--dry-run` if you want to inspect the generated command first.

### Chapters

- [Installation]({{ '/manual/installation/' | relative_url }}) — install `ffrs` and `ffmpeg`, verify both.
- [Usage]({{ '/manual/usage/' | relative_url }}) — direct mode, interactive mode, and execution rules.
- [Command syntax]({{ '/manual/syntax/' | relative_url }}) — operations, parameters, paths, and validation.
- [Common tasks]({{ '/manual/tasks/' | relative_url }}) — conversion, audio extraction, resizing, and transcoding.
- [Troubleshooting]({{ '/manual/troubleshooting/' | relative_url }}) — error messages and what they mean.
- [Reference]({{ '/manual/reference/' | relative_url }}) — formats, options, exit codes, and library API.

### Quick example

```bash
ffrs --dry-run "convert video.mp4 to video.avi"
```

```text
ffmpeg "-n" "-i" "video.mp4" "video.avi"
```
