use anyhow::{Context, Result};
use std::path::PathBuf;
use std::fs;
use crate::state::ProjectState;
use crate::utils;

pub fn generate(state: &ProjectState) -> Result<Vec<PathBuf>> {
    // Validate state
    if state.project_name.is_none() {
        anyhow::bail!("Project state is incomplete: missing project_name. Run /discuss first.");
    }

    let doplan_dir = utils::doplan_dir()
        .context("Failed to get doplan directory")?;
    let sops_dir = doplan_dir.join("SOPS");
    utils::ensure_dir(&sops_dir)
        .context("Failed to create SOPS directory")?;

    let mut generated = Vec::new();
    let mut services = Vec::new();

    // Detect services from tech stack
    if let Some(tech_stack) = &state.tech_stack {
        for tech in tech_stack {
            if let Some(service) = detect_service(tech) {
                if !services.contains(&service) {
                    services.push(service);
                }
            }
        }
    }

    // Detect from package.json if exists
    let project_root = utils::project_root()?;
    if let Ok(content) = fs::read_to_string(project_root.join("package.json")) {
        if let Ok(pkg) = serde_json::from_str::<serde_json::Value>(&content) {
            if let Some(deps) = pkg.get("dependencies").and_then(|d| d.as_object()) {
                for dep in deps.keys() {
                    if let Some(service) = detect_service_from_dependency(dep) {
                        if !services.contains(&service) {
                            services.push(service);
                        }
                    }
                }
            }
        }
    }

    // Detect from Cargo.toml if exists
    if let Ok(content) = fs::read_to_string(project_root.join("Cargo.toml")) {
        for line in content.lines() {
            if line.contains('=') && !line.trim_start().starts_with('#') {
                if let Some(service) = detect_service_from_cargo_dep(line) {
                    if !services.contains(&service) {
                        services.push(service);
                    }
                }
            }
        }
    }

    // Generate SOPS for each service
    for service in services {
        let (category, name) = service;
        let category_dir = sops_dir.join(&category);
        utils::ensure_dir(&category_dir)
            .with_context(|| format!("Failed to create SOPS category directory: {}", category))?;

        let sops_path = category_dir.join(format!("{}.md", name));
        utils::validate_write_path(&sops_path)
            .with_context(|| format!("Invalid path for SOPS file: {}", sops_path.display()))?;
        generate_service_sops(&sops_path, &category, &name)?;
        utils::verify_file_write(&sops_path, 100)
            .with_context(|| format!("SOPS file verification failed: {}", sops_path.display()))?;
        generated.push(sops_path);
    }

    Ok(generated)
}

fn detect_service(tech: &str) -> Option<(String, String)> {
    let tech_lower = tech.to_lowercase();
    
    if tech_lower.contains("postgresql") || tech_lower.contains("postgres") {
        Some(("database".to_string(), "postgresql".to_string()))
    } else if tech_lower.contains("mongodb") {
        Some(("database".to_string(), "mongodb".to_string()))
    } else if tech_lower.contains("redis") {
        Some(("cache".to_string(), "redis".to_string()))
    } else if tech_lower.contains("docker") {
        Some(("deployment".to_string(), "docker".to_string()))
    } else if tech_lower.contains("vercel") {
        Some(("deployment".to_string(), "vercel".to_string()))
    } else if tech_lower.contains("railway") {
        Some(("deployment".to_string(), "railway".to_string()))
    } else {
        None
    }
}

fn detect_service_from_dependency(dep: &str) -> Option<(String, String)> {
    let dep_lower = dep.to_lowercase();
    
    if dep_lower.contains("postgres") || dep_lower.contains("pg") {
        Some(("database".to_string(), "postgresql".to_string()))
    } else if dep_lower.contains("mongodb") || dep_lower.contains("mongoose") {
        Some(("database".to_string(), "mongodb".to_string()))
    } else if dep_lower.contains("redis") {
        Some(("cache".to_string(), "redis".to_string()))
    } else if dep_lower.contains("aws") {
        Some(("cloud".to_string(), "aws".to_string()))
    } else if dep_lower.contains("azure") {
        Some(("cloud".to_string(), "azure".to_string()))
    } else {
        None
    }
}

fn detect_service_from_cargo_dep(line: &str) -> Option<(String, String)> {
    let line_lower = line.to_lowercase();
    
    if line_lower.contains("postgres") || line_lower.contains("sqlx") {
        Some(("database".to_string(), "postgresql".to_string()))
    } else if line_lower.contains("redis") {
        Some(("cache".to_string(), "redis".to_string()))
    } else {
        None
    }
}

fn generate_service_sops(path: &PathBuf, category: &str, service: &str) -> Result<()> {
    let mut content = String::new();
    content.push_str("# Service Operating Procedures\n\n");
    content.push_str(&format!("**Service:** {}\n\n", service));
    content.push_str(&format!("**Category:** {}\n\n", category));
    content.push_str(&format!("**Date:** {}\n\n", chrono::Utc::now().format("%Y-%m-%d")));
    content.push_str("---\n\n");

    content.push_str("## Overview\n\n");
    content.push_str(&format!("Operating procedures for {} service.\n\n", service));

    content.push_str("## Setup\n\n");
    content.push_str("### Installation\n");
    content.push_str("_Installation instructions_\n\n");

    content.push_str("### Configuration\n");
    content.push_str("_Configuration steps_\n\n");

    content.push_str("## Usage\n\n");
    content.push_str("### Basic Operations\n");
    content.push_str("_Basic usage examples_\n\n");

    content.push_str("### Advanced Operations\n");
    content.push_str("_Advanced usage examples_\n\n");

    content.push_str("## Troubleshooting\n\n");
    content.push_str("### Common Issues\n");
    content.push_str("_Common issues and solutions_\n\n");

    content.push_str("## Best Practices\n\n");
    content.push_str("- Follow service-specific best practices\n");
    content.push_str("- Monitor performance and errors\n");
    content.push_str("- Keep dependencies updated\n\n");

    content.push_str("## Resources\n\n");
    content.push_str("- Official documentation: _Link_\n");
    content.push_str("- API reference: _Link_\n");
    content.push_str("- Community: _Link_\n\n");

    // Validate content before writing
    utils::validate_content(&content, 100)
        .context("Generated SOPS content is too short")?;

    std::fs::write(path, &content)
        .with_context(|| format!("Failed to write SOPS to: {}", path.display()))?;

    Ok(())
}

