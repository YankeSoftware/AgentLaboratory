use crate::utils::error::AgentResult;
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{info, warn};

pub struct InitManager {
    root_dir: PathBuf,
}

impl InitManager {
    pub fn new<P: AsRef<Path>>(root_dir: P) -> Self {
        Self {
            root_dir: root_dir.as_ref().to_path_buf(),
        }
    }

    pub fn ensure_directories(&self) -> AgentResult<()> {
        // Create main directories
        let dirs = vec![
            "output",
            "research_dir",
            "research_dir/src",
            "research_dir/tex",
            "state_saves",
            "logs",
            "experiments",
            "temp",
        ];

        for dir in dirs {
            let path = self.root_dir.join(dir);
            if !path.exists() {
                info!("Creating directory: {}", path.display());
                fs::create_dir_all(&path)?;
                
                // Set permissions - more permissive for test environments
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    fs::set_permissions(&path, fs::Permissions::from_mode(0o777))?;
                }
            }
        }

        Ok(())
    }

    pub fn cleanup_temp(&self) -> AgentResult<()> {
        let temp_dir = self.root_dir.join("temp");
        if temp_dir.exists() {
            info!("Cleaning temporary directory");
            fs::remove_dir_all(&temp_dir)?;
            fs::create_dir(&temp_dir)?;
        }
        Ok(())
    }

    pub fn get_path(&self, subpath: &str) -> PathBuf {
        self.root_dir.join(subpath)
    }

    pub fn verify_llm_credentials(&self) -> bool {
        // Check for DeepSeek API key
        if let Ok(key) = std::env::var("DEEPSEEK_API_KEY") {
            if !key.is_empty() {
                info!("Found DeepSeek API key");
                return true;
            }
        }

        // Check for Claude/Anthropic API key
        if let Ok(key) = std::env::var("ANTHROPIC_API_KEY") {
            if !key.is_empty() {
                info!("Found Anthropic/Claude API key");
                return true;
            }
        }

        warn!("No valid LLM API keys found. Please set DEEPSEEK_API_KEY or ANTHROPIC_API_KEY");
        false
    }
}