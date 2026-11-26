---
layout: manual
title: Command syntax
description: "Operations, parameters, and validation rules for ffrs commands."
---

## Command syntax

The general pattern is:

```text
[operation] <input file> to [parameter] <output file>
```

### Operations

| Operation | Example | Notes |
|---|---|---|
| `convert` | `convert clip.mp4 to clip.avi` | Change container or format |
| `extract audio` | `extract audio from clip.mp4 to song.mp3` | `extract audio clip.mp4 to song.mp3` and `extract clip.mp4 to song.mp3` also work |
| `resize` | `resize clip.mp4 to 1280x720 small.mp4` | Without a parameter, uses 1920x1080 |
| `transcode` | `transcode clip.mp4 to h265 out.mp4` | Without a parameter, uses libx264 + aac |

### Parameters

- **Resize**: width and height are positive integers separated by `x`, for example `1280x720`. A malformed resolution fails before `ffmpeg` runs.
- **Transcode video**: `h264`, `libx264`, `h265`, `libx265`, `vp9`, `av1`, `mpeg4`. Sets `-c:v`.
- **Transcode audio**: `aac`, `mp3`, `flac`, `opus`, `vorbis`. Sets `-c:a`.
- An unknown codec fails before `ffmpeg` runs.

### Derived output names

- `video.mp4` → `video.avi` when converting to a different format.
- `path/to/video.mp4` → `path/to/video.avi`, in the same directory as the input.
- `convert video.mp4 to .avi` → `video.avi`, keeping the input base name.

### Validation

Before executing anything, `ffrs` checks that:

- the operation is recognized;
- the input path exists and is a file;
- the input extension is supported;
- the output extension is supported;
- the output path differs from the input path;
- `--output`, when given, points to an existing directory;
- a resolution or codec parameter, when given, is valid.

See [Troubleshooting]({{ '/manual/troubleshooting/' | relative_url }}) for the corresponding messages.
