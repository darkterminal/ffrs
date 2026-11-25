use std::process::Command;
use tempfile::tempdir;

fn ffrs() -> &'static str {
    env!("CARGO_BIN_EXE_ffrs")
}

#[test]
fn rejects_missing_output_directory() {
    let dir = tempdir().unwrap();
    std::fs::write(dir.path().join("input.mp4"), b"").unwrap();
    let missing = dir.path().join("missing");

    let output = Command::new(ffrs())
        .current_dir(dir.path())
        .args(["--dry-run", "--output"])
        .arg(&missing)
        .arg("convert input.mp4 to out.avi")
        .output()
        .expect("failed to run ffrs");

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Output directory does not exist"),
        "unexpected stderr: {stderr}"
    );
}

#[cfg(unix)]
#[test]
fn rejects_output_that_resolves_to_input() {
    let dir = tempdir().unwrap();
    let input = dir.path().join("video.mp4");
    std::fs::write(&input, b"").unwrap();

    let output = Command::new(ffrs())
        .args(["--dry-run", "--output"])
        .arg(dir.path())
        .arg(format!("convert {} to video.mp4", input.display()))
        .output()
        .expect("failed to run ffrs");

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("must differ from input path"),
        "unexpected stderr: {stderr}"
    );
}
