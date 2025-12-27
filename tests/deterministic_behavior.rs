use ff::{Tokenizer, Parser, CommandBuilder};

#[test]
fn test_deterministic_behavior() {
    // Test that identical input produces identical output
    let input_command = "convert video.mp4 to video.avi";

    // Process the command multiple times
    let result1 = process_command(input_command);
    let result2 = process_command(input_command);
    let result3 = process_command(input_command);

    // All results should be identical
    assert_eq!(result1, result2);
    assert_eq!(result2, result3);

    // Verify the command structure is as expected
    assert!(result1.starts_with("ffmpeg -i"));
    assert!(result1.contains("video.mp4"));
    assert!(result1.contains("video.avi"));
}

fn process_command(command: &str) -> String {
    // Tokenize the command
    let mut tokenizer = Tokenizer::new(command);
    let tokens = tokenizer.tokenize();

    // Parse the tokens into an intent
    let mut parser = Parser::new(tokens);
    let intent = parser.parse().expect("Parsing should succeed");

    // Build the ffmpeg command
    let cmd_builder = CommandBuilder::new();
    cmd_builder.build_command(&intent).expect("Command building should succeed")
}