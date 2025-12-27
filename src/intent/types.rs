use std::path::PathBuf;

/// Types of operations that can be performed by the FF CLI tool.
#[derive(Debug, Clone, PartialEq)]
pub enum OperationType {
    /// Convert a media file from one format to another
    Convert,
    /// Resize a media file to specific dimensions
    Resize,
    /// Transcode a media file with specific codecs
    Transcode,
    /// Extract audio from a media file
    ExtractAudio,
}

/// Represents a user's intent to perform a media conversion operation.
#[derive(Debug, Clone, PartialEq)]
pub struct Intent {
    /// The type of operation to perform
    pub operation: OperationType,
    /// The path to the input media file
    pub input_path: PathBuf,
    /// The path where the output media file should be saved
    pub output_path: PathBuf,
    /// Additional parameters for the operation
    pub parameters: std::collections::HashMap<String, String>,
}