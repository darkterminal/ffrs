# CLI Interface Contract: Media Conversion Tool

## Command Structure

### Direct Invocation
```
ff [OPTIONS] "<plain english command>"
```

### Interactive Invocation
```
ff
> <plain english command>
```

## Options Contract

### Standard Options
- `--dry-run`: Display the ffmpeg command without executing it
- `--output <path>`: Specify output directory (default: same as input)
- `--help`: Display help information
- `--version`: Display version information

## Exit Codes Contract

- `0`: Success - command executed successfully
- `1`: User input error - invalid command or grammar
- `2`: FFmpeg execution failure - ffmpeg returned an error
- `>2`: Internal error - unexpected application error

## Input Contract

### Plain English Commands
- Must be grammatically valid conversion instructions
- Format: "convert <input> to <output>" or similar
- Input and output must specify valid media formats
- File paths must be accessible from current working directory

### Supported Formats
Input and output formats include: MP4, AVI, MOV, WMV, MKV, WebM, MP3, WAV, FLAC, JPG, PNG, GIF

## Output Contract

### Command Display
- The equivalent ffmpeg command is always printed before execution
- Format: `ffmpeg [arguments]`

### Execution Output
- Execution result streams directly from ffmpeg
- No additional UI elements (spinners, animations, TUI)

### Error Output
- Detailed error messages with specific guidance on how to fix the issue
- Error messages are human-readable and actionable

## Behavior Contract

### Deterministic Mapping
- Identical input commands always produce identical ffmpeg commands
- No state affects command generation

### Offline Operation
- Tool operates without internet connection
- No external dependencies beyond ffmpeg

### File Handling
- Output files placed in same directory as input files by default
- Predictable naming based on input file name