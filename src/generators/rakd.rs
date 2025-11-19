use anyhow::{Context, Result};
use std::path::PathBuf;
use std::fs;
use crate::state::ProjectState;
use crate::utils;

pub fn generate(state: &ProjectState) -> Result<PathBuf> {
    let doplan_dir = utils::doplan_dir()?;
    let rakd_path = doplan_dir.join("RAKD.md");

    let project_root = utils::project_root()?;
    let mut required_keys = Vec::new();
    let mut validated_keys = Vec::new();

    // Detect required API keys from services
    if let Some(tech_stack) = &state.tech_stack {
        for tech in tech_stack {
            if let Some(keys) = detect_api_keys(tech) {
                for key in keys {
                    if !required_keys.contains(&key) {
                        required_keys.push(key);
                    }
                }
            }
        }
    }

    // Check .env files for keys
    let env_files = vec![".env", ".env.local", ".env.production"];
    for env_file in env_files {
        let env_path = project_root.join(env_file);
        if env_path.exists() {
            if let Ok(content) = fs::read_to_string(&env_path) {
                for line in content.lines() {
                    if line.contains('=') && !line.trim_start().starts_with('#') {
                        if let Some((key, _)) = line.split_once('=') {
                            let key = key.trim();
                            if required_keys.iter().any(|rk| key.contains(rk)) {
                                validated_keys.push(key.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    // Generate RAKD
    let mut content = String::new();
    content.push_str("# Required API Keys Document (RAKD)\n\n");
    content.push_str(&format!("**Project:** {}\n\n", 
        state.project_name.as_ref().unwrap_or(&"Untitled Project".to_string())));
    content.push_str(&format!("**Date:** {}\n\n", chrono::Utc::now().format("%Y-%m-%d")));
    content.push_str("---\n\n");

    content.push_str("## Overview\n\n");
    content.push_str("This document lists all required API keys and their validation status.\n\n");
    content.push_str("---\n\n");

    if required_keys.is_empty() {
        content.push_str("## Status\n\n");
        content.push_str("✅ No API keys required for this project.\n\n");
    } else {
        content.push_str("## Required API Keys\n\n");
        for key in &required_keys {
            let status = if validated_keys.iter().any(|vk| vk.contains(key)) {
                "✅ Configured"
            } else {
                "❌ Missing"
            };
            content.push_str(&format!("- **{}**: {}\n", key, status));
        }
        content.push_str("\n");

        content.push_str("## Configuration\n\n");
        content.push_str("Add the following to your `.env` file:\n\n");
        content.push_str("```env\n");
        for key in &required_keys {
            content.push_str(&format!("{}=\n", key.to_uppercase().replace(" ", "_")));
        }
        content.push_str("```\n\n");
    }

    content.push_str("## Security Notes\n\n");
    content.push_str("- Never commit API keys to version control\n");
    content.push_str("- Use environment variables for all keys\n");
    content.push_str("- Rotate keys regularly\n");
    content.push_str("- Use different keys for development and production\n\n");

    std::fs::write(&rakd_path, content)
        .context("Failed to write RAKD")?;

    Ok(rakd_path)
}

fn detect_api_keys(tech: &str) -> Option<Vec<String>> {
    let tech_lower = tech.to_lowercase();
    let mut keys = Vec::new();

    if tech_lower.contains("aws") {
        keys.push("AWS_ACCESS_KEY_ID".to_string());
        keys.push("AWS_SECRET_ACCESS_KEY".to_string());
    }
    if tech_lower.contains("mongodb") {
        keys.push("MONGODB_URI".to_string());
    }
    if tech_lower.contains("postgres") {
        keys.push("DATABASE_URL".to_string());
    }
    if tech_lower.contains("redis") {
        keys.push("REDIS_URL".to_string());
    }
    if tech_lower.contains("vercel") {
        keys.push("VERCEL_TOKEN".to_string());
    }

    if keys.is_empty() {
        None
    } else {
        Some(keys)
    }
}

