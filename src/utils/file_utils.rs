use std::path::Path;

/// Checks if the file format is supported
pub fn is_supported_format<P: AsRef<Path>>(file_path: P) -> bool {
    let file_path = file_path.as_ref();
    if let Some(ext) = file_path.extension() {
        let ext_lower = ext.to_string_lossy().to_lowercase();
        matches!(ext_lower.as_str(),
            "mp4" | "avi" | "mov" | "wmv" | "mkv" | "webm" |
            "mp3" | "wav" | "flac" | "jpg" | "png" | "gif"
        )
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_supported_format() {
        assert!(is_supported_format("test.mp4"));
        assert!(is_supported_format("test.avi"));
        assert!(!is_supported_format("test.txt"));
    }
}