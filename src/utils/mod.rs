use std::path::{Path, PathBuf};
use anyhow::{Context, Result};

pub fn ensure_dir(path: &Path) -> Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path)
            .with_context(|| format!("Failed to create directory: {}", path.display()))?;
    }
    Ok(())
}

pub fn project_root() -> Result<PathBuf> {
    let current_dir = std::env::current_dir()
        .context("Failed to get current directory. Ensure you're in a valid project directory")?;
    Ok(current_dir)
}

pub fn doplan_dir() -> Result<PathBuf> {
    let root = project_root()?;
    Ok(root.join("doplan"))
}

pub fn dot_doplan_dir() -> Result<PathBuf> {
    let root = project_root()?;
    Ok(root.join(".doplan"))
}

pub fn ai_commands_dir() -> Result<PathBuf> {
    let dot_doplan = dot_doplan_dir()?;
    Ok(dot_doplan.join("ai").join("commands"))
}

/// Verify that a file was written successfully by checking its existence and size
pub fn verify_file_write(path: &Path, min_size: usize) -> Result<()> {
    if !path.exists() {
        anyhow::bail!("File was not created: {}", path.display());
    }
    
    let metadata = std::fs::metadata(path)
        .with_context(|| format!("Failed to read metadata for: {}", path.display()))?;
    
    if metadata.len() < min_size as u64 {
        anyhow::bail!(
            "File is too small ({} bytes, expected at least {} bytes): {}",
            metadata.len(),
            min_size,
            path.display()
        );
    }
    
    Ok(())
}

/// Validate that a path is safe to write to (not a directory, parent exists)
pub fn validate_write_path(path: &Path) -> Result<()> {
    if path.exists() && path.is_dir() {
        anyhow::bail!("Path is a directory, cannot write file: {}", path.display());
    }
    
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            anyhow::bail!("Parent directory does not exist: {}", parent.display());
        }
    }
    
    Ok(())
}

/// Validate that content is not empty
pub fn validate_content(content: &str, min_length: usize) -> Result<()> {
    if content.trim().len() < min_length {
        anyhow::bail!(
            "Content is too short ({} chars, expected at least {} chars)",
            content.len(),
            min_length
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_ensure_dir() {
        let temp_dir = TempDir::new().unwrap();
        let test_path = temp_dir.path().join("test_dir");
        
        assert!(!test_path.exists());
        ensure_dir(&test_path).unwrap();
        assert!(test_path.exists());
        assert!(test_path.is_dir());
    }

    #[test]
    fn test_ensure_dir_nested() {
        let temp_dir = TempDir::new().unwrap();
        let test_path = temp_dir.path().join("a").join("b").join("c");
        
        assert!(!test_path.exists());
        ensure_dir(&test_path).unwrap();
        assert!(test_path.exists());
        assert!(test_path.is_dir());
    }

    #[test]
    fn test_verify_file_write() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test.txt");
        
        // File doesn't exist
        assert!(verify_file_write(&test_file, 10).is_err());
        
        // Create file with content
        std::fs::write(&test_file, "Hello, World!").unwrap();
        
        // Should pass with min_size 10
        assert!(verify_file_write(&test_file, 10).is_ok());
        
        // Should fail with min_size 100
        assert!(verify_file_write(&test_file, 100).is_err());
    }

    #[test]
    fn test_validate_write_path() {
        let temp_dir = TempDir::new().unwrap();
        let parent_dir = temp_dir.path().join("parent");
        std::fs::create_dir(&parent_dir).unwrap();
        
        let test_file = parent_dir.join("test.txt");
        
        // Should pass - parent exists
        assert!(validate_write_path(&test_file).is_ok());
        
        // Should fail - parent doesn't exist
        let bad_file = temp_dir.path().join("nonexistent").join("test.txt");
        assert!(validate_write_path(&bad_file).is_err());
        
        // Should fail - path is a directory
        assert!(validate_write_path(&parent_dir).is_err());
    }

    #[test]
    fn test_validate_content() {
        // Valid content
        assert!(validate_content("This is a long enough string", 10).is_ok());
        
        // Too short
        assert!(validate_content("Short", 10).is_err());
        
        // Empty string
        assert!(validate_content("", 10).is_err());
        
        // Whitespace only
        assert!(validate_content("   ", 10).is_err());
        
        // Exactly min_length
        assert!(validate_content("1234567890", 10).is_ok());
    }
}
