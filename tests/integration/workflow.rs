use ff::{Tokenizer, Parser, CommandBuilder, Runner};

#[test]
fn test_complete_workflow() {
    // Test a complete workflow from command to execution
    let command = "convert test_input.mp4 to test_output.avi";
    
    // Tokenize the command
    let mut tokenizer = Tokenizer::new(command);
    let tokens = tokenizer.tokenize();
    
    // Parse the tokens into an intent
    let mut parser = Parser::new(tokens);
    let intent = parser.parse().expect("Parsing should succeed");
    
    // Build the ffmpeg command
    let cmd_builder = CommandBuilder::new();
    let ffmpeg_cmd = cmd_builder.build_command(&intent).expect("Command building should succeed");
    
    // Verify the command structure
    assert!(ffmpeg_cmd.starts_with("ffmpeg -i"));
    assert!(ffmpeg_cmd.contains("test_input.mp4"));
    assert!(ffmpeg_cmd.contains("test_output.avi"));
    
    // Note: We're not actually executing the command in tests to avoid requiring ffmpeg
    // and creating actual files. The Runner functionality is tested separately.
    println!("Generated command: {}", ffmpeg_cmd);
}