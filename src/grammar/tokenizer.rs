
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Word(String),
    Path(String),
    Format(String),
    Number(f64),
    Unknown(String),
}

pub struct Tokenizer {
    text: String,
    position: usize,
}

impl Tokenizer {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            position: 0,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        
        while self.position < self.text.len() {
            if let Some(token) = self.next_token() {
                tokens.push(token);
            } else {
                // Skip one character if no token is matched
                self.position += 1;
            }
        }
        
        tokens
    }

    fn next_token(&mut self) -> Option<Token> {
        // Skip whitespace
        while self.position < self.text.len() &&
              self.text.chars().nth(self.position).unwrap().is_whitespace() {
            self.position += 1;
        }

        if self.position >= self.text.len() {
            return None;
        }

        // Check for file extensions (like .mp4, .avi, etc.) - this needs to be first
        if self.text.chars().nth(self.position).unwrap() == '.' {
            return Some(self.tokenize_format());
        }

        // Check for file paths (containing /)
        if self.text.chars().nth(self.position).unwrap() == '/' {
            return Some(self.tokenize_path());
        }

        // Check for numbers
        if self.text.chars().nth(self.position).unwrap().is_numeric() {
            return Some(self.tokenize_number());
        }

        // Tokenize words
        Some(self.tokenize_word())
    }

    fn tokenize_word(&mut self) -> Token {
        let start = self.position;
        while self.position < self.text.len() {
            let ch = self.text.chars().nth(self.position).unwrap();
            if ch.is_alphanumeric() || ch == '_' || ch == '.' {
                self.position += 1;
            } else {
                break;
            }
        }
        let word = self.text[start..self.position].to_string();

        // Check if the word looks like a file path (contains a dot followed by an extension)
        if word.contains('.') && !word.starts_with('.') {
            Token::Path(word)
        } else {
            Token::Word(word.to_lowercase())
        }
    }

    fn tokenize_path(&mut self) -> Token {
        let start = self.position;
        while self.position < self.text.len() {
            let ch = self.text.chars().nth(self.position).unwrap();
            if ch.is_alphanumeric() || ch == '/' || ch == '_' || ch == '-' || ch == '~' {
                self.position += 1;
            } else if ch == '.' && self.position + 1 < self.text.len() {
                // Check if this is a file extension (followed by alphanumeric characters)
                let next_chars = &self.text[self.position + 1..];
                if next_chars.chars().next().map_or(false, |c| c.is_alphanumeric()) {
                    self.position += 1;
                } else {
                    // If the dot is not followed by alphanumeric characters, it's not part of a path
                    break;
                }
            } else {
                break;
            }
        }
        let path = self.text[start..self.position].to_string();
        Token::Path(path)
    }

    fn tokenize_format(&mut self) -> Token {
        let start = self.position;
        // Skip the dot
        if self.text.chars().nth(self.position).unwrap() == '.' {
            self.position += 1;
        }

        while self.position < self.text.len() {
            let ch = self.text.chars().nth(self.position).unwrap();
            if ch.is_alphanumeric() {
                self.position += 1;
            } else {
                break;
            }
        }
        let format = self.text[start..self.position].to_string();
        Token::Format(format.to_lowercase())
    }

    fn tokenize_number(&mut self) -> Token {
        let start = self.position;
        let mut has_decimal = false;
        
        while self.position < self.text.len() {
            let ch = self.text.chars().nth(self.position).unwrap();
            if ch.is_numeric() {
                self.position += 1;
            } else if ch == '.' && !has_decimal {
                has_decimal = true;
                self.position += 1;
            } else {
                break;
            }
        }
        
        let number_str = self.text[start..self.position].to_string();
        if let Ok(number) = number_str.parse::<f64>() {
            Token::Number(number)
        } else {
            Token::Unknown(number_str)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_simple_command() {
        let mut tokenizer = Tokenizer::new("convert video.mp4 to video.avi");
        let tokens = tokenizer.tokenize();
        
        assert_eq!(tokens, vec![
            Token::Word("convert".to_string()),
            Token::Path("video.mp4".to_string()),
            Token::Word("to".to_string()),
            Token::Path("video.avi".to_string()),
        ]);
    }

    #[test]
    fn test_tokenize_with_format() {
        let mut tokenizer = Tokenizer::new("convert video to .avi");
        let tokens = tokenizer.tokenize();
        
        assert_eq!(tokens, vec![
            Token::Word("convert".to_string()),
            Token::Word("video".to_string()),
            Token::Word("to".to_string()),
            Token::Format(".avi".to_string()),
        ]);
    }
}