use ff::{Tokenizer, Parser, CommandBuilder};

#[test]
fn test_quickstart_scenarios() {
    // Test scenario from quickstart guide: "convert video.mp4 to video.webm"
    let command = "convert video.mp4 to video.webm";

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
    assert_eq!(ffmpeg_cmd, "ffmpeg -i \"video.mp4\" \"video.webm\"");

    // Test another scenario: "convert myvideo.mp4 to myvideo.avi"
    let command2 = "convert myvideo.mp4 to myvideo.avi";

    let mut tokenizer2 = Tokenizer::new(command2);
    let tokens2 = tokenizer2.tokenize();

    let mut parser2 = Parser::new(tokens2);
    let intent2 = parser2.parse().expect("Parsing should succeed");

    let cmd_builder2 = CommandBuilder::new();
    let ffmpeg_cmd2 = cmd_builder2.build_command(&intent2).expect("Command building should succeed");

    assert_eq!(ffmpeg_cmd2, "ffmpeg -i \"myvideo.mp4\" \"myvideo.avi\"");

    // Test format conversion: "convert video.mp4 to .avi"
    let command3 = "convert video.mp4 to .avi";

    let mut tokenizer3 = Tokenizer::new(command3);
    let tokens3 = tokenizer3.tokenize();

    let mut parser3 = Parser::new(tokens3);
    let intent3 = parser3.parse().expect("Parsing should succeed");

    let cmd_builder3 = CommandBuilder::new();
    let ffmpeg_cmd3 = cmd_builder3.build_command(&intent3).expect("Command building should succeed");

    assert_eq!(ffmpeg_cmd3, "ffmpeg -i \"video.mp4\" \"video.avi\"");
}