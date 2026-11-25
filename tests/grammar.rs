use ffrs::grammar::{ParseError, Token};
use ffrs::intent::types::OperationType;
use ffrs::{Parser, Tokenizer};
use std::path::PathBuf;

#[test]
fn test_tokenizer_basic_command() {
    let mut tokenizer = Tokenizer::new("convert video.mp4 to video.avi");
    let tokens = tokenizer.tokenize();

    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0], Token::Word("convert".to_string()));
    assert_eq!(tokens[1], Token::Path("video.mp4".to_string()));
    assert_eq!(tokens[2], Token::Word("to".to_string()));
    assert_eq!(tokens[3], Token::Path("video.avi".to_string()));
}

#[test]
fn test_parser_basic_convert() {
    let tokens = vec![
        Token::Word("convert".to_string()),
        Token::Path("input.mp4".to_string()),
        Token::Word("to".to_string()),
        Token::Path("output.avi".to_string()),
    ];

    let mut parser = Parser::new(tokens);
    let intent = parser.parse().unwrap();

    assert_eq!(intent.operation, OperationType::Convert);
    assert_eq!(intent.input_path, PathBuf::from("input.mp4"));
    assert_eq!(intent.output_path, PathBuf::from("output.avi"));
}

#[test]
fn test_parser_with_format() {
    let tokens = vec![
        Token::Word("convert".to_string()),
        Token::Path("input.mp4".to_string()),
        Token::Word("to".to_string()),
        Token::Format(".avi".to_string()),
    ];

    let mut parser = Parser::new(tokens);
    let intent = parser.parse().unwrap();

    assert_eq!(intent.operation, OperationType::Convert);
    assert_eq!(intent.input_path, PathBuf::from("input.mp4"));
    assert_eq!(intent.output_path, PathBuf::from("input.avi"));
}

#[test]
fn test_parser_error_on_invalid_operation() {
    let tokens = vec![
        Token::Word("invalid_operation".to_string()),
        Token::Path("input.mp4".to_string()),
        Token::Word("to".to_string()),
        Token::Path("output.avi".to_string()),
    ];

    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        ParseError::UnexpectedToken(_)
    ));
}

fn parse(command: &str) -> Result<ffrs::Intent, ParseError> {
    let mut tokenizer = Tokenizer::new(command);
    let tokens = tokenizer.tokenize();
    Parser::new(tokens).parse()
}

#[test]
fn test_unknown_operation_is_rejected() {
    let error = parse("destroy video.mp4 to out.avi").unwrap_err();

    assert!(matches!(error, ParseError::UnexpectedToken(_)));
}

#[test]
fn test_unsupported_input_format_is_rejected() {
    let error = parse("convert video.xyz to out.mp4").unwrap_err();

    assert!(matches!(error, ParseError::UnsupportedFormat(_)));
}

#[test]
fn test_missing_to_is_rejected() {
    let error = parse("convert video.mp4 out.avi").unwrap_err();

    assert!(matches!(error, ParseError::UnexpectedToken(_)));
}
