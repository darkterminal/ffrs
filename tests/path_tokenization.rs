use std::process::Command;
use tempfile::tempdir;

fn ffrs() -> &'static str {
    env!("CARGO_BIN_EXE_ffrs")
}

#[test]
fn accepts_relative_path_with_directory() {
    let dir = tempdir().unwrap();
    std::fs::create_dir(dir.path().join("sub")).unwrap();
    std::fs::write(dir.path().join("sub/clip.mp4"), b"").unwrap();

    let output = Command::new(ffrs())
        .current_dir(dir.path())
        .args(["--dry-run", "convert sub/clip.mp4 to out.avi"])
        .output()
        .expect("failed to run ffrs");

    assert_eq!(
        output.status.code(),
        Some(0),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("ffmpeg \"-i\" \"sub/clip.mp4\" \"out.avi\""),
        "stdout: {stdout}"
    );
}

#[test]
fn accepts_dot_relative_path() {
    let dir = tempdir().unwrap();
    std::fs::write(dir.path().join("clip.mp4"), b"").unwrap();

    let output = Command::new(ffrs())
        .current_dir(dir.path())
        .args(["--dry-run", "convert ./clip.mp4 to out.avi"])
        .output()
        .expect("failed to run ffrs");

    assert_eq!(
        output.status.code(),
        Some(0),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn accepts_quoted_path_with_spaces() {
    let dir = tempdir().unwrap();
    std::fs::write(dir.path().join("my video.mp4"), b"").unwrap();

    let output = Command::new(ffrs())
        .current_dir(dir.path())
        .args(["--dry-run", "convert \"my video.mp4\" to \"my video.avi\""])
        .output()
        .expect("failed to run ffrs");

    assert_eq!(
        output.status.code(),
        Some(0),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"my video.mp4\""), "stdout: {stdout}");
}
