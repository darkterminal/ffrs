/// Token types for the FF CLI tool.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// A regular word token
    Word(String),
    /// A file path token
    Path(String),
    /// A format token (e.g., .mp4, .avi)
    Format(String),
    /// A numeric value
    Number(f64),
    /// An unknown token type
    Unknown(String),
}

/// Tokenizer for converting plain English commands into tokens.
///
/// Input is split on whitespace, except inside single or double quotes.
/// Each raw token is then classified by its shape: an explicit format
/// (`.avi`), a path (contains a separator or a file extension), a number,
/// or a word.
pub struct Tokenizer {
    chars: Vec<char>,
    position: usize,
}

#[derive(Debug)]
struct RawToken {
    text: String,
    quoted: bool,
}

impl Tokenizer {
    /// Creates a new tokenizer for the given text.
    ///
    /// # Arguments
    ///
    /// * `text` - The input text to tokenize
    ///
    /// # Examples
    ///
    /// ```
    /// use ffrs::Tokenizer;
    /// let mut tokenizer = Tokenizer::new("convert video.mp4 to video.avi");
    /// let tokens = tokenizer.tokenize();
    /// ```
    pub fn new(text: &str) -> Self {
        Self {
            chars: text.chars().collect(),
            position: 0,
        }
    }

    /// Tokenizes the input text into a vector of tokens.
    ///
    /// # Returns
    ///
    /// A vector of `Token` enums representing the parsed tokens.
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(raw) = self.next_raw_token() {
            tokens.push(classify_token(raw));
        }

        tokens
    }

    /// Peeks at the current character without advancing.
    #[inline]
    fn peek(&self) -> Option<char> {
        self.chars.get(self.position).copied()
    }

    /// Extracts a string slice from `start` to `end` (char indices).
    #[inline]
    fn slice(&self, start: usize, end: usize) -> String {
        self.chars[start..end].iter().collect()
    }

    /// Reads one whitespace-delimited token, honoring quotes.
    ///
    /// Every call advances `position` when it returns a token, so an
    /// unexpected character can never stall tokenization.
    fn next_raw_token(&mut self) -> Option<RawToken> {
        while matches!(self.peek(), Some(ch) if ch.is_whitespace()) {
            self.position += 1;
        }

        if self.position >= self.chars.len() {
            return None;
        }

        if matches!(self.peek(), Some('"' | '\'')) {
            let quote = self.peek()?;
            self.position += 1;

            let start = self.position;
            while let Some(ch) = self.peek() {
                if ch == quote {
                    break;
                }
                self.position += 1;
            }

            let text = self.slice(start, self.position);
            if self.peek() == Some(quote) {
                self.position += 1;
            }

            return Some(RawToken { text, quoted: true });
        }

        let start = self.position;
        while matches!(self.peek(), Some(ch) if !ch.is_whitespace()) {
            self.position += 1;
        }

        Some(RawToken {
            text: self.slice(start, self.position),
            quoted: false,
        })
    }
}

fn classify_token(raw: RawToken) -> Token {
    let text = raw.text;

    if is_format_literal(&text) {
        return Token::Format(text.to_lowercase());
    }

    if is_path_literal(&text, raw.quoted) {
        return Token::Path(text);
    }

    let is_number = if text.contains('.') {
        text.chars().all(|ch| ch.is_ascii_digit() || ch == '.')
    } else {
        text.chars().all(|ch| ch.is_ascii_digit())
    };

    if is_number {
        return match text.parse::<f64>() {
            Ok(number) => Token::Number(number),
            Err(_) => Token::Unknown(text),
        };
    }

    Token::Word(text.to_lowercase())
}

fn is_format_literal(text: &str) -> bool {
    text.starts_with('.') && text.len() > 1 && text[1..].chars().all(|ch| ch.is_alphanumeric())
}

