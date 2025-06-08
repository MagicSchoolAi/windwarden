use crate::{Result, WindWardenError};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Atomic file writer that ensures safe file operations
///
/// This prevents file corruption by writing to a temporary file first,
/// then atomically moving it to the target location.
pub struct AtomicWriter {
    target_path: PathBuf,
    temp_path: PathBuf,
    temp_file: Option<fs::File>,
}

impl AtomicWriter {
    /// Create a new atomic writer for the given file path
    pub fn new(target_path: impl AsRef<Path>) -> Result<Self> {
        let target_path = target_path.as_ref().to_path_buf();

        // Create temporary file path in the same directory as target
        // This ensures the atomic move works (same filesystem)
        let temp_path = Self::create_temp_path(&target_path)?;

        // Create parent directory if it doesn't exist
        if let Some(parent) = target_path.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                WindWardenError::from_io_error(e, Some(&parent.display().to_string()))
            })?;
        }

        // Create the temporary file
        let temp_file = fs::File::create(&temp_path).map_err(|e| {
            WindWardenError::from_io_error(e, Some(&temp_path.display().to_string()))
        })?;

        Ok(Self {
            target_path,
            temp_path,
            temp_file: Some(temp_file),
        })
    }

    /// Write content to the temporary file
    pub fn write(&mut self, content: &str) -> Result<()> {
        let file = self
            .temp_file
            .as_mut()
            .ok_or_else(|| WindWardenError::internal_error("AtomicWriter already finalized"))?;

        file.write_all(content.as_bytes()).map_err(|e| {
            WindWardenError::from_io_error(e, Some(&self.temp_path.display().to_string()))
        })?;

        file.flush().map_err(|e| {
            WindWardenError::from_io_error(e, Some(&self.temp_path.display().to_string()))
        })?;

        Ok(())
    }

    /// Commit the changes by atomically moving the temporary file to the target
    pub fn commit(mut self) -> Result<()> {
        // Ensure the file is closed before moving
        if let Some(file) = self.temp_file.take() {
            // Sync to disk to ensure all data is written
            file.sync_all().map_err(|e| {
                WindWardenError::from_io_error(e, Some(&self.temp_path.display().to_string()))
            })?;
            drop(file);
        }

        // Atomically move the temporary file to the target path
        fs::rename(&self.temp_path, &self.target_path).map_err(|e| {
            // Clean up temp file on failure
            let _ = fs::remove_file(&self.temp_path);
            WindWardenError::from_io_error(e, Some(&self.target_path.display().to_string()))
        })?;

        Ok(())
    }

    /// Create a temporary file path in the same directory as the target
    fn create_temp_path(target_path: &Path) -> Result<PathBuf> {
        let parent = target_path.parent().unwrap_or_else(|| Path::new("."));

        let file_name = target_path
            .file_name()
            .ok_or_else(|| WindWardenError::config_error("Invalid target file path"))?
            .to_string_lossy();

        // Use a random suffix to avoid conflicts
        let temp_name = format!(".{}.tmp.{}", file_name, generate_random_suffix());

        Ok(parent.join(temp_name))
    }
}

impl Drop for AtomicWriter {
    fn drop(&mut self) {
        // Clean up temporary file if commit wasn't called
        if self.temp_file.is_some() {
            let _ = fs::remove_file(&self.temp_path);
        }
    }
}

/// Generate a random suffix for temporary files
fn generate_random_suffix() -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::{SystemTime, UNIX_EPOCH};

    let mut hasher = DefaultHasher::new();

    // Use current time and process ID for randomness
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos()
        .hash(&mut hasher);

    std::process::id().hash(&mut hasher);

    hasher.finish()
}

/// Atomic file operations helper functions
pub mod operations {
    use super::*;

    /// Atomically write content to a file
    pub fn write_file(path: impl AsRef<Path>, content: &str) -> Result<()> {
        let mut writer = AtomicWriter::new(path)?;
        writer.write(content)?;
        writer.commit()?;
        Ok(())
    }

    /// Atomically write content to a file with backup
    pub fn write_file_with_backup(path: impl AsRef<Path>, content: &str) -> Result<()> {
        let path = path.as_ref();

        // Create backup if file exists
        if path.exists() {
            let backup_path = create_backup_path(path)?;
            fs::copy(path, &backup_path).map_err(|e| {
                WindWardenError::from_io_error(e, Some(&backup_path.display().to_string()))
            })?;
        }

        // Write the file atomically
        write_file(path, content)?;

        Ok(())
    }

    /// Create a backup path for a file
    fn create_backup_path(path: &Path) -> Result<PathBuf> {
        let file_name = path
            .file_name()
            .ok_or_else(|| WindWardenError::config_error("Invalid file path for backup"))?
            .to_string_lossy();

        let parent = path.parent().unwrap_or_else(|| Path::new("."));

        // Try different backup names until we find one that doesn't exist
        for i in 1..=999 {
            let backup_name = format!(
                "{}.bak{}",
                file_name,
                if i == 1 {
                    String::new()
                } else {
                    format!(".{}", i)
                }
            );
            let backup_path = parent.join(backup_name);

            if !backup_path.exists() {
                return Ok(backup_path);
            }
        }

        Err(WindWardenError::internal_error(
            "Unable to create backup file - too many existing backups",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_atomic_write_success() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");

        let mut writer = AtomicWriter::new(&file_path).unwrap();
        writer.write("Hello, World!").unwrap();
        writer.commit().unwrap();

        let content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "Hello, World!");
    }

    #[test]
    fn test_atomic_write_cleanup_on_drop() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");

        let temp_path = {
            let mut writer = AtomicWriter::new(&file_path).unwrap();
            writer.write("Hello, World!").unwrap();
            writer.temp_path.clone()
            // writer is dropped here without commit
        };

        // Original file should not exist
        assert!(!file_path.exists());

        // Temporary file should be cleaned up
        assert!(!temp_path.exists());
    }

    #[test]
    fn test_atomic_write_helper() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");

        operations::write_file(&file_path, "Test content").unwrap();

        let content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "Test content");
    }

    #[test]
    fn test_write_with_backup() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");

        // Create initial file
        fs::write(&file_path, "Original content").unwrap();

        // Write new content with backup
        operations::write_file_with_backup(&file_path, "New content").unwrap();

        // Check new content
        let content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "New content");

        // Check backup exists
        let backup_path = temp_dir.path().join("test.txt.bak");
        assert!(backup_path.exists());
        let backup_content = fs::read_to_string(&backup_path).unwrap();
        assert_eq!(backup_content, "Original content");
    }

    #[test]
    fn test_create_directories() {
        let temp_dir = TempDir::new().unwrap();
        let nested_path = temp_dir.path().join("nested").join("dirs").join("test.txt");

        operations::write_file(&nested_path, "Test content").unwrap();

        let content = fs::read_to_string(&nested_path).unwrap();
        assert_eq!(content, "Test content");
    }
}
