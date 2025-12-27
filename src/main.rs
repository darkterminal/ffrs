use clap::Parser as ClapParser;
use std::io::{self, Write};

mod grammar;
mod intent;
mod command_builder;
mod executor;
mod utils;

use grammar::{Tokenizer, Parser as GrammarParser};
use command_builder::CommandBuilder;
use executor::runner::Runner;

#[derive(ClapParser)]
#[command(name = "ff")]
#[command(about = "A CLI tool that translates plain English commands into ffmpeg commands")]
struct Cli {
    #[arg(value_parser)]
    command: Option<String>,

    #[arg(short, long, default_value_t = false)]
    interactive: bool,

    #[arg(long, default_value_t = false)]
    dry_run: bool,

    #[arg(long)]
    output: Option<String>,
}

fn main() {
    let args = Cli::parse();

    if args.interactive {
        run_interactive_mode(args.dry_run);
    } else if let Some(command) = args.command {
        run_direct_mode(&command, args.dry_run, args.output);
    } else {
        eprintln!("Error: No command provided. Use --help for usage information.");
        std::process::exit(1);
    }
}

fn run_interactive_mode(dry_run: bool) {
    println!("FF - Media Conversion Tool (Interactive Mode)");
    println!("Enter 'quit' or 'exit' to exit the program");

    loop {
        print!("> ");
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed immediately

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();

                if input.eq_ignore_ascii_case("quit") || input.eq_ignore_ascii_case("exit") {
                    break;
                }

                if !input.is_empty() {
                    match process_command(input, dry_run, None) {
                        Ok(_) => {},
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
            Err(error) => {
                eprintln!("Error reading input: {}", error);
                break;
            }
        }
    }
}

fn run_direct_mode(command: &str, dry_run: bool, output: Option<String>) {
    match process_command(command, dry_run, output) {
        Ok(_) => std::process::exit(0), // Success
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1); // User input error
        }
    }
}

fn process_command(command: &str, dry_run: bool, output: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let mut tokenizer = Tokenizer::new(command);
    let tokens = tokenizer.tokenize();

    let mut parser = GrammarParser::new(tokens);
    let intent = match parser.parse() {
        Ok(intent) => intent,
        Err(e) => {
            eprintln!("Parse Error: {}", e);
            eprintln!("Guidance: Make sure your command follows the format 'convert <input> to <output>' or similar.");
            eprintln!("Example: 'convert video.mp4 to video.avi'");
            return Err(Box::new(e));
        }
    };

    let cmd_builder = CommandBuilder::new();
    let ffmpeg_cmd = match cmd_builder.build_command(&intent) {
        Ok(cmd) => cmd,
        Err(e) => {
            eprintln!("Command Build Error: {}", e);
            eprintln!("Guidance: Check that your input and output paths are valid and formats are supported.");
            return Err(e);
        }
    };

    let final_cmd = if let Some(output_dir) = output {
        let output_path = std::path::PathBuf::from(&output_dir)
            .join(intent.output_path.file_name().ok_or("Invalid output path")?);

        match cmd_builder.build_command_with_output_path(&intent, output_path) {
            Ok(cmd) => cmd,
            Err(e) => {
                eprintln!("Command Build Error: {}", e);
                eprintln!("Guidance: Check that your output directory is valid and writable.");
                return Err(e);
            }
        }
    } else {
        ffmpeg_cmd
    };

    println!("{}", final_cmd);

    if !dry_run {
        let runner = Runner::new();
        match runner.execute(&final_cmd) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Execution Error: {}", e);
                eprintln!("Guidance: Make sure ffmpeg is installed and accessible in your PATH.");
                return Err(Box::new(e));
            }
        }
    }

    Ok(())
}
