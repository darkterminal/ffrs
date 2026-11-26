use std::process::Command;
use tempfile::tempdir;

fn ffrs() -> &'static str {
    env!("CARGO_BIN_EXE_ffrs")
}

fn dry_run(args: &[&str]) -> (i32, String) {
    let output = Command::new(ffrs())
        .args(args)
        .output()
        .expect("failed to run ffrs");
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    (output.status.code().unwrap_or(-1), stdout)
}

#[test]
fn default_dry_run_uses_no_overwrite() {
    let dir = tempdir().unwrap();
    std::fs::write(dir.path().join("clip.mp4"), b"").unwrap();

    let command = format!(
        "convert {} to out.avi",
        dir.path().join("clip.mp4").display()
    );
    let (code, stdout) = dry_run(&["--dry-run", &command]);

    assert_eq!(code, 0, "stdout: {stdout}");
    assert!(stdout.contains("\"-n\" \"-i\""), "stdout: {stdout}");
    assert!(!stdout.contains("\"-y\""), "stdout: {stdout}");
}

#[test]
fn yes_flag_dry_run_uses_overwrite() {
    let dir = tempdir().unwrap();
    std::fs::write(dir.path().join("clip.mp4"), b"").unwrap();

    let command = format!(
        "convert {} to out.avi",
        dir.path().join("clip.mp4").display()
    );
    let (code, stdout) = dry_run(&["--dry-run", "--yes", &command]);

    assert_eq!(code, 0, "stdout: {stdout}");
    assert!(stdout.contains("\"-y\" \"-i\""), "stdout: {stdout}");
    assert!(!stdout.contains("\"-n\""), "stdout: {stdout}");
}
