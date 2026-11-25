use std::process::Command;
use tempfile::tempdir;

fn ffrs() -> &'static str {
    env!("CARGO_BIN_EXE_ffrs")
}

#[test]
fn documented_extract_audio_syntax_builds_command() {
    let dir = tempdir().unwrap();
    std::fs::write(dir.path().join("video.mp4"), b"").unwrap();

    let output = Command::new(ffrs())
        .current_dir(dir.path())
        .args(["--dry-run", "extract audio from video.mp4 to audio.mp3"])
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
        stdout.contains("\"-q:a\" \"0\" \"-map\" \"a\""),
        "stdout: {stdout}"
    );
}

#[test]
fn short_extract_audio_syntax_builds_command() {
    let dir = tempdir().unwrap();
    std::fs::write(dir.path().join("video.mp4"), b"").unwrap();

    let output = Command::new(ffrs())
        .current_dir(dir.path())
        .args(["--dry-run", "extract video.mp4 to audio.mp3"])
        .output()
        .expect("failed to run ffrs");

    assert_eq!(
        output.status.code(),
        Some(0),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
