use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

use crate::error::Result;

const SD_CLI_BYTES: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/sd-cli"));

/// Extract the embedded sd-cli binary to the cache directory.
///
/// Re-extracts only when the file is missing or its size differs
/// from the embedded copy (cheap staleness check).
pub fn extract() -> Result<PathBuf> {
    extract_to(&cache_dir())
}

fn extract_to(dir: &Path) -> Result<PathBuf> {
    std::fs::create_dir_all(dir)?;
    let dest = dir.join("sd-cli");

    let needs_extract = match std::fs::metadata(&dest) {
        Ok(meta) => meta.len() != SD_CLI_BYTES.len() as u64,
        Err(_) => true,
    };

    if needs_extract {
        std::fs::write(&dest, SD_CLI_BYTES)?;
        std::fs::set_permissions(&dest, std::fs::Permissions::from_mode(0o755))?;
    }

    Ok(dest)
}

fn cache_dir() -> PathBuf {
    std::env::var("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            std::env::var("HOME")
                .map(|h| PathBuf::from(h).join(".cache"))
                .unwrap_or_else(|_| PathBuf::from(".cache"))
        })
        .join("sdx")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_binary_is_not_empty() {
        assert!(!SD_CLI_BYTES.is_empty());
    }

    #[test]
    fn extract_writes_executable() {
        let dir = std::env::temp_dir().join(format!("sd-embed-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);

        let path = extract_to(&dir).expect("extract succeeds");
        assert!(path.exists());

        let meta = std::fs::metadata(&path).expect("metadata");
        assert_eq!(meta.len(), SD_CLI_BYTES.len() as u64);
        assert_ne!(meta.permissions().mode() & 0o111, 0);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn extract_skips_when_unchanged() {
        let dir = std::env::temp_dir().join(format!("sd-embed-skip-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);

        let path1 = extract_to(&dir).expect("first extract");
        let mtime1 = std::fs::metadata(&path1)
            .expect("meta")
            .modified()
            .expect("mtime");

        std::thread::sleep(std::time::Duration::from_millis(50));

        let path2 = extract_to(&dir).expect("second extract");
        let mtime2 = std::fs::metadata(&path2)
            .expect("meta")
            .modified()
            .expect("mtime");

        assert_eq!(mtime1, mtime2, "file should not be rewritten");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
