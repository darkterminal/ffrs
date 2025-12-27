# Data Model: Media Conversion CLI Tool

## Core Entities

### Input File
- **Path**: String representing the file system path to the input media file
- **Validation**: Must exist and be readable, valid media format from supported list
- **Relationships**: One-to-one with Output File (for conversion operations)

### Output File  
- **Path**: String representing the file system path where output will be written
- **Validation**: Directory must be writable, filename follows predictable naming from input
- **Relationships**: One-to-one with Input File (for conversion operations)

### Plain English Command
- **Text**: The original user input as a string
- **Validation**: Must conform to defined grammar rules
- **Relationships**: One-to-one with Intent Struct

### Intent Struct
- **Operation Type**: Enum (Convert, Resize, Transcode, etc.)
- **Input Path**: Reference to Input File
- **Output Path**: Reference to Output File  
- **Parameters**: Map of operation-specific parameters (format, quality, etc.)
- **Validation**: All required fields present and valid
- **Relationships**: One-to-one with Plain English Command, one-to-one with FFmpeg Command

### FFmpeg Command
- **Command String**: The complete ffmpeg command to execute
- **Validation**: Properly formatted, all required arguments present
- **Relationships**: One-to-one with Intent Struct

## State Transitions

### Command Processing Flow
1. **Input Received**: Plain English Command enters the system
2. **Tokenized**: Command is broken into tokens
3. **Parsed**: Tokens are converted to Intent Struct
4. **Command Built**: Intent Struct becomes FFmpeg Command
5. **Executed**: FFmpeg Command is executed with appropriate output
6. **Completed**: Process finishes with success or error status

## Validation Rules

### From Requirements
- Input file must exist and be readable (FR-001)
- Output file must be written to same directory as input by default (FR-007)
- Command must be deterministic (FR-001)
- System must operate offline (FR-006)

### Format Validation
- Supported input formats: MP4, AVI, MOV, WMV, MKV, WebM, MP3, WAV, FLAC, JPG, PNG, GIF
- Output format must be different from input format
- File paths must be valid for the target platform