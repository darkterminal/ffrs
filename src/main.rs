use clap::error::ErrorKind;
use clap::Parser as ClapParser;
use std::io;

mod command_builder;
mod errors;
mod executor;
mod grammar;
mod intent;
mod utils;

use command_builder::CommandBuilder;
use errors::FfError;
use executor::runner::Runner;
use grammar::{Parser as GrammarParser, Tokenizer};
use utils::file_utils;

#[derive(ClapParser)]
#[command(version)]
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
    let args = match Cli::try_parse() {
        Ok(args) => args,
        Err(error) => {
            let exit_code = match error.kind() {
                ErrorKind::DisplayHelp | ErrorKind::DisplayVersion => 0,
                _ => 1,
            };
            let _ = error.print();
            std::process::exit(exit_code);
        }
    };

    if args.interactive {
        let stdin = io::stdin();
        let mut stdout = io::stdout();
        if let Err(error) =
            run_interactive_mode(stdin.lock(), &mut stdout, args.dry_run, args.output)
        {
            eprintln!("Error reading input: {}", error);
            std::process::exit(1);
        }
    } else if let Some(command) = args.command {
        run_direct_mode(&command, args.dry_run, args.output);
    } else {
        eprintln!("Error: No command provided. Use --help for usage information.");
        std::process::exit(1);
    }
}

fn run_interactive_mode<R: io::BufRead, W: io::Write>(
    mut reader: R,
    writer: &mut W,
    dry_run: bool,
    output: Option<String>,
) -> io::Result<()> {
    writeln!(writer, "FF - Media Conversion Tool (Interactive Mode)")?;
    writeln!(writer, "Enter 'quit' or 'exit' to exit the program")?;

    loop {
        write!(writer, "> ")?;
        writer.flush()?;

        let mut input = String::new();
        let bytes_read = reader.read_line(&mut input)?;
        if bytes_read == 0 {
            break;
        }

        let input = input.trim();
        if input.eq_ignore_ascii_case("quit") || input.eq_ignore_ascii_case("exit") {
            break;
        }
        if !input.is_empty() {
            // clone() karena process_command consume Option<String>
            if let Err(error) = process_command(input, dry_run, output.clone()) {
                eprintln!("{}", error);
                print_guidance(&error);
            }
        }
    }

    Ok(())
}

fn run_direct_mode(command: &str, dry_run: bool, output: Option<String>) {
    match process_command(command, dry_run, output) {
        Ok(_) => std::process::exit(0),
        Err(e) => {
            eprintln!("{}", e);
            print_guidance(&e);
            std::process::exit(e.exit_code());
        }
    }
}

fn print_guidance(err: &FfError) {
    match err {
        FfError::UserInput(msg) => {
            if msg.contains("Output directory") {
                eprintln!("Guidance: Create the output directory first or choose an existing one.");
            } else if msg.contains("must differ from input") {
                eprintln!("Guidance: Choose a different output file name or format.");
            } else if msg.contains("must use a supported format") {
                eprintln!(
                    "Guidance: Use a supported output extension: MP4, AVI, MOV, WMV, MKV, WebM, MP3, WAV, FLAC, JPG, PNG, GIF"
                );
            } else if msg.contains("not exist") || msg.contains("not found") {
                eprintln!(
                    "Guidance: Make sure the input file path is correct and the file exists."
                );
            } else if msg.contains("Unsupported format") {
                eprintln!("Guidance: Check supported formats: MP4, AVI, MOV, WMV, MKV, WebM, MP3, WAV, FLAC, JPG, PNG, GIF");
            } else {
                eprintln!("Guidance: Make sure your command follows the format 'convert <input> to <output>' or similar.");
                eprintln!("Example: 'convert video.mp4 to video.avi'");
            }
        }
        FfError::ExecutionFailure(msg) => {
            if msg.contains("not available") {
                eprintln!("Guidance: Install ffmpeg and ensure it is accessible in your PATH.");
            } else {
                eprintln!("Guidance: Check ffmpeg output above for details. Input file may be corrupted or format may be invalid.");
            }
        }
        FfError::Internal(_) => {
            eprintln!("Guidance: This is an unexpected bug. Please report it at https://github.com/darkterminal/ffrs/issues");
        }
    }
}

