use ff::{Tokenizer, Parser, CommandBuilder};

#[test]
fn test_command_generation_snapshot() {
    let test_cases = vec![
        ("convert video.mp4 to video.avi", "ffmpeg -i \"video.mp4\" \"video.avi\""),
        ("convert input.mov to output.mp4", "ffmpeg -i \"input.mov\" \"output.mp4\""),
        ("convert audio.wav to audio.mp3", "ffmpeg -i \"audio.wav\" \"audio.mp3\""),
    ];

    for (input, expected_output) in test_cases {
        // Tokenize the command
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize();
        
        // Parse the tokens into an intent
        let mut parser = Parser::new(tokens);
        let intent = parser.parse().expect("Parsing should succeed");
        
        // Build the ffmpeg command
        let cmd_builder = CommandBuilder::new();
        let result = cmd_builder.build_command(&intent).expect("Command building should succeed");
        
        assert_eq!(result, expected_output);
    }
}

// Additional snapshot test with different operations
#[test]
fn test_command_generation_snapshot_extended() {
    // This would typically use a snapshot testing library to automatically update and compare snapshots
    // For now, we'll use hardcoded expected values
    
    let input = "convert video.mp4 to .avi";
    
    // Tokenize the command
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize();
    
    // Parse the tokens into an intent
    let mut parser = Parser::new(tokens);
    let intent = parser.parse().expect("Parsing should succeed");
    
    // Build the ffmpeg command
    let cmd_builder = CommandBuilder::new();
    let result = cmd_builder.build_command(&intent).expect("Command building should succeed");
    
    assert_eq!(result, "ffmpeg -i \"video.mp4\" \"video.avi\"");
}