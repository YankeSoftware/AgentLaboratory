"""Directory management utilities for Agent Laboratory."""

import os
import shutil
import logging
from pathlib import Path
from typing import Optional, List, Dict

class DirectoryManager:
    """Handles directory operations with proper error handling and logging."""
    
    def __init__(self, base_dir: str = "/workspace"):
        """Initialize directory manager.
        
        Args:
            base_dir: Base directory for all operations
        """
        self.base_dir = Path(base_dir)
        self.required_dirs = {
            "research": self.base_dir / "research_dir",
            "source": self.base_dir / "research_dir" / "src",
            "tex": self.base_dir / "research_dir" / "tex",
            "output": self.base_dir / "output",
            "state": self.base_dir / "state_saves",
        }
        self.logger = logging.getLogger(__name__)

    def setup_directories(self) -> Dict[str, bool]:
        """Create all required directories.
        
        Returns:
            Dictionary of directory paths and their creation status
        """
        results = {}
        for name, path in self.required_dirs.items():
            try:
                path.mkdir(parents=True, exist_ok=True)
                # Ensure write permissions
                path.chmod(0o777)
                results[name] = True
                self.logger.info(f"Directory {path} created/verified successfully")
            except Exception as e:
                self.logger.error(f"Failed to create directory {path}: {str(e)}")
                results[name] = False
        return results

    def cleanup_directory(self, dir_type: str) -> bool:
        """Clean up specified directory type.
        
        Args:
            dir_type: Type of directory to clean ('research', 'source', 'tex', 'output', 'state')
            
        Returns:
            True if cleanup successful, False otherwise
        """
        if dir_type not in self.required_dirs:
            self.logger.error(f"Unknown directory type: {dir_type}")
            return False
            
        try:
            path = self.required_dirs[dir_type]
            if path.exists():
                # Remove contents but keep directory
                for item in path.iterdir():
                    if item.is_file():
                        item.unlink()
                    elif item.is_dir():
                        shutil.rmtree(item)
                self.logger.info(f"Cleaned directory {path}")
                return True
            return False
        except Exception as e:
            self.logger.error(f"Failed to clean directory {path}: {str(e)}")
            return False

    def get_path(self, dir_type: str) -> Optional[Path]:
        """Get path for specified directory type.
        
        Args:
            dir_type: Type of directory ('research', 'source', 'tex', 'output', 'state')
            
        Returns:
            Path object if valid, None if invalid
        """
        return self.required_dirs.get(dir_type)

    def ensure_file_directory(self, filepath: str) -> bool:
        """Ensure directory exists for given file path.
        
        Args:
            filepath: Path to file
            
        Returns:
            True if directory exists/created, False otherwise
        """
        try:
            directory = Path(filepath).parent
            directory.mkdir(parents=True, exist_ok=True)
            return True
        except Exception as e:
            self.logger.error(f"Failed to create directory for {filepath}: {str(e)}")
            return False

    def list_files(self, dir_type: str, pattern: str = "*") -> List[Path]:
        """List files in specified directory matching pattern.
        
        Args:
            dir_type: Type of directory
            pattern: Glob pattern for matching files
            
        Returns:
            List of matching file paths
        """
        if dir_type not in self.required_dirs:
            return []
            
        try:
            path = self.required_dirs[dir_type]
            return list(path.glob(pattern))
        except Exception as e:
            self.logger.error(f"Failed to list files in {path}: {str(e)}")
            return []

    def validate_permissions(self) -> Dict[str, bool]:
        """Validate permissions for all directories.
        
        Returns:
            Dictionary of directory paths and their permission status
        """
        results = {}
        for name, path in self.required_dirs.items():
            try:
                # Check if directory exists and is writable
                if not path.exists():
                    results[name] = False
                    continue
                    
                # Try to create a temporary file
                test_file = path / ".permission_test"
                test_file.touch()
                test_file.unlink()
                results[name] = True
            except Exception as e:
                self.logger.error(f"Permission validation failed for {path}: {str(e)}")
                results[name] = False
        return results