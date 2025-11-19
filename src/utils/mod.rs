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

