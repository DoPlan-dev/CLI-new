use anyhow::{Context, Result};
use std::path::PathBuf;
use crate::state::ProjectState;
use crate::utils;

pub fn generate(state: &ProjectState) -> Result<PathBuf> {
    let project_root = utils::project_root()?;
    let context_path = project_root.join("CONTEXT.md");

    let project_name = state.project_name.as_ref()
        .map(|s| s.as_str())
        .unwrap_or("Untitled Project");

    let mut content = String::new();
    content.push_str("# Project Context\n\n");
    content.push_str(&format!("**Project:** {}\n\n", project_name));
    content.push_str(&format!("**Date:** {}\n\n", chrono::Utc::now().format("%Y-%m-%d")));
    content.push_str("---\n\n");

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

    content.push_str("## Project Structure\n\n");
    content.push_str("```\n");
    content.push_str("project-root/\n");
    content.push_str("├── src/                    # Source code\n");
    content.push_str("├── tests/                  # Test files\n");
    content.push_str("├── docs/                   # Documentation\n");
    content.push_str("├── doplan/                 # DoPlan project files\n");
    content.push_str("└── README.md               # Project README\n");
    content.push_str("```\n\n");

    content.push_str("## Documentation\n\n");
    content.push_str("### DoPlan Documents\n");
    content.push_str("- [PRD](./doplan/PRD.md) - Product Requirements Document\n");
    content.push_str("- [Structure](./doplan/structure.md) - Project structure and architecture\n");
    content.push_str("- [DPR](./doplan/design/DPR.md) - Design Preferences & Requirements\n");
    content.push_str("- [RAKD](./doplan/RAKD.md) - Required API Keys Document\n");
    content.push_str("- [SOPS](./doplan/SOPS/) - Service Operating Procedures\n\n");

    content.push_str("### Contracts\n");
    content.push_str("- [API Specification](./doplan/contracts/api-spec.json) - OpenAPI specification\n");
    content.push_str("- [Data Model](./doplan/contracts/data-model.md) - Data models and schemas\n\n");

    content.push_str("## Development Workflow\n\n");
    content.push_str("1. **Planning**: Review plans in `doplan/plan/`\n");
    content.push_str("2. **Design**: Follow DPR guidelines\n");
    content.push_str("3. **Implementation**: Write code following structure guidelines\n");
    content.push_str("4. **Testing**: Write tests alongside implementation\n");
    content.push_str("5. **Review**: Code review before merging\n");
    content.push_str("6. **Deployment**: Follow deployment procedures\n\n");

    content.push_str("## Key Resources\n\n");
    content.push_str("### Design\n");
    content.push_str("- [Design Tokens](./doplan/design/design-tokens.json)\n");
    content.push_str("- [Design Rules](./.doplan/ai/rules/design_rules.mdc)\n\n");

    content.push_str("### Templates\n");
    content.push_str("- [Plan Template](./doplan/templates/plan-template.md)\n");
    content.push_str("- [Design Template](./doplan/templates/design-template.md)\n");
    content.push_str("- [Tasks Template](./doplan/templates/tasks-template.md)\n\n");

    std::fs::write(&context_path, content)
        .context("Failed to write CONTEXT")?;

    Ok(context_path)
}

