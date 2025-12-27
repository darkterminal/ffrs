use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub enum OperationType {
    Convert,
    Resize,
    Transcode,
    ExtractAudio,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Intent {
    pub operation: OperationType,
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub parameters: std::collections::HashMap<String, String>,
}