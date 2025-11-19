use std::path::{Path, PathBuf};
use anyhow::Result;

pub fn ensure_dir(path: &Path) -> Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}

pub fn project_root() -> Result<PathBuf> {
    let current_dir = std::env::current_dir()?;
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

