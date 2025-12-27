# Quickstart Guide: Media Conversion CLI Tool

## Installation

1. Download the appropriate binary for your platform from the releases page
2. Make it executable: `chmod +x ff`
3. Place it in your PATH or run directly

## Interactive Mode

1. Run `ff` to enter interactive mode
2. Enter a plain English command like "convert video.mp4 to video.webm"
3. The tool will display the equivalent ffmpeg command
4. Press Enter to execute or Ctrl+C to cancel

## Direct Mode

Run the tool directly with your command:

```bash
ff "convert video.mp4 to video.webm"
```

## Dry Run

Test your command without executing it:

```bash
ff --dry-run "convert video.mp4 to video.webm"
```

## Examples

- Convert video format: `ff "convert myvideo.mp4 to myvideo.avi"`
- Change quality: `ff "convert myvideo.mp4 to high quality"`
- Extract audio: `ff "extract audio from myvideo.mp4 to myvideo.mp3"`

## Output Location

By default, output files are placed in the same directory as input files. You can specify an output directory with the `--output` flag:

```bash
ff --output /path/to/output "convert video.mp4 to video.webm"
```

## Error Handling

If you encounter an error, the tool will provide a detailed message with specific guidance on how to fix the issue.