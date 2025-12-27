use std::process::Command;

#[derive(Debug)]
pub struct Runner;

#[derive(Debug)]
pub enum ExecutionError {
    CommandFailed(String),
    InvalidCommand(String),
}

impl std::fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExecutionError::CommandFailed(msg) => write!(f, "Command failed: {}", msg),
            ExecutionError::InvalidCommand(cmd) => write!(f, "Invalid command: {}", cmd),
        }
    }
}

impl std::error::Error for ExecutionError {}

impl Runner {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(&self, cmd: &str) -> Result<(), ExecutionError> {
        // Check if ffmpeg is available
        self.check_ffmpeg_availability()?;

        // Parse the command string into program and arguments
        let parts: Vec<&str> = cmd.split_whitespace().collect();

        if parts.is_empty() {
            return Err(ExecutionError::InvalidCommand("Command is empty".to_string()));
        }

        let program = parts[0];
        let args: Vec<&str> = parts[1..].to_vec();

        // Execute the command
        let output = Command::new(program)
            .args(&args)
            .output()
            .map_err(|e| ExecutionError::CommandFailed(format!("Failed to execute command: {}", e)))?;

        // Check if the command was successful
        if output.status.success() {
            // Print stdout if there's any
            if !output.stdout.is_empty() {
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
            Ok(())
        } else {
            // Print stderr if there's any
            if !output.stderr.is_empty() {
                eprintln!("{}", String::from_utf8_lossy(&output.stderr));
            }
            Err(ExecutionError::CommandFailed(format!(
                "Command exited with status: {}",
                output.status
            )))
        }
    }

    fn check_ffmpeg_availability(&self) -> Result<(), ExecutionError> {
        match Command::new("ffmpeg")
            .arg("-version")
            .output() {
            Ok(_) => Ok(()),
            Err(_) => Err(ExecutionError::CommandFailed("ffmpeg is not available in PATH".to_string())),
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runner_creation() {
        let runner = Runner::new();
        assert_eq!(format!("{:?}", runner), "Runner");
    }
}