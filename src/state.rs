use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use anyhow::{Context, Result};
use crate::utils;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectState {
    pub project_name: Option<String>,
    pub idea: Option<String>,
    pub tech_stack: Option<Vec<String>>,
    pub features: Option<Vec<Feature>>,
    pub phases: Option<Vec<Phase>>,
    pub improvements: Option<Vec<String>>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feature {
    pub name: String,
    pub description: String,
    pub priority: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Phase {
    pub name: String,
    pub description: String,
    pub features: Vec<String>,
}

impl ProjectState {
    pub fn new() -> Self {
        Self {
            project_name: None,
            idea: None,
            tech_stack: None,
            features: None,
            phases: None,
            improvements: None,
            notes: None,
        }
    }

    pub fn load() -> Result<Self> {
        let state_path = Self::state_path()?;
        
        if !state_path.exists() {
            return Ok(Self::new());
        }

        let content = std::fs::read_to_string(&state_path)
            .context("Failed to read state file")?;
        
        let state: ProjectState = serde_json::from_str(&content)
            .context("Failed to parse state file")?;
        
        Ok(state)
    }

    pub fn save(&self) -> Result<()> {
        let state_path = Self::state_path()?;
        utils::ensure_dir(state_path.parent().unwrap())?;
        
        let content = serde_json::to_string_pretty(self)
            .context("Failed to serialize state")?;
        
        std::fs::write(&state_path, content)
            .context("Failed to write state file")?;
        
        Ok(())
    }

    fn state_path() -> Result<PathBuf> {
        let dot_doplan = utils::dot_doplan_dir()?;
        Ok(dot_doplan.join("state.json"))
    }
}

