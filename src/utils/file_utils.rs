use std::path::{Component, Path, PathBuf};

/// Checks if the given file path has a supported media format.
///
/// # Arguments
///
/// * `file_path` - The file path to check
///
/// # Returns
///
/// `true` if the file format is supported, `false` otherwise.
///
/// # Examples
///
/// ```
/// use ffrs::utils::file_utils::is_supported_format;
/// assert!(is_supported_format("video.mp4"));
/// assert!(!is_supported_format("document.txt"));
/// ```
pub fn is_supported_format<P: AsRef<Path>>(file_path: P) -> bool {
    let file_path = file_path.as_ref();
    if let Some(ext) = file_path.extension() {
        let ext_lower = ext.to_string_lossy().to_lowercase();
        matches!(
            ext_lower.as_str(),
            "mp4"
                | "avi"
                | "mov"
                | "wmv"
                | "mkv"
                | "webm"
                | "mp3"
                | "wav"
                | "flac"
                | "jpg"
                | "png"
                | "gif"
        )
    } else {
        false
    }
}

/// Returns `true` when both paths resolve to the same file.
///
/// Existing paths are compared with [`std::fs::canonicalize`], which also
/// catches symlinks. When a path does not exist yet (the common case for an
/// output file), both paths are normalized lexically against the current
/// directory.
///
/// # Examples
///
/// ```
/// use ffrs::utils::file_utils::same_file;
/// assert!(same_file("video.mp4", "./video.mp4"));
/// assert!(!same_file("video.mp4", "video.avi"));
/// ```
pub fn same_file<P: AsRef<Path>, Q: AsRef<Path>>(a: P, b: Q) -> bool {
    let a = a.as_ref();
    let b = b.as_ref();

    if let (Ok(canonical_a), Ok(canonical_b)) = (std::fs::canonicalize(a), std::fs::canonicalize(b))
    {
        return canonical_a == canonical_b;
    }

    normalize_lexical(a) == normalize_lexical(b)
}

fn normalize_lexical(path: &Path) -> PathBuf {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .map(|cwd| cwd.join(path))
            .unwrap_or_else(|_| path.to_path_buf())
    };

    let mut normalized = PathBuf::new();
    for component in absolute.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                normalized.pop();
            }
            other => normalized.push(other.as_os_str()),
        }
    }
    normalized
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

    #[test]
    fn test_same_file_lexical() {
        assert!(same_file("video.mp4", "./video.mp4"));
        assert!(same_file("media/../video.mp4", "video.mp4"));
        assert!(!same_file("video.mp4", "video.avi"));
    }

    #[test]
    fn test_same_file_existing_path() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("clip.mp4");
        std::fs::write(&file, b"fixture").unwrap();
        let alias = dir.path().join(".").join("clip.mp4");

        assert!(same_file(&file, &alias));
    }
}
