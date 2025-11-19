use anyhow::{Context, Result};
use std::path::PathBuf;
use crate::utils;

pub fn generate_all() -> Result<Vec<PathBuf>> {
    let mut generated = Vec::new();

    generated.push(generate_plan_template()?);
    generated.push(generate_design_template()?);
    generated.push(generate_tasks_template()?);

    Ok(generated)
}

pub fn generate_plan_template() -> Result<PathBuf> {
    let doplan_dir = utils::doplan_dir()?;
    let templates_dir = doplan_dir.join("templates");
    utils::ensure_dir(&templates_dir)?;

    let template_path = templates_dir.join("plan-template.md");

    let content = r#"# Feature Plan Template

## Feature: [Feature Name]

### Overview
Brief description of the feature and its purpose.

### Goals
- Primary goal 1
- Primary goal 2
- Primary goal 3

### User Stories
- As a [user type], I want [action] so that [benefit]
- As a [user type], I want [action] so that [benefit]

### Requirements
#### Functional Requirements
- Requirement 1
- Requirement 2
- Requirement 3

#### Non-Functional Requirements
- Performance: [requirements]
- Security: [requirements]
- Accessibility: [requirements]

### Design Considerations
#### Pages
- Page 1: [Description]
- Page 2: [Description]

#### Sections
- Section 1: [Description]
- Section 2: [Description]

#### Components
- Component 1: [Description]
- Component 2: [Description]

#### Cards/UI Elements
- Card 1: [Description]
- Card 2: [Description]

### Technical Approach
- Technology choices
- Architecture decisions
- Integration points

### Dependencies
- Dependency 1
- Dependency 2

### Acceptance Criteria
- [ ] Criterion 1
- [ ] Criterion 2
- [ ] Criterion 3

### Timeline
- Start: [Date]
- End: [Date]
- Milestones:
  - Milestone 1: [Date]
  - Milestone 2: [Date]

### Notes
Additional notes and considerations.
"#;

    std::fs::write(&template_path, content)
        .context("Failed to write plan template")?;

    Ok(template_path)
}

pub fn generate_design_template() -> Result<PathBuf> {
    let doplan_dir = utils::doplan_dir()?;
    let templates_dir = doplan_dir.join("templates");
    utils::ensure_dir(&templates_dir)?;

    let template_path = templates_dir.join("design-template.md");

    let content = r##"# Design Specification Template

## Feature: [Feature Name]

### Design Overview
High-level design description and approach.

### Visual Design
#### Layout
- Layout structure
- Grid system
- Spacing guidelines

#### Colors
- Primary color: [Color]
- Secondary color: [Color]
- Accent color: [Color]

#### Typography
- Headings: [Font, Size, Weight]
- Body text: [Font, Size, Weight]
- Links: [Font, Size, Color]

### Components
#### Component 1
- Purpose: [Description]
- Props: [List]
- States: [List]
- Interactions: [List]

#### Component 2
- Purpose: [Description]
- Props: [List]
- States: [List]
- Interactions: [List]

### User Interactions
- Interaction 1: [Description]
- Interaction 2: [Description]

### Responsive Design
- Mobile: [Breakpoints and considerations]
- Tablet: [Breakpoints and considerations]
- Desktop: [Breakpoints and considerations]

### Accessibility
- WCAG compliance level: [Level]
- Keyboard navigation: [Requirements]
- Screen reader support: [Requirements]

### Design Assets
- Mockups: [Links]
- Icons: [Links]
- Images: [Links]

### Design Tokens
```json
{
  "colors": {
    "primary": "#000000",
    "secondary": "#ffffff"
  },
  "spacing": {
    "small": "8px",
    "medium": "16px",
    "large": "24px"
  }
}
```

### Notes
Additional design considerations and notes.
"##;

    std::fs::write(&template_path, content)
        .context("Failed to write design template")?;

    Ok(template_path)
}

pub fn generate_tasks_template() -> Result<PathBuf> {
    let doplan_dir = utils::doplan_dir()?;
    let templates_dir = doplan_dir.join("templates");
    utils::ensure_dir(&templates_dir)?;

    let template_path = templates_dir.join("tasks-template.md");

    let content = r#"# Tasks Template

## Feature: [Feature Name]

### Tasks

#### Task 1: [Task Name]
- **Status**: [ ] Not Started | [ ] In Progress | [ ] Completed | [ ] Blocked
- **Priority**: High | Medium | Low
- **Assignee**: [Name]
- **Description**: [Task description]
- **Acceptance Criteria**:
  - [ ] Criterion 1
  - [ ] Criterion 2
- **Estimated Time**: [Hours]
- **Actual Time**: [Hours]
- **Notes**: [Additional notes]

#### Task 2: [Task Name]
- **Status**: [ ] Not Started | [ ] In Progress | [ ] Completed | [ ] Blocked
- **Priority**: High | Medium | Low
- **Assignee**: [Name]
- **Description**: [Task description]
- **Acceptance Criteria**:
  - [ ] Criterion 1
  - [ ] Criterion 2
- **Estimated Time**: [Hours]
- **Actual Time**: [Hours]
- **Notes**: [Additional notes]

### Progress Tracking

**Overall Progress**: [X]%

- Completed: [Number]
- In Progress: [Number]
- Not Started: [Number]
- Blocked: [Number]

### Dependencies
- Task 1 depends on: [Task/Feature]
- Task 2 depends on: [Task/Feature]

### Blockers
- Blocker 1: [Description]
- Blocker 2: [Description]

### Notes
Additional notes about tasks and progress.
"#;

    std::fs::write(&template_path, content)
        .context("Failed to write tasks template")?;

    Ok(template_path)
}

