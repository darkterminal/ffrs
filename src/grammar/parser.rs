use crate::intent::types::{Intent, OperationType};
use std::path::PathBuf;
use crate::grammar::tokenizer::Token;
use crate::utils::file_utils;

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(String),
    MissingToken(String),
    InvalidPath(String),
    UnsupportedFormat(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedToken(token) => write!(f, "Unexpected token: {}", token),
            ParseError::MissingToken(expected) => write!(f, "Missing expected token: {}", expected),
            ParseError::InvalidPath(path) => write!(f, "Invalid path: {}", path),
            ParseError::UnsupportedFormat(format) => write!(f, "Unsupported format: {}", format),
        }
    }
}

impl std::error::Error for ParseError {}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, position: 0 }
    }

    pub fn parse(&mut self) -> Result<Intent, ParseError> {
        // Expect a command word like "convert", "transcode", etc.
        let operation = self.parse_operation()?;

        // Parse input path
        let input_path = self.parse_path()?;

        // Validate input file format
        if !file_utils::is_supported_format(&input_path) {
            return Err(ParseError::UnsupportedFormat(input_path.clone()));
        }

        // Expect "to" keyword
        self.expect_word("to")?;

        // Parse output path or format
        let output_path = self.parse_output_path_or_format(&input_path)?;

        // Validate output file format (only if it's a file path with extension)
        if output_path.contains('.') && !file_utils::is_supported_format(&output_path) {
            return Err(ParseError::UnsupportedFormat(output_path.clone()));
        }

        Ok(Intent {
            operation,
            input_path: PathBuf::from(input_path),
            output_path: PathBuf::from(output_path),
            parameters: std::collections::HashMap::new(),
        })
    }

    fn parse_operation(&mut self) -> Result<OperationType, ParseError> {
        if self.position >= self.tokens.len() {
            return Err(ParseError::MissingToken("operation".to_string()));
        }

        match &self.tokens[self.position] {
            Token::Word(word) => {
                self.position += 1;
                
                match word.as_str() {
                    "convert" => Ok(OperationType::Convert),
                    "resize" => Ok(OperationType::Resize),
                    "transcode" => Ok(OperationType::Transcode),
                    "extract" | "extractaudio" => Ok(OperationType::ExtractAudio),
                    _ => Err(ParseError::UnexpectedToken(format!("Unknown operation: {}", word))),
                }
            },
            _ => Err(ParseError::UnexpectedToken("Expected operation word".to_string())),
        }
    }

    fn parse_path(&mut self) -> Result<String, ParseError> {
        if self.position >= self.tokens.len() {
            return Err(ParseError::MissingToken("path".to_string()));
        }

        match &self.tokens[self.position] {
            Token::Path(path) => {
                self.position += 1;
                Ok(path.clone())
            },
            Token::Word(word) => {
                // Check if it looks like a path (contains extension)
                if word.contains('.') {
                    self.position += 1;
                    Ok(word.clone())
                } else {
                    Err(ParseError::UnexpectedToken(format!("Expected path, got: {}", word)))
                }
            },
            _ => Err(ParseError::UnexpectedToken("Expected path".to_string())),
        }
    }

    fn expect_word(&mut self, expected: &str) -> Result<(), ParseError> {
        if self.position >= self.tokens.len() {
            return Err(ParseError::MissingToken(expected.to_string()));
        }

        match &self.tokens[self.position] {
            Token::Word(word) => {
                if word == expected {
                    self.position += 1;
                    Ok(())
                } else {
                    Err(ParseError::UnexpectedToken(format!("Expected '{}', got: {}", expected, word)))
                }
            },
            _ => Err(ParseError::UnexpectedToken(format!("Expected '{}', got different token type", expected))),
        }
    }

    fn parse_output_path_or_format(&mut self, input_path: &str) -> Result<String, ParseError> {
        if self.position >= self.tokens.len() {
            return Err(ParseError::MissingToken("output path or format".to_string()));
        }

        match &self.tokens[self.position] {
            Token::Path(path) => {
                self.position += 1;
                Ok(path.clone())
            },
            Token::Format(format) => {
                // Replace the extension in the input path with the new format
                self.position += 1;
                
                // Get the directory and base name from the input path
                let input_path_buf = PathBuf::from(input_path);
                let dir = input_path_buf.parent().unwrap_or_else(|| std::path::Path::new(""));
                let base_name = input_path_buf.file_stem()
                    .ok_or_else(|| ParseError::InvalidPath(input_path.to_string()))?;
                
                // Create new path with the new extension
                let dir_str = dir.to_string_lossy();
                let base_name_str = base_name.to_string_lossy();
                let format_str = format.trim_start_matches('.');

                let new_path = if dir_str.is_empty() {
                    format!("{}.{}", base_name_str, format_str)
                } else {
                    format!("{}/{}.{}", dir_str, base_name_str, format_str)
                };
                
                Ok(new_path)
            },
            Token::Word(word) => {
                // Check if it looks like a path (contains extension)
                if word.contains('.') {
                    self.position += 1;
                    Ok(word.clone())
                } else {
                    Err(ParseError::UnexpectedToken(format!("Expected path or format, got: {}", word)))
                }
            },
            _ => Err(ParseError::UnexpectedToken("Expected output path or format".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grammar::tokenizer::{Tokenizer};

    #[test]
    fn test_parse_convert_command() {
        let mut tokenizer = Tokenizer::new("convert video.mp4 to video.avi");
        let tokens = tokenizer.tokenize();
        let mut parser = Parser::new(tokens);
        let intent = parser.parse().unwrap();

        assert_eq!(intent.operation, OperationType::Convert);
        assert_eq!(intent.input_path, PathBuf::from("video.mp4"));
        assert_eq!(intent.output_path, PathBuf::from("video.avi"));
    }

    #[test]
    fn test_parse_convert_with_format() {
        let mut tokenizer = Tokenizer::new("convert video.mp4 to .avi");
        let tokens = tokenizer.tokenize();
        let mut parser = Parser::new(tokens);
        let intent = parser.parse().unwrap();

        assert_eq!(intent.operation, OperationType::Convert);
        assert_eq!(intent.input_path, PathBuf::from("video.mp4"));
        assert_eq!(intent.output_path, PathBuf::from("video.avi"));
    }
}