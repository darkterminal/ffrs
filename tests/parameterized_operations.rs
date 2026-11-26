use std::path::Path;
use std::process::Command;
use tempfile::{tempdir, TempDir};

fn ffrs() -> &'static str {
    env!("CARGO_BIN_EXE_ffrs")
}

fn fixture() -> TempDir {
    let dir = tempdir().unwrap();
    std::fs::write(dir.path().join("clip.mp4"), b"").unwrap();
    dir
}

fn dry_run(dir: &Path, command: &str) -> (i32, String, String) {
    let output = Command::new(ffrs())
        .current_dir(dir)
        .args(["--dry-run", command])
        .output()
        .expect("failed to run ffrs");
    (
        output.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&output.stdout).into_owned(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
    )
}

#[test]
fn resize_dimensions_reach_ffmpeg_args() {
    let dir = fixture();
    let (code, stdout, stderr) = dry_run(dir.path(), "resize clip.mp4 to 1280x720 out.mp4");

    assert_eq!(code, 0, "stdout: {stdout}, stderr: {stderr}");
    assert!(
        stdout.contains("\"-vf\" \"scale=1280:720\""),
        "stdout: {stdout}"
    );
}

#[test]
fn transcode_video_codec_reaches_ffmpeg_args() {
    let dir = fixture();
    let (code, stdout, stderr) = dry_run(dir.path(), "transcode clip.mp4 to h265 out.mp4");

    assert_eq!(code, 0, "stdout: {stdout}, stderr: {stderr}");
    assert!(stdout.contains("\"-c:v\" \"h265\""), "stdout: {stdout}");
}

#[test]
fn transcode_audio_codec_reaches_ffmpeg_args() {
    let dir = fixture();
    let (code, stdout, stderr) = dry_run(dir.path(), "transcode clip.mp4 to opus out.mp4");

    assert_eq!(code, 0, "stdout: {stdout}, stderr: {stderr}");
    assert!(stdout.contains("\"-c:a\" \"opus\""), "stdout: {stdout}");
}

#[test]
fn omitted_parameters_keep_documented_defaults() {
    let dir = fixture();
    let (code, stdout, stderr) = dry_run(dir.path(), "resize clip.mp4 to out.mp4");

    assert_eq!(code, 0, "stdout: {stdout}, stderr: {stderr}");
    assert!(
        stdout.contains("\"-vf\" \"scale=1920:1080\""),
        "stdout: {stdout}"
    );
}

#[test]
fn malformed_dimensions_fail_fast() {
    let dir = fixture();
    let (code, _, stderr) = dry_run(dir.path(), "resize clip.mp4 to 1280x out.mp4");

    assert_eq!(code, 1, "stderr: {stderr}");
    assert!(stderr.contains("Invalid parameter"), "stderr: {stderr}");
}

#[test]
fn unknown_codec_fails_fast() {
    let dir = fixture();
    let (code, _, stderr) = dry_run(dir.path(), "transcode clip.mp4 to h266 out.mp4");

    assert_eq!(code, 1, "stderr: {stderr}");
    assert!(stderr.contains("unknown codec"), "stderr: {stderr}");
}
