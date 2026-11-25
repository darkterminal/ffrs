use std::process::{Command, Stdio};

fn ffrs() -> &'static str {
    env!("CARGO_BIN_EXE_ffrs")
}

#[test]
fn version_flag_exits_zero() {
    let output = Command::new(ffrs())
        .arg("--version")
        .output()
        .expect("failed to run ffrs");

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(
        stdout.trim(),
        format!("ffrs {}", env!("CARGO_PKG_VERSION")),
        "unexpected stdout: {stdout}"
    );
}

#[test]
fn help_flag_exits_zero() {
    let output = Command::new(ffrs())
        .arg("--help")
        .output()
        .expect("failed to run ffrs");

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Usage"), "unexpected stdout: {stdout}");
}

#[test]
fn unknown_flag_exits_one() {
    let output = Command::new(ffrs())
        .arg("--definitely-not-a-flag")
        .output()
        .expect("failed to run ffrs");

    assert_eq!(
        output.status.code(),
        Some(1),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn missing_command_exits_one() {
    let output = Command::new(ffrs()).output().expect("failed to run ffrs");

    assert_eq!(output.status.code(), Some(1));
}

#[test]
fn interactive_mode_exits_on_eof() {
    let output = Command::new(ffrs())
        .arg("--interactive")
        .stdin(Stdio::null())
        .output()
        .expect("failed to run ffrs");

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.matches("> ").count(), 1, "stdout: {stdout}");
}
