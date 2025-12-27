use crate::intent::types::{Intent, OperationType};
use std::path::PathBuf;

/// Command builder for converting intents into ffmpeg commands.
#[derive(Debug)]
pub struct CommandBuilder;

impl CommandBuilder {
    /// Creates a new command builder.
    pub fn new() -> Self {
        Self
    }

    /// Builds an ffmpeg command from the given intent.
    ///
    /// # Arguments
    ///
    /// * `intent` - The intent to convert to a command
    ///
    /// # Returns
    ///
    /// A `Result` containing either the ffmpeg command string or an error.
    pub fn build_command(&self, intent: &Intent) -> Result<String, Box<dyn std::error::Error>> {
        self.build_command_with_output_path(intent, intent.output_path.clone())
    }

    /// Builds an ffmpeg command with a specific output path.
    ///
    /// # Arguments
    ///
    /// * `intent` - The intent to convert to a command
    /// * `output_path` - The specific output path to use
    ///
    /// # Returns
    ///
    /// A `Result` containing either the ffmpeg command string or an error.
    pub fn build_command_with_output_path(&self, intent: &Intent, output_path: PathBuf) -> Result<String, Box<dyn std::error::Error>> {
        let input_path = intent.input_path.to_string_lossy();
        let output_path = output_path.to_string_lossy();

        let cmd = match &intent.operation {
            OperationType::Convert => {
                format!("ffmpeg -i \"{}\" \"{}\"", input_path, output_path)
            },
            OperationType::Resize => {
                let width = intent.parameters.get("width").unwrap_or(&"1920".to_string()).clone();
                let height = intent.parameters.get("height").unwrap_or(&"1080".to_string()).clone();
                format!("ffmpeg -i \"{}\" -vf scale={}:{} \"{}\"", input_path, width, height, output_path)
            },
            OperationType::Transcode => {
                let video_codec = intent.parameters.get("vcodec").unwrap_or(&"libx264".to_string()).clone();
                let audio_codec = intent.parameters.get("acodec").unwrap_or(&"aac".to_string()).clone();
                format!("ffmpeg -i \"{}\" -c:v {} -c:a {} \"{}\"", input_path, video_codec, audio_codec, output_path)
            },
            OperationType::ExtractAudio => {
                format!("ffmpeg -i \"{}\" -q:a 0 -map a \"{}\"", input_path, output_path)
            },
        };

        Ok(cmd)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intent::types::{Intent, OperationType};
    use std::path::PathBuf;

    #[test]
    fn test_build_convert_command() {
        let builder = CommandBuilder::new();
        let intent = Intent {
            operation: OperationType::Convert,
            input_path: PathBuf::from("input.mp4"),
            output_path: PathBuf::from("output.avi"),
            parameters: std::collections::HashMap::new(),
        };

        let cmd = builder.build_command(&intent).unwrap();
        assert_eq!(cmd, "ffmpeg -i \"input.mp4\" \"output.avi\"");
    }

    #[test]
    fn test_build_resize_command() {
        let builder = CommandBuilder::new();
        let mut params = std::collections::HashMap::new();
        params.insert("width".to_string(), "1280".to_string());
        params.insert("height".to_string(), "720".to_string());
        
        let intent = Intent {
            operation: OperationType::Resize,
            input_path: PathBuf::from("input.mp4"),
            output_path: PathBuf::from("output.mp4"),
            parameters: params,
        };

        let cmd = builder.build_command(&intent).unwrap();
        assert_eq!(cmd, "ffmpeg -i \"input.mp4\" -vf scale=1280:720 \"output.mp4\"");
    }
}