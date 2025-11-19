# Documentation Organization and Naming Rules

## Documentation Structure

### Root Documentation Files

All project documentation follows a consistent structure:

```
project-root/
├── README.md                    # Main project documentation
├── CONTEXT.md                   # Tech stack context (for IDEs/CLIs)
├── CHANGELOG.md                 # Version history
├── CONTRIBUTING.md              # Contribution guidelines
├── LICENSE                      # License file
├── doplan/                      # DoPlan planning documents
│   ├── PRD.md                  # Product Requirements Document
│   ├── structure.md            # Project structure
│   ├── dashboard.md            # Progress dashboard (markdown)
│   ├── dashboard.html          # Progress dashboard (HTML)
│   ├── idea-notes.md            # Idea discussion notes
│   ├── contracts/               # API contracts and specifications
│   │   ├── api-spec.json       # OpenAPI/Swagger specification
│   │   └── data-model.md       # Data models documentation
│   ├── templates/              # Document templates
│   │   ├── plan.md             # Feature plan template
│   │   ├── design.md          # Design specification template
│   │   └── tasks.md            # Task list template
│   └── XX-phase/               # Phase directories (01-phase, 02-phase, etc.)
│       ├── phase-plan.md       # Phase planning document
│       ├── phase-progress.json # Phase progress tracking
│       └── XX-Feature/         # Feature directories (01-Feature, 02-Feature, etc.)
│           ├── plan.md         # Feature plan
│           ├── design.md       # Feature design
│           ├── tasks.md        # Feature tasks
│           └── progress.json   # Feature progress tracking
```

## File Naming Conventions

### General Rules

1. **Use lowercase with hyphens** for file names:
   - `api-spec.json`
   - `data-model.md`
   - NOT: `ApiSpec.json`
   - NOT: `data_model.md`

2. **Use descriptive names**:
   - `user-authentication-plan.md`
   - NOT: `plan1.md`

3. **Use consistent extensions**:
   - Markdown: `*.md`
   - JSON: `*.json`
   - YAML: `*.yml` or `*.yaml`
   - HTML: `*.html`

### Phase and Feature Naming

1. **Phase directories**: `XX-phase` (e.g., `01-phase`, `02-phase`)
   - Always use two-digit numbers with leading zeros
   - Always use lowercase `phase`

2. **Feature directories**: `XX-Feature` (e.g., `01-Feature`, `02-Feature`)
   - Always use two-digit numbers with leading zeros
   - Capitalize `Feature`

3. **Feature-specific files**: Always lowercase with hyphens
   - `plan.md`
   - `design.md`
   - `tasks.md`
   - `progress.json`

### Document File Naming

| Document Type | File Name | Location | Description |
|--------------|-----------|----------|-------------|
| Product Requirements | `PRD.md` | `doplan/` | Product Requirements Document |
| Project Structure | `structure.md` | `doplan/` | Architecture and structure |
| Tech Stack Context | `CONTEXT.md` | Root | Technology stack with docs links |
| Dashboard | `dashboard.md` / `dashboard.html` | `doplan/` | Progress visualization |
| Idea Notes | `idea-notes.md` | `doplan/` | Initial idea discussion |
| API Specification | `api-spec.json` | `doplan/contracts/` | OpenAPI/Swagger spec |
| Data Models | `data-model.md` | `doplan/contracts/` | Data structure documentation |
| Phase Plan | `phase-plan.md` | `doplan/XX-phase/` | Phase planning document |
| Phase Progress | `phase-progress.json` | `doplan/XX-phase/` | Phase progress tracking |
| Feature Plan | `plan.md` | `doplan/XX-phase/XX-Feature/` | Feature planning |
| Feature Design | `design.md` | `doplan/XX-phase/XX-Feature/` | Feature design specs |
| Feature Tasks | `tasks.md` | `doplan/XX-phase/XX-Feature/` | Task breakdown |
| Feature Progress | `progress.json` | `doplan/XX-phase/XX-Feature/` | Feature progress |

## Documentation Content Standards

### Markdown Files

1. **Always include frontmatter** (for IDE context):
   ```yaml
   ---
   title: Document Title
   description: Brief description
   last_updated: YYYY-MM-DD
   ---
   ```

2. **Use consistent heading hierarchy**:
   - `#` - Document title
   - `##` - Main sections
   - `###` - Subsections
   - `####` - Sub-subsections

3. **Include table of contents** for long documents:
   ```markdown
   ## Table of Contents
   - [Section 1](#section-1)
   - [Section 2](#section-2)
   ```

### JSON Files

1. **Use consistent structure**:
   - Always include `version` field
   - Use descriptive property names
   - Include `last_updated` timestamp

2. **Progress JSON structure**:
   ```json
   {
     "version": "1.0.0",
     "status": "in-progress",
     "progress": 45,
     "completed_tasks": 9,
     "total_tasks": 20,
     "last_updated": "2024-01-15T10:30:00Z"
   }
   ```

## CONTEXT.md Structure

The `CONTEXT.md` file provides comprehensive tech stack information for IDEs and CLIs.

