---
layout: manual
title: Common tasks
description: "Everyday ffrs recipes for conversion, audio, resizing, and codecs."
permalink: /manual/tasks/
---

## Common tasks

All examples can be inspected first by adding `--dry-run`.

### Convert a video format

```bash
ffrs "convert myvideo.mp4 to myvideo.avi"
ffrs "convert myvideo.mp4 to .avi"
```

The second form keeps the input base name.

### Extract audio

```bash
ffrs "extract audio from myvideo.mp4 to myaudio.mp3"
```

Shorter forms work too:

```bash
ffrs "extract audio myvideo.mp4 to myaudio.mp3"
ffrs "extract myvideo.mp4 to myaudio.mp3"
```

### Convert audio

```bash
ffrs "convert mysong.wav to mysong.mp3"
```

### Convert images

```bash
ffrs "convert photo.jpg to photo.png"
```

### Resize a video

```bash
ffrs "resize myvideo.mp4 to 1280x720 small.mp4"
```

Omit the resolution to use 1920x1080:

```bash
ffrs "resize myvideo.mp4 to small.mp4"
```

### Choose a codec

```bash
ffrs "transcode myvideo.mp4 to h265 out.mp4"
ffrs "transcode myvideo.mp4 to opus out.mp4"
```

Video codecs set `-c:v`; audio codecs set `-c:a`. The full lists are in the [syntax reference]({{ '/manual/syntax/' | relative_url }}).
