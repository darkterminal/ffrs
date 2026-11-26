//! Unified error types with proper exit code mapping.
//!
//! Exit codes per constitution:
//! - 0: success
//! - 1: user input error
//! - 2: ffmpeg execution failure
//! - >2: internal error

use crate::command_builder::BuildError;

/// Application-level error with explicit exit code mapping.
#[derive(Debug)]
pub enum FfError {
    UserInput(String),
    ExecutionFailure(String),
    /// Reserved for unexpected internal bugs. Exit code: 3
    #[allow(dead_code)]
    Internal(String),
}

impl std::fmt::Display for FfError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FfError::UserInput(msg) => write!(f, "Input error: {}", msg),
            FfError::ExecutionFailure(msg) => write!(f, "Execution error: {}", msg),
            FfError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for FfError {}

impl From<BuildError> for FfError {
    fn from(error: BuildError) -> Self {
        FfError::UserInput(error.to_string())
    }
}

impl FfError {
    /// Maps this error to its designated exit code.
    pub fn exit_code(&self) -> i32 {
        match self {
            FfError::UserInput(_) => 1,
            FfError::ExecutionFailure(_) => 2,
            FfError::Internal(_) => 3,
        }
    }

    /// Returns the next step for the user, without the `Guidance:` prefix.
    pub fn guidance(&self) -> &'static str {
        match self {
            FfError::UserInput(msg) => {
                if msg.contains("Output directory") {
                    "Create the output directory first or choose an existing one."
                } else if msg.contains("must differ from input") {
                    "Choose a different output file name or format."
                } else if msg.contains("must use a supported format") {
                    "Use a supported output extension: MP4, AVI, MOV, WMV, MKV, WebM, MP3, WAV, FLAC, JPG, PNG, GIF"
                } else if msg.contains("not exist") || msg.contains("not found") {
                    "Make sure the input file path is correct and the file exists."
                } else if msg.contains("Unsupported format") {
                    "Check supported formats: MP4, AVI, MOV, WMV, MKV, WebM, MP3, WAV, FLAC, JPG, PNG, GIF"
                } else {
                    "Make sure your command follows the format 'convert <input> to <output>' or similar.\nExample: 'convert video.mp4 to video.avi'"
                }
            }
            FfError::ExecutionFailure(msg) => {
                if msg.contains("not available") {
                    "Install ffmpeg and ensure it is accessible in your PATH."
                } else {
                    "Check ffmpeg output above for details. Input file may be corrupted or format may be invalid."
                }
            }
            FfError::Internal(_) => {
                "This is an unexpected bug. Please report it at https://github.com/darkterminal/ffrs/issues"
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exit_codes() {
        assert_eq!(FfError::UserInput("bad input".into()).exit_code(), 1);
        assert_eq!(
            FfError::ExecutionFailure("ffmpeg failed".into()).exit_code(),
            2
        );
        assert_eq!(FfError::Internal("unexpected bug".into()).exit_code(), 3);
    }

    #[test]
    fn test_guidance_preserves_every_branch() {
        let cases = [
            (
                FfError::UserInput("Output directory does not exist".into()),
                "Create the output directory first or choose an existing one.",
            ),
            (
                FfError::UserInput("Output path must differ from input path".into()),
                "Choose a different output file name or format.",
            ),
            (
                FfError::UserInput("output path must use a supported format".into()),
                "Use a supported output extension: MP4, AVI, MOV, WMV, MKV, WebM, MP3, WAV, FLAC, JPG, PNG, GIF",
            ),
            (
                FfError::UserInput("Input file does not exist: missing.mp4".into()),
                "Make sure the input file path is correct and the file exists.",
            ),
            (
                FfError::UserInput("file not found".into()),
                "Make sure the input file path is correct and the file exists.",
            ),
            (
                FfError::UserInput("Unsupported format: xyz".into()),
                "Check supported formats: MP4, AVI, MOV, WMV, MKV, WebM, MP3, WAV, FLAC, JPG, PNG, GIF",
            ),
            (
                FfError::UserInput("bad command".into()),
                "Make sure your command follows the format 'convert <input> to <output>' or similar.\nExample: 'convert video.mp4 to video.avi'",
            ),
            (
                FfError::ExecutionFailure("ffmpeg is not available in PATH".into()),
                "Install ffmpeg and ensure it is accessible in your PATH.",
            ),
            (
                FfError::ExecutionFailure("ffmpeg failed".into()),
                "Check ffmpeg output above for details. Input file may be corrupted or format may be invalid.",
            ),
            (
                FfError::Internal("panic".into()),
                "This is an unexpected bug. Please report it at https://github.com/darkterminal/ffrs/issues",
            ),
        ];

        for (error, expected) in cases {
            assert_eq!(error.guidance(), expected, "error: {error}");
        }
    }

    #[test]
    fn test_build_error_maps_to_user_input() {
        let error = FfError::from(BuildError::InvalidIntent("bad intent".to_string()));

        assert!(matches!(error, FfError::UserInput(_)));
        assert_eq!(error.exit_code(), 1);
    }

    #[test]
    fn test_display() {
        assert_eq!(
            format!("{}", FfError::UserInput("file not found".into())),
            "Input error: file not found"
        );
        assert_eq!(
            format!("{}", FfError::ExecutionFailure("not in PATH".into())),
            "Execution error: not in PATH"
        );
        assert_eq!(
            format!("{}", FfError::Internal("panic".into())),
            "Internal error: panic"
        );
    }
}
