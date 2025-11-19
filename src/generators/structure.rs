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
    utils::ensure_dir(&doplan_dir)
        .context("Failed to create doplan directory")?;

    let structure_path = doplan_dir.join("structure.md");
    utils::validate_write_path(&structure_path)
        .context("Invalid path for structure.md")?;

    let project_name = state.project_name.as_ref()
        .map(|s| s.as_str())
        .unwrap_or("Untitled Project");

    let mut content = String::new();

    // Header
    content.push_str("# Project Structure & Architecture\n\n");
    content.push_str(&format!("**Project:** {}\n\n", project_name));
    content.push_str(&format!("**Date:** {}\n\n", chrono::Utc::now().format("%Y-%m-%d")));
    content.push_str("---\n\n");

    // Architecture Overview
    content.push_str("## Architecture Overview\n\n");
    content.push_str("This document outlines the project structure, architecture decisions, and organization.\n\n");
    content.push_str("---\n\n");

    // Technology Stack
    content.push_str("## Technology Stack\n\n");
    if let Some(tech_stack) = &state.tech_stack {
        if !tech_stack.is_empty() {
            for tech in tech_stack {
                content.push_str(&format!("- {}\n", tech));
            }
            content.push_str("\n");
        }
    } else {
        content.push_str("_Technology stack to be defined_\n\n");
    }

    // Project Structure
    content.push_str("## Project Structure\n\n");
    content.push_str("```\n");
    content.push_str("project-root/\n");
    content.push_str("├── src/                    # Source code\n");
    content.push_str("├── tests/                  # Test files\n");
    content.push_str("├── docs/                   # Documentation\n");
    content.push_str("├── doplan/                 # DoPlan project files\n");
    content.push_str("│   ├── PRD.md\n");
    content.push_str("│   ├── structure.md\n");
    content.push_str("│   ├── contracts/\n");
    content.push_str("│   ├── templates/\n");
    content.push_str("│   └── plan/\n");
    content.push_str("├── .doplan/                # DoPlan configuration\n");
    content.push_str("│   ├── state.json\n");
    content.push_str("│   └── ai/\n");
    content.push_str("└── README.md\n");
    content.push_str("```\n\n");

    // Architecture Layers
    content.push_str("## Architecture Layers\n\n");
    content.push_str("### Presentation Layer\n");
    content.push_str("- User interface components\n");
    content.push_str("- User interaction handling\n");
    content.push_str("- Responsive design implementation\n\n");

    content.push_str("### Business Logic Layer\n");
    content.push_str("- Core business rules\n");
    content.push_str("- Feature implementations\n");
    content.push_str("- Data processing\n\n");

    content.push_str("### Data Layer\n");
    content.push_str("- Database connections\n");
    content.push_str("- Data models\n");
    content.push_str("- Data access patterns\n\n");

    content.push_str("### Integration Layer\n");
    content.push_str("- External API integrations\n");
    content.push_str("- Third-party services\n");
    content.push_str("- Authentication services\n\n");

    // Design Patterns
    content.push_str("## Design Patterns\n\n");
    content.push_str("### Recommended Patterns\n");
    content.push_str("- **MVC/MVP/MVVM**: For UI architecture\n");
    content.push_str("- **Repository Pattern**: For data access\n");
    content.push_str("- **Service Layer**: For business logic\n");
    content.push_str("- **Factory Pattern**: For object creation\n");
    content.push_str("- **Observer Pattern**: For event handling\n\n");

    // File Organization
    content.push_str("## File Organization\n\n");
    content.push_str("### Source Code Structure\n");
    content.push_str("```\n");
    content.push_str("src/\n");
    content.push_str("├── components/      # Reusable UI components\n");
    content.push_str("├── pages/          # Page-level components\n");
    content.push_str("├── services/       # Business logic services\n");
    content.push_str("├── models/         # Data models\n");
    content.push_str("├── utils/          # Utility functions\n");
    content.push_str("├── hooks/          # Custom hooks (if applicable)\n");
    content.push_str("└── config/         # Configuration files\n");
    content.push_str("```\n\n");

    // Naming Conventions
    content.push_str("## Naming Conventions\n\n");
    content.push_str("### Files\n");
    content.push_str("- Use kebab-case for file names: `user-profile.tsx`\n");
    content.push_str("- Use PascalCase for component files: `UserProfile.tsx`\n");
    content.push_str("- Use camelCase for utility files: `formatDate.ts`\n\n");

    content.push_str("### Variables and Functions\n");
    content.push_str("- Use camelCase for variables and functions\n");
    content.push_str("- Use PascalCase for classes and components\n");
    content.push_str("- Use UPPER_SNAKE_CASE for constants\n\n");

    // Development Workflow
    content.push_str("## Development Workflow\n\n");
    content.push_str("1. **Planning**: Create feature plans in `doplan/plan/`\n");
    content.push_str("2. **Design**: Follow design specifications from DPR\n");
    content.push_str("3. **Implementation**: Write code following structure guidelines\n");
    content.push_str("4. **Testing**: Write tests alongside implementation\n");
    content.push_str("5. **Review**: Code review before merging\n");
    content.push_str("6. **Deployment**: Follow deployment procedures\n\n");

    // Validate content before writing
    utils::validate_content(&content, 200)
        .context("Generated structure content is too short")?;

    std::fs::write(&structure_path, &content)
        .with_context(|| format!("Failed to write structure document to: {}", structure_path.display()))?;

    // Verify file was written successfully
    utils::verify_file_write(&structure_path, 200)
        .context("Structure file verification failed")?;

    Ok(structure_path)
}

