use crate::utils::error::{AgentError, AgentResult};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
use tempfile::NamedTempFile;
use tracing::{debug, info, warn};

pub struct FileOps {
    test_mode: bool,
}

impl FileOps {
    /// Create a new FileOps instance
    /// 
    /// * `test_mode` - When true, uses more permissive file permissions and performs
    ///                 additional verifications for testing purposes
    pub fn new(test_mode: bool) -> Self {
        Self { test_mode }
    }
    
    /// Create a new FileOps instance for production use
    pub fn production() -> Self {
        Self::new(false)
    }

    /// Create a new FileOps instance for testing
    pub fn testing() -> Self {
        Self::new(true)
    }

    /// Returns whether this instance is in test mode
    pub fn is_test_mode(&self) -> bool {
        self.test_mode
    }

    /// Safely save content to a file with atomic operations and backup
    pub fn safe_save<P, C>(&self, content: C, path: P) -> AgentResult<()>
    where
        P: AsRef<Path>,
        C: AsRef<[u8]>,
    {
        let path = path.as_ref();
        debug!("Attempting to save file: {}", path.display());

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| AgentError::FileOp {
                path: parent.to_path_buf(),
                message: format!("Failed to create directory: {}", e),
            })?;
        }

        // Create backup if file exists
        if path.exists() {
            let backup_path = path.with_extension("bak");
            fs::copy(path, &backup_path).map_err(|e| AgentError::FileOp {
                path: backup_path,
                message: format!("Failed to create backup: {}", e),
            })?;
        }

        // Create temporary file in the same directory
        let dir = path.parent().unwrap_or_else(|| Path::new("."));
        let mut temp_file = NamedTempFile::new_in(dir).map_err(|e| AgentError::FileOp {
            path: dir.to_path_buf(),
            message: format!("Failed to create temp file: {}", e),
        })?;

        // Write content to temp file
        temp_file.write_all(content.as_ref()).map_err(|e| AgentError::FileOp {
            path: path.to_path_buf(),
            message: format!("Failed to write content: {}", e),
        })?;

        // Atomically rename temp file to target path
        temp_file.persist(path).map_err(|e| AgentError::FileOp {
            path: path.to_path_buf(),
            message: format!("Failed to persist file: {}", e),
        })?;

        info!("Successfully saved file: {}", path.display());
        Ok(())
    }

    /// Safely load file content with backup fallback
    pub fn safe_load<P>(&self, path: P) -> AgentResult<Vec<u8>>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        debug!("Attempting to load file: {}", path.display());

        let try_load = |p: &Path| -> io::Result<Vec<u8>> {
            let mut file = File::open(p)?;
            let mut content = Vec::new();
            file.read_to_end(&mut content)?;
            Ok(content)
        };

        // Try loading main file
        match try_load(path) {
            Ok(content) => {
                debug!("Successfully loaded file: {}", path.display());
                Ok(content)
            }
            Err(e) => {
                warn!("Failed to load main file: {}, trying backup", e);
                let backup_path = path.with_extension("bak");
                if backup_path.exists() {
                    try_load(&backup_path).map_err(|e| AgentError::FileOp {
                        path: backup_path,
                        message: format!("Failed to load backup: {}", e),
                    })
                } else {
                    Err(AgentError::FileOp {
                        path: path.to_path_buf(),
                        message: format!("No backup available: {}", e),
                    })
                }
            }
        }
    }

    /// Safely remove a file or directory with backup creation
    pub fn safe_remove<P>(&self, path: P) -> AgentResult<()>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        debug!("Attempting to remove: {}", path.display());

        if !path.exists() {
            return Ok(());
        }

        // Create backup
        let backup_path = path.with_extension("bak");
        if path.is_file() {
            fs::copy(path, &backup_path).map_err(|e| AgentError::FileOp {
                path: backup_path.clone(),
                message: format!("Failed to create backup: {}", e),
            })?;
            fs::remove_file(path)
        } else {
            fs::rename(path, &backup_path).map_err(|e| AgentError::FileOp {
                path: backup_path,
                message: format!("Failed to create backup: {}", e),
            })?;
            Ok(())
        }
        .map_err(|e| AgentError::FileOp {
            path: path.to_path_buf(),
            message: format!("Failed to remove: {}", e),
        })?;

        info!("Successfully removed: {}", path.display());
        Ok(())
    }

    /// Ensure directory exists with proper permissions
    pub fn ensure_directory<P>(&self, path: P) -> AgentResult<()>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        debug!("Ensuring directory exists: {}", path.display());

        if !path.exists() {
            fs::create_dir_all(path).map_err(|e| AgentError::FileOp {
                path: path.to_path_buf(),
                message: format!("Failed to create directory: {}", e),
            })?;

            // Set permissions based on mode
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mode = if self.test_mode {
                    0o777 // Full permissions for test directories
                } else {
                    0o755 // Standard permissions for production
                };
                fs::set_permissions(path, fs::Permissions::from_mode(mode)).map_err(|e| {
                    AgentError::FileOp {
                        path: path.to_path_buf(),
                        message: format!("Failed to set permissions: {}", e),
                    }
                })?;
            }
        }

        info!("Directory ready: {}", path.display());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_fs::prelude::*;
    use assert_fs::TempDir;


    fn fixture_err<E: std::fmt::Display>(e: E) -> AgentError {
        AgentError::Fixture(e.to_string())
    }

    #[test]
    fn test_safe_save_and_load() -> AgentResult<()> {
        let temp = TempDir::new().map_err(fixture_err)?;
        let file_ops = FileOps::new(true);
        let test_file = temp.child("test.txt");
        let content = b"Hello, World!";

        // Create initial file
        test_file.write_binary(content).map_err(fixture_err)?;
        assert!(test_file.path().exists());

        // Test save should create backup
        let new_content = b"Updated content";
        file_ops.safe_save(new_content, test_file.path())?;

        // Test load
        let loaded = file_ops.safe_load(test_file.path())?;
        assert_eq!(loaded, new_content);

        // Verify backup exists and contains original content
        let backup = test_file.path().with_extension("bak");
        assert!(backup.exists());
        let backup_content = fs::read(backup)?;
        assert_eq!(backup_content, content);

        Ok(())
    }

    #[test]
    fn test_safe_remove() -> AgentResult<()> {
        let temp = TempDir::new().map_err(fixture_err)?;
        let file_ops = FileOps::new(true);
        let test_file = temp.child("test.txt");
        test_file.write_str("test content").map_err(fixture_err)?;

        // Test remove
        file_ops.safe_remove(test_file.path())?;
        assert!(!test_file.path().exists());

        // Test backup exists
        let backup = test_file.path().with_extension("bak");
        assert!(backup.exists());

        Ok(())
    }

    #[test]
    fn test_directory_permissions() -> AgentResult<()> {
        let temp = TempDir::new().map_err(fixture_err)?;
        let file_ops = FileOps::new(true);
        let test_dir = temp.child("test_dir");

        // Test directory creation
        file_ops.ensure_directory(test_dir.path())?;
        assert!(test_dir.path().exists());
        assert!(test_dir.path().is_dir());

        // Test permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let metadata = fs::metadata(test_dir.path())?;
            assert_eq!(metadata.permissions().mode() & 0o777, 0o777);
        }

        Ok(())
    }
}