### Required Sections

1. **Programming Languages**
   - Language name
   - Version
   - Official documentation link
   - Key features used

2. **Frameworks**
   - Framework name
   - Version
   - Official documentation link
   - Purpose in project

3. **CLIs and Tools**
   - Tool name
   - Version
   - Installation command
   - Documentation link
   - Usage examples

4. **Services**
   - Service name
   - Provider
   - Documentation link
   - Configuration details

5. **Databases**
   - Database type
   - Version
   - Documentation link
   - Connection details

6. **Development Tools**
   - IDE/Editor
   - Extensions/Plugins
   - Configuration files

### CONTEXT.md Format

```markdown
# Project Technology Stack

## Programming Languages

### Go
- **Version:** 1.24.0
- **Documentation:** https://go.dev/doc/
- **Usage:** Backend CLI development
- **Key Features:** Goroutines, Channels, Interfaces

## Frameworks

### Cobra
- **Version:** v1.8.0
- **Documentation:** https://github.com/spf13/cobra
- **Purpose:** CLI command structure

## CLIs and Tools

### DoPlan CLI
- **Version:** 1.0.0
- **Installation:** `go install github.com/DoPlan-dev/CLI@latest`
- **Documentation:** https://github.com/DoPlan-dev/CLI
- **Usage:** `doplan install`

## Services

### GitHub
- **Provider:** GitHub
- **Documentation:** https://docs.github.com/
- **Purpose:** Version control and CI/CD

## Databases

### SQLite (if applicable)
- **Version:** 3.x
- **Documentation:** https://www.sqlite.org/docs.html
- **Usage:** Local development database

## Development Tools

### Cursor IDE
- **Extensions:** DoPlan commands
- **Configuration:** `.cursor/rules/`, `.cursor/commands/`
```

## Documentation Generation Rules

### When to Generate CONTEXT.md

1. **During installation** (`doplan install`):
   - Automatically generate initial `CONTEXT.md`
   - Include detected technologies from project files

2. **During /Discuss command**:
   - Update `CONTEXT.md` with recommended tech stack
   - Add documentation links for selected technologies

3. **During /Generate command**:
   - Update `CONTEXT.md` with finalized tech stack
   - Include all technologies mentioned in PRD

4. **Manual updates**:
   - Update when adding new dependencies
   - Update when changing tech stack
   - Keep documentation links current

### Auto-Detection Rules

When generating `CONTEXT.md`, detect technologies from:

1. **Package managers**:
   - `go.mod` → Go, Go modules
   - `package.json` → Node.js, npm packages
   - `requirements.txt` → Python, pip packages
   - `Cargo.toml` → Rust, Cargo packages
   - `pom.xml` → Java, Maven
   - `Gemfile` → Ruby, Bundler

2. **Configuration files**:
   - `.git/config` → Git version
   - `docker-compose.yml` → Docker, services
   - `.github/workflows/` → GitHub Actions
   - `tsconfig.json` → TypeScript
   - `webpack.config.js` → Webpack

3. **Project structure**:
   - `src/` → Common source structure
   - `api/` → API-related code
   - `frontend/` → Frontend framework
   - `backend/` → Backend framework

## Documentation Maintenance

### Update Frequency

- **CONTEXT.md**: Update when tech stack changes
- **PRD.md**: Update when requirements change
- **structure.md**: Update when architecture changes
- **Progress files**: Update automatically via /Progress command
- **Dashboard**: Update automatically via /Dashboard command

### Version Control

- All documentation files should be committed to Git
- Use conventional commit messages: `docs: update CONTEXT.md`
- Keep documentation in sync with code changes
- Review documentation during PR reviews

## IDE/CLI Integration

### Cursor IDE

- Rules in: `.cursor/rules/`
- Commands in: `.cursor/commands/`
- Context file: `CONTEXT.md` (auto-loaded)

### Gemini CLI

- Commands in: `.gemini/commands/`
- Context file: `CONTEXT.md` (reference in prompts)

### Claude Code

- Commands in: `.claude/commands/`
- Context file: `CONTEXT.md` (reference with @CONTEXT.md)

### Codex CLI

- Prompts in: `.codex/prompts/`
- Context file: `CONTEXT.md` (reference in prompts)

### OpenCode

- Commands in: `.opencode/command/`
- Context file: `CONTEXT.md` (reference with @CONTEXT.md)

### Qwen Code

- Commands in: `.qwen/commands/`
- Context file: `CONTEXT.md` (reference in prompts)

## Best Practices

1. **Keep documentation up-to-date**:
   - Update when code changes
   - Review during PR process
   - Automate updates where possible

2. **Use consistent formatting**:
   - Follow markdown best practices
   - Use consistent heading levels
   - Include code examples

3. **Link to official documentation**:
   - Always include official docs links
   - Keep links current
   - Verify links periodically

4. **Make documentation discoverable**:
   - Use clear file names
   - Include in README.md
   - Reference in IDE commands

5. **Automate documentation**:
   - Generate from code when possible
   - Use templates for consistency
   - Update via CLI commands
