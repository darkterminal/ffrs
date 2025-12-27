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
        self.check_ffmpeg_availability()?;

        let parts: Vec<&str> = cmd.split_whitespace().collect();

        if parts.is_empty() {
            return Err(ExecutionError::InvalidCommand("Command is empty".to_string()));
        }

        let program = parts[0];
        let args: Vec<&str> = parts[1..].to_vec();

        let output = Command::new(program)
            .args(&args)
            .output()
            .map_err(|e| ExecutionError::CommandFailed(format!("Failed to execute command: {}", e)))?;

        if output.status.success() {
            if !output.stdout.is_empty() {
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }
            Ok(())
        } else {
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