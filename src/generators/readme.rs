use anyhow::{Context, Result};
use std::path::PathBuf;
use crate::state::ProjectState;
use crate::utils;

pub fn generate(state: &ProjectState) -> Result<PathBuf> {
    // Validate state
    if state.project_name.is_none() {
        anyhow::bail!("Project state is incomplete: missing project_name. Run /discuss first.");
    }

    let project_root = utils::project_root()
        .context("Failed to get project root directory")?;
    let readme_path = project_root.join("README.md");
    utils::validate_write_path(&readme_path)
        .context("Invalid path for README.md")?;

    let project_name = state.project_name.as_ref()
        .map(|s| s.as_str())
        .unwrap_or("Untitled Project");

    let idea = state.idea.as_ref()
        .map(|s| s.as_str())
        .unwrap_or("No description available");

    let mut content = String::new();
    content.push_str(&format!("# {}\n\n", project_name));
    content.push_str(&format!("{}\n\n", idea));
    content.push_str("---\n\n");

    content.push_str("## Features\n\n");
    if let Some(features) = &state.features {
        if !features.is_empty() {
            for feature in features {
                content.push_str(&format!("- **{}** ({}) - {}\n", 
                    feature.name, feature.priority, feature.description));
            }
            content.push_str("\n");
        }
    } else {
        content.push_str("_Features to be defined_\n\n");
    }

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

    content.push_str("## Getting Started\n\n");
    content.push_str("### Prerequisites\n\n");
    content.push_str("_List prerequisites_\n\n");

    content.push_str("### Installation\n\n");
    content.push_str("```bash\n");
    content.push_str("# Installation steps\n");
    content.push_str("```\n\n");

    content.push_str("### Configuration\n\n");
    content.push_str("1. Copy `.env.example` to `.env`\n");
    content.push_str("2. Configure environment variables\n");
    content.push_str("3. See [RAKD](./doplan/RAKD.md) for required API keys\n\n");

    content.push_str("### Running the Project\n\n");
    content.push_str("```bash\n");
    content.push_str("# Development\n");
    content.push_str("# Production\n");
    content.push_str("```\n\n");

    content.push_str("## Project Structure\n\n");
    content.push_str("```\n");
    content.push_str("project-root/\n");
    content.push_str("├── src/                    # Source code\n");
    content.push_str("├── tests/                  # Test files\n");
    content.push_str("├── doplan/                 # DoPlan project files\n");
    content.push_str("│   ├── PRD.md              # Product Requirements\n");
    content.push_str("│   ├── structure.md         # Project structure\n");
    content.push_str("│   ├── design/             # Design documents\n");
    content.push_str("│   ├── plan/               # Phase and feature plans\n");
    content.push_str("│   └── contracts/          # API and data contracts\n");
    content.push_str("└── README.md               # This file\n");
    content.push_str("```\n\n");

    content.push_str("## Documentation\n\n");
    content.push_str("- [PRD](./doplan/PRD.md) - Product Requirements Document\n");
    content.push_str("- [Structure](./doplan/structure.md) - Project architecture\n");
    content.push_str("- [DPR](./doplan/design/DPR.md) - Design Preferences & Requirements\n");
    content.push_str("- [CONTEXT](./CONTEXT.md) - Project context and resources\n");
    content.push_str("- [RAKD](./doplan/RAKD.md) - Required API Keys Document\n\n");

    content.push_str("## Development Phases\n\n");
    if let Some(phases) = &state.phases {
        if !phases.is_empty() {
            for (i, phase) in phases.iter().enumerate() {
                content.push_str(&format!("### Phase {}: {}\n\n", i + 1, phase.name));
                content.push_str(&format!("{}\n\n", phase.description));
                if !phase.features.is_empty() {
                    content.push_str("**Features:**\n");
                    for feature_name in &phase.features {
                        content.push_str(&format!("- {}\n", feature_name));
                    }
                    content.push_str("\n");
                }
            }
        }
    }

    content.push_str("## Contributing\n\n");
    content.push_str("_Contributing guidelines_\n\n");

    content.push_str("## License\n\n");
    content.push_str("_License information_\n\n");

    // Validate content before writing
    utils::validate_content(&content, 100)
        .context("Generated README content is too short")?;

    std::fs::write(&readme_path, &content)
        .with_context(|| format!("Failed to write README to: {}", readme_path.display()))?;

    // Verify file was written successfully
    utils::verify_file_write(&readme_path, 100)
        .context("README file verification failed")?;

    Ok(readme_path)
}