fn is_path_literal(text: &str, quoted: bool) -> bool {
    if text.is_empty() {
        return false;
    }

    if text.contains('/') || text.contains('\\') {
        return true;
    }

    if text.starts_with('.') {
        return false;
    }

    if quoted {
        return text.contains('.');
    }

    text.contains('.')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_simple_command() {
        let mut tokenizer = Tokenizer::new("convert video.mp4 to video.avi");
        let tokens = tokenizer.tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::Word("convert".to_string()),
                Token::Path("video.mp4".to_string()),
                Token::Word("to".to_string()),
                Token::Path("video.avi".to_string()),
            ]
        );
    }

    #[test]
    fn test_tokenize_with_format() {
        let mut tokenizer = Tokenizer::new("convert video to .avi");
        let tokens = tokenizer.tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::Word("convert".to_string()),
                Token::Word("video".to_string()),
                Token::Word("to".to_string()),
                Token::Format(".avi".to_string()),
            ]
        );
    }

    #[test]
    fn test_tokenize_path_with_directories() {
        let mut tokenizer = Tokenizer::new("convert /home/user/video.mp4 to output.avi");
        let tokens = tokenizer.tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::Word("convert".to_string()),
                Token::Path("/home/user/video.mp4".to_string()),
                Token::Word("to".to_string()),
                Token::Path("output.avi".to_string()),
            ]
        );
    }

    #[test]
    fn test_tokenize_relative_paths() {
        let mut tokenizer = Tokenizer::new("convert sub/clip.mp4 to ../out.avi");
        let tokens = tokenizer.tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::Word("convert".to_string()),
                Token::Path("sub/clip.mp4".to_string()),
                Token::Word("to".to_string()),
                Token::Path("../out.avi".to_string()),
            ]
        );
    }

    #[test]
    fn test_tokenize_dot_relative_paths() {
        let mut tokenizer = Tokenizer::new("convert ./clip.mp4 to ./out.avi");
        let tokens = tokenizer.tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::Word("convert".to_string()),
                Token::Path("./clip.mp4".to_string()),
                Token::Word("to".to_string()),
                Token::Path("./out.avi".to_string()),
            ]
        );
    }

    #[test]
    fn test_tokenize_quoted_paths() {
        let mut tokenizer = Tokenizer::new("convert \"my video.mp4\" to 'my video.avi'");
        let tokens = tokenizer.tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::Word("convert".to_string()),
                Token::Path("my video.mp4".to_string()),
                Token::Word("to".to_string()),
                Token::Path("my video.avi".to_string()),
            ]
        );
    }

    #[test]
    fn test_tokenize_windows_paths() {
        let mut tokenizer = Tokenizer::new("convert C:\\Users\\me\\clip.mp4 to D:\\out\\clip.avi");
        let tokens = tokenizer.tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::Word("convert".to_string()),
                Token::Path("C:\\Users\\me\\clip.mp4".to_string()),
                Token::Word("to".to_string()),
                Token::Path("D:\\out\\clip.avi".to_string()),
            ]
        );
    }

    #[test]
    fn test_tokenize_unc_path() {
        let mut tokenizer = Tokenizer::new("convert \\\\server\\share\\clip.mp4 to out.avi");
        let tokens = tokenizer.tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::Word("convert".to_string()),
                Token::Path("\\\\server\\share\\clip.mp4".to_string()),
                Token::Word("to".to_string()),
                Token::Path("out.avi".to_string()),
            ]
        );
    }

    #[test]
    fn test_keywords_are_case_insensitive() {
        let mut tokenizer = Tokenizer::new("CONVERT clip.mp4 TO out.avi");
        let tokens = tokenizer.tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::Word("convert".to_string()),
                Token::Path("clip.mp4".to_string()),
                Token::Word("to".to_string()),
                Token::Path("out.avi".to_string()),
            ]
        );
    }

    #[test]
    fn test_unexpected_character_still_advances() {
        let mut tokenizer = Tokenizer::new("! convert clip.mp4");
        let tokens = tokenizer.tokenize();
        assert_eq!(
            tokens,
            vec![
                Token::Word("!".to_string()),
                Token::Word("convert".to_string()),
                Token::Path("clip.mp4".to_string()),
            ]
        );
    }

    #[test]
    fn test_tokenize_empty_input() {
        let mut tokenizer = Tokenizer::new("");
        let tokens = tokenizer.tokenize();
        assert_eq!(tokens, vec![]);
    }

    #[test]
    fn test_tokenize_whitespace_only() {
        let mut tokenizer = Tokenizer::new("   ");
        let tokens = tokenizer.tokenize();
        assert_eq!(tokens, vec![]);
    }
}