fn process_command(command: &str, dry_run: bool, output: Option<String>) -> Result<(), FfError> {
    // Step 1: Tokenize & Parse
    let mut tokenizer = Tokenizer::new(command);
    let tokens = tokenizer.tokenize();

    let mut parser = GrammarParser::new(tokens);
    let intent = parser
        .parse()
        .map_err(|e| FfError::UserInput(e.to_string()))?;

    // Step 2: Fail Fast — Validate input file exists
    if !intent.input_path.exists() {
        return Err(FfError::UserInput(format!(
            "Input file does not exist: {}",
            intent.input_path.display()
        )));
    }
    if !intent.input_path.is_file() {
        return Err(FfError::UserInput(format!(
            "Input path is not a file: {}",
            intent.input_path.display()
        )));
    }

    // Step 3: Build ffmpeg command
    let cmd_builder = CommandBuilder::new();

    let ffmpeg_cmd = if let Some(output_dir) = output {
        let output_dir = std::path::PathBuf::from(&output_dir);
        if !output_dir.is_dir() {
            return Err(FfError::UserInput(format!(
                "Output directory does not exist or is not a directory: {}",
                output_dir.display()
            )));
        }

        let output_path = output_dir.join(
            intent
                .output_path
                .file_name()
                .ok_or_else(|| FfError::UserInput("Invalid output path".to_string()))?,
        );

        if file_utils::same_file(&intent.input_path, &output_path) {
            return Err(FfError::UserInput(format!(
                "Output path must differ from input path: {}",
                output_path.display()
            )));
        }

        cmd_builder
            .build_command_with_output_path(&intent, output_path)
            .map_err(|e| FfError::UserInput(e.to_string()))?
    } else {
        cmd_builder
            .build_command(&intent)
            .map_err(|e| FfError::UserInput(e.to_string()))?
    };

    // Step 4: Print the command (constitution requirement)
    println!("{}", ffmpeg_cmd.to_display_string());

    // Step 5: Execute (unless dry-run)
    if !dry_run {
        let runner = Runner::new();
        runner
            .execute(&ffmpeg_cmd)
            .map_err(|e| FfError::ExecutionFailure(e.to_string()))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn run_interactive(input: &str) -> (io::Result<()>, String) {
        let mut output = Vec::new();
        let result = run_interactive_mode(Cursor::new(input), &mut output, true, None);
        let output = String::from_utf8(output).expect("interactive output should be UTF-8");
        (result, output)
    }

    #[test]
    fn interactive_mode_stops_at_eof() {
        let (result, output) = run_interactive("");

        assert!(result.is_ok());
        assert_eq!(output.matches("> ").count(), 1, "output: {output}");
    }

    #[test]
    fn interactive_mode_stops_after_quit() {
        let (result, output) = run_interactive("quit\n");

        assert!(result.is_ok());
        assert_eq!(output.matches("> ").count(), 1, "output: {output}");
    }

    #[test]
    fn interactive_mode_skips_blank_lines_before_eof() {
        let (result, output) = run_interactive("\n\n");

        assert!(result.is_ok());
        assert_eq!(output.matches("> ").count(), 3, "output: {output}");
    }

    #[test]
    fn interactive_mode_propagates_read_errors() {
        struct FailingReader;

        impl io::Read for FailingReader {
            fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
                Err(io::Error::other("boom"))
            }
        }

        impl io::BufRead for FailingReader {
            fn fill_buf(&mut self) -> io::Result<&[u8]> {
                Err(io::Error::other("boom"))
            }

            fn consume(&mut self, _amount: usize) {}
        }

        let mut output = Vec::new();
        let result = run_interactive_mode(FailingReader, &mut output, true, None);

        assert!(result.is_err());
    }
}
