use anyhow::{Context, Result};
use std::path::PathBuf;
use crate::state::ProjectState;
use crate::utils;

pub fn generate(state: &ProjectState, _idea_notes: &Option<String>) -> Result<PathBuf> {
    // Validate state
    if state.project_name.is_none() {
        anyhow::bail!("Project state is incomplete: missing project_name. Run /discuss first.");
    }

    let doplan_dir = utils::doplan_dir()
        .context("Failed to get doplan directory")?;
    let contracts_dir = doplan_dir.join("contracts");
    utils::ensure_dir(&contracts_dir)
        .context("Failed to create contracts directory")?;

    let data_model_path = contracts_dir.join("data-model.md");
    utils::validate_write_path(&data_model_path)
        .context("Invalid path for data-model.md")?;

    let project_name = state.project_name.as_ref()
        .map(|s| s.as_str())
        .unwrap_or("Untitled Project");

    let mut content = String::new();

    // Header
    content.push_str("# Data Model & Schemas\n\n");
    content.push_str(&format!("**Project:** {}\n\n", project_name));
    content.push_str(&format!("**Date:** {}\n\n", chrono::Utc::now().format("%Y-%m-%d")));
    content.push_str("---\n\n");

    // Overview
    content.push_str("## Overview\n\n");
    content.push_str("This document defines the data models, schemas, and database structure for the project.\n\n");
    content.push_str("---\n\n");

    // Core Entities
    content.push_str("## Core Entities\n\n");

    // User entity (common in most apps)
    content.push_str("### User\n\n");
    content.push_str("Represents a user in the system.\n\n");
    content.push_str("```typescript\n");
    content.push_str("interface User {\n");
    content.push_str("  id: string;              // Unique identifier\n");
    content.push_str("  email: string;            // User email\n");
    content.push_str("  name: string;             // User name\n");
    content.push_str("  createdAt: Date;          // Account creation date\n");
    content.push_str("  updatedAt: Date;          // Last update date\n");
    content.push_str("  status: 'active' | 'inactive' | 'suspended';\n");
    content.push_str("}\n");
    content.push_str("```\n\n");

    // Add feature-based entities
    if let Some(features) = &state.features {
        for feature in features {
            let entity_name = feature.name.replace(" ", "");
            content.push_str(&format!("### {}\n\n", entity_name));
            content.push_str(&format!("{}\n\n", feature.description));
            content.push_str("```typescript\n");
            content.push_str(&format!("interface {} {{\n", entity_name));
            content.push_str("  id: string;\n");
            content.push_str("  // Add properties based on feature requirements\n");
            content.push_str("  createdAt: Date;\n");
            content.push_str("  updatedAt: Date;\n");
            content.push_str("}\n");
            content.push_str("```\n\n");
        }
    }

    // Database Schema
    content.push_str("## Database Schema\n\n");
    content.push_str("### Tables\n\n");
    content.push_str("#### users\n");
    content.push_str("| Column | Type | Constraints | Description |\n");
    content.push_str("|--------|------|-------------|-------------|\n");
    content.push_str("| id | UUID | PRIMARY KEY | Unique identifier |\n");
    content.push_str("| email | VARCHAR(255) | UNIQUE, NOT NULL | User email |\n");
    content.push_str("| name | VARCHAR(255) | NOT NULL | User name |\n");
    content.push_str("| created_at | TIMESTAMP | NOT NULL | Creation timestamp |\n");
    content.push_str("| updated_at | TIMESTAMP | NOT NULL | Update timestamp |\n");
    content.push_str("| status | VARCHAR(20) | NOT NULL | User status |\n\n");

    // Relationships
    content.push_str("## Relationships\n\n");
    content.push_str("### Entity Relationships\n\n");
    content.push_str("- **User** has many **Items** (one-to-many)\n");
    content.push_str("- **User** belongs to **Organization** (many-to-one)\n");
    content.push_str("- **Items** can have many **Tags** (many-to-many)\n\n");

    // Data Validation Rules
    content.push_str("## Data Validation Rules\n\n");
    content.push_str("### User Entity\n");
    content.push_str("- Email must be valid format\n");
    content.push_str("- Email must be unique\n");
    content.push_str("- Name must be between 2-100 characters\n");
    content.push_str("- Status must be one of: active, inactive, suspended\n\n");

    // Indexes
    content.push_str("## Database Indexes\n\n");
    content.push_str("### Recommended Indexes\n");
    content.push_str("- `users.email` - For fast email lookups\n");
    content.push_str("- `users.created_at` - For sorting by creation date\n");
    content.push_str("- `users.status` - For filtering by status\n\n");

    // Data Migration Strategy
    content.push_str("## Data Migration Strategy\n\n");
    content.push_str("### Version Control\n");
    content.push_str("- Use migration files for schema changes\n");
    content.push_str("- Version all schema changes\n");
    content.push_str("- Test migrations on staging before production\n\n");

    content.push_str("### Migration Best Practices\n");
    content.push_str("- Always backup before migrations\n");
    content.push_str("- Test rollback procedures\n");
    content.push_str("- Monitor migration performance\n");
    content.push_str("- Document breaking changes\n\n");

    // API Data Contracts
    content.push_str("## API Data Contracts\n\n");
    content.push_str("### Request/Response Formats\n\n");
    content.push_str("All API requests and responses should follow these formats:\n\n");
    content.push_str("#### Standard Response\n");
    content.push_str("```json\n");
    content.push_str("{\n");
    content.push_str("  \"success\": true,\n");
    content.push_str("  \"data\": {},\n");
    content.push_str("  \"message\": \"Operation successful\"\n");
    content.push_str("}\n");
    content.push_str("```\n\n");

    content.push_str("#### Error Response\n");
    content.push_str("```json\n");
    content.push_str("{\n");
    content.push_str("  \"success\": false,\n");
    content.push_str("  \"error\": {\n");
    content.push_str("    \"code\": \"ERROR_CODE\",\n");
    content.push_str("    \"message\": \"Error description\"\n");
    content.push_str("  }\n");
    content.push_str("}\n");
    content.push_str("```\n\n");

    // Validate content before writing
    utils::validate_content(&content, 200)
        .context("Generated data model content is too short")?;

    std::fs::write(&data_model_path, &content)
        .with_context(|| format!("Failed to write data model document to: {}", data_model_path.display()))?;

    // Verify file was written successfully
    utils::verify_file_write(&data_model_path, 200)
        .context("Data model file verification failed")?;

    Ok(data_model_path)
}

