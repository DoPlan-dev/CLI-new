# DoPlan CLI - Project Recreation Prompt (Rust Version)

Use this prompt with AI coding assistants (Claude, ChatGPT, Cursor, etc.) to recreate the DoPlan CLI project from scratch using Rust.

---

## Project Overview

Create a comprehensive CLI tool called **DoPlan** that automates project workflow from idea to deployment. The tool should:

- Transform app ideas into well-structured, documented, and trackable development projects
- Integrate seamlessly with AI-powered IDEs (Cursor, Claude CLI, Gemini CLI, Codex CLI, OpenCode, Qwen Code)
- Provide automated project management, documentation generation, Git workflow automation, and visual dashboards
- Support multiple installation methods (npm, Homebrew, binary releases)
- Use Rust as the primary language with a Node.js wrapper for npm distribution

## Core Workflow

**IDE Commands Workflow:**
```
/discuss  → Idea discussion, tech stack recommendations
    ↓
/generate → PHASE 1: Generate PRD, contracts, structure.md, templates
    ↓
/plan     → Create phase/feature structure with plan.md files
    ↓
/generate → PHASE 2: Generate DPR (from plan.md files) + SOPS (from detected services) + RAKD + CONTEXT + README
    ↓
/design   → Design specifications (using DPR)
    ↓
/implement → Start feature implementation
    ↓
/test     → Run tests
    ↓
/review   → Code review
    ↓
/deploy   → Deploy to platforms
```

**Complete Workflow Sequence:**
```
/Discuss → /Refine → /Generate → /Plan → /Generate → /Design → /Implement → /Test → /Review → /Deploy
```

**CLI Commands (Minimal - Only 4):**
- `doplan install` - Install DoPlan in your project
- `doplan dashboard` - Show project dashboard
- `doplan server` - Run development server
- `doplan github` - Sync GitHub data

**Flags:**
- `-v` or `--version` - Show version
- `--tui` - Launch TUI
- `-h` or `--help` - Show help

**TUI (Minimal, Non-Fullscreen):**

**For New Folders (DoPlan not installed):**
```
──────────────────────
1- Install workflow

Transform your idea into a structured project. 
DoPlan will guide you through planning, design, 
and development to make your project real.
──────────────────────
```

**For Installed Projects:**
```
──────────────────────
1- Run Server
2- Settings
   You can add other IDE or CLI like Gemini 
   if you decided to change or use more than one
3- Dashboard
──────────────────────
```

## Technology Stack

### Core Technologies
- **Rust 1.70+** - Primary implementation language
- **Node.js 14+** - For npm package wrapper
- **clap** - CLI framework (`clap` crate with derive feature)
- **ratatui** - TUI framework (formerly `tui-rs`)
- **crossterm** - Cross-platform terminal manipulation
- **git2** - Git operations (`git2-rs` crate)
- **serde** - Serialization/deserialization for config
- **serde_yaml** - YAML configuration support
- **tokio** - Async runtime (for async operations)

### Key Dependencies (Cargo.toml)
```toml
[dependencies]
clap = { version = "4.4", features = ["derive"] }
ratatui = "0.26"
crossterm = "0.28"
git2 = "0.18"
serde = { version = "1.0", features = ["derive"] }
serde_yaml = "0.9"
tokio = { version = "1.35", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }
anyhow = "1.0"
thiserror = "1.0"
colored = "2.1"
indicatif = "0.17"
walkdir = "2.4"
regex = "1.10"
```

### Development Dependencies
```toml
[dev-dependencies]
assert_cmd = "2.0"
predicates = "3.0"
tempfile = "3.10"
mockito = "1.2"
```

## Project Structure

```
doplan-cli/
├── bin/
│   ├── doplan.js          # Node.js wrapper script
│   └── doplan              # Binary executable (built)
├── src/
│   ├── main.rs             # Entry point
│   ├── lib.rs              # Library root (if needed)
│   ├── commands/           # CLI command implementations
│   │   ├── mod.rs
│   │   ├── install.rs
│   │   ├── dashboard.rs
│   │   ├── server.rs
│   │   ├── github.rs
│   │   ├── plan.rs
│   │   ├── generate.rs
│   │   ├── design.rs
│   │   ├── deploy.rs
│   │   ├── publish.rs
│   │   ├── keys.rs
│   │   ├── security.rs
│   │   ├── fix.rs
│   │   └── executor.rs     # TUI command executor
│   ├── agents/             # AI agent definitions (6 agents)
│   ├── checkpoint/         # Checkpoint/snapshot system
│   │   ├── mod.rs
│   │   └── checkpoint.rs
│   ├── config/             # Configuration management
│   │   ├── mod.rs
│   │   └── manager.rs
│   ├── context/            # Project context detection
│   │   ├── mod.rs
│   │   ├── detector.rs
│   │   └── analyzer.rs
│   ├── dashboard/          # Dashboard generation
│   │   ├── mod.rs
│   │   └── generator.rs
│   ├── deployment/         # Deployment platform support
│   │   ├── mod.rs
│   │   └── platforms.rs
│   ├── dpr/                # Design Preferences & Requirements
│   │   ├── mod.rs
│   │   └── generator.rs
│   ├── error/              # Error handling
│   │   ├── mod.rs
│   │   └── error.rs
│   ├── fixer/              # Auto-fix functionality
│   │   ├── mod.rs
│   │   └── fixer.rs
│   ├── generators/         # Document generators
│   │   ├── mod.rs
│   │   ├── prd.rs
│   │   ├── dpr.rs
│   │   ├── sops.rs
│   │   ├── rakd.rs
│   │   └── context.rs
│   ├── git/                # Git utilities
│   │   ├── mod.rs
│   │   └── operations.rs
│   ├── github/             # GitHub integration
│   │   ├── mod.rs
│   │   └── client.rs
│   ├── integration/        # IDE integration
│   │   ├── mod.rs
│   │   └── ide.rs
│   ├── migration/          # Migration system
│   │   ├── mod.rs
│   │   └── migrator.rs
│   ├── publisher/          # Package publishing
│   │   ├── mod.rs
│   │   └── publisher.rs
│   ├── rakd/               # Required API Keys Document
│   │   ├── mod.rs
│   │   └── generator.rs
│   ├── security/           # Security scanning
│   │   ├── mod.rs
│   │   └── scanner.rs
│   ├── sops/               # Service Operating Procedures
│   │   ├── mod.rs
│   │   └── generator.rs
│   ├── statistics/         # Statistics and analytics
│   │   ├── mod.rs
│   │   └── stats.rs
│   ├── template/           # Template system
│   │   ├── mod.rs
│   │   └── engine.rs
│   ├── tui/                # TUI implementation
│   │   ├── mod.rs
│   │   ├── app.rs
│   │   └── screens/
│   │       ├── mod.rs
│   │       ├── menu.rs
│   │       └── dashboard.rs
│   ├── ui/                 # UI components
│   │   ├── mod.rs
│   │   └── components.rs
│   ├── utils/              # Utilities
│   │   ├── mod.rs
│   │   └── helpers.rs
│   ├── validator/          # Validation
│   │   ├── mod.rs
│   │   └── validator.rs
│   ├── wizard/             # Interactive wizards
│   │   ├── mod.rs
│   │   └── wizard.rs
│   └── workflow/           # Workflow management
│       ├── mod.rs
│       └── workflow.rs
├── tests/                   # Integration tests
│   ├── integration_test.rs
│   └── helpers/
│       └── mod.rs
├── scripts/
│   ├── build.js            # Build script
│   ├── postinstall.js      # Post-install hook
│   ├── prepublish.js       # Pre-publish validation
│   ├── release.sh          # Release script
│   ├── check-production-ready.sh  # Production readiness check
│   └── [other test/deployment scripts]
├── templates/              # Code templates
├── docs/                   # Documentation
│   ├── guides/             # User guides
│   ├── implementation/     # Implementation docs
│   ├── references/         # Reference docs
│   └── releases/           # Release notes
├── dist/                   # Build artifacts
├── Cargo.toml              # Rust dependencies
├── Cargo.lock              # Rust lock file
├── package.json            # npm package config
├── Makefile                # Build automation
├── Dockerfile              # Docker build
├── LICENSE                 # MIT License
└── README.md               # Main documentation
```

## Core Features to Implement

### 1. Context-Aware CLI
- Detect project state (empty folder, existing code, DoPlan installed, etc.)
- Show appropriate TUI menu based on context
- Support different views for features, phases, and project root

### 2. Installation System
- `doplan install` - Install DoPlan in a project
- IDE selection (Cursor, Gemini CLI, Claude CLI, Codex CLI, OpenCode, Qwen Code)
- Create IDE-specific directories (`.cursor/`, `.gemini/`, etc.)
- Generate IDE command files
- Set up project structure with `.doplan/` directory

### 3. Interactive TUI (Terminal User Interface)
- **Minimal, non-fullscreen design**
- Context-aware menu (different for new vs installed projects)
- **For new folders (DoPlan not installed):**
  - Show: "1- Install workflow"
  - Include description: "Transform your idea into a structured project. DoPlan will guide you through planning, design, and development to make your project real."
- **For installed projects:**
  - Show: "1- Run Server"
  - Show: "2- Settings" (description: "You can add other IDE or CLI like Gemini if you decided to change or use more than one")
  - Show: "3- Dashboard"
- Simple, clean interface (not fullscreen)
- Progress bars for phases and features
- Real-time updates

### 4. Workflow Commands (via IDE integration)

**Command Details:**

- **`/discuss`** - Refine app idea, get tech stack recommendations

- **`/generate`** - **Context-aware command with two phases:**

  **Phase 1 Detection:**
  - Check if `doplan/PRD.md` exists
  - ❌ **No** → Generate foundational documents (Phase 1)
  - ✅ **Yes** → Continue to Phase 2 check

  **Phase 1 Generation (if PRD doesn't exist):**
  1. Generate `doplan/PRD.md`
  2. Generate `doplan/structure.md`
  3. Generate `doplan/contracts/api-spec.json`
  4. Generate `doplan/contracts/data-model.md`
  5. Generate templates (if not exist):
     - `doplan/templates/plan-template.md`
     - `doplan/templates/design-template.md`
     - `doplan/templates/tasks-template.md`

  **Phase 2 Detection:**
  - Check if `doplan/PRD.md` exists
  - Check if `doplan/plan/01-phases/` exists
  - ✅ **Both exist** → Generate detailed documents (Phase 2)

  **Phase 2 Generation (if PRD exists AND plan exists):**
  1. **Generate DPR:**
     - Read all `doplan/plan/01-phases/*/plan.md` files
     - Extract design information (pages, sections, components, cards)
     - Generate `doplan/design/DPR.md`
     - Generate `doplan/design/design-tokens.json`
     - Generate `.doplan/ai/rules/design_rules.mdc`
  2. **Generate SOPS:**
     - Detect services from `package.json` dependencies
     - Detect services from `Cargo.toml` dependencies
     - Detect services from PRD.md mentions
     - Generate `doplan/SOPS/{category}/{service}.md` for each service
  3. **Generate RAKD:**
     - Detect required API keys for detected services
     - Validate keys in `.env` files
     - Generate `doplan/RAKD.md` with status
  4. **Generate CONTEXT.md:**
     - Detect tech stack from project files
     - Link to documentation and SOPS guides
     - Generate `CONTEXT.md` in project root
  5. **Generate README.md:**
     - Use project information from state
     - Include features, tech stack, setup instructions
     - Generate `README.md` in project root

- **`/plan`** - Create phase and feature structure with plan.md files
- **`/design`** - Design specifications using DPR (must run after Phase 2 /generate)
- **`/implement`** - Start feature implementation
- **`/test`** - Run tests
- **`/review`** - Code review
- **`/deploy`** - Deploy to platforms

### 5. Project Structure Generation
Create this structure when installed:
```
project-root/
├── .doplan/
│   ├── ai/
│   │   ├── agents/         # 6 agent definitions
│   │   ├── rules/          # Workflow rules (includes design_rules.mdc from Phase 2)
│   │   └── commands/       # IDE commands
│   └── config.yaml
├── doplan/
│   ├── dashboard.md        # Markdown dashboard
│   ├── dashboard.html      # HTML dashboard
│   ├── PRD.md              # Product Requirements (Phase 1)
│   ├── structure.md        # Project structure (Phase 1)
│   ├── RAKD.md             # Required API Keys (Phase 2)
│   ├── design/
│   │   ├── DPR.md          # Design Preferences (Phase 2, from plan.md files)
│   │   └── design-tokens.json
│   ├── SOPS/               # Service Operating Procedures (Phase 2)
│   │   └── {category}/
│   │       └── {service}.md
│   ├── contracts/
│   │   ├── api-spec.json   # Phase 1
│   │   └── data-model.md   # Phase 1
│   ├── templates/          # Phase 1
│   │   ├── plan-template.md
│   │   ├── design-template.md
│   │   └── tasks-template.md
│   └── plan/               # Created by /plan command
│       └── 01-phases/
│           └── [features]/
│               └── plan.md  # Used to generate DPR in Phase 2
├── CONTEXT.md               # Tech stack info (Phase 2, in project root)
├── README.md                # Project README (Phase 2, in project root)
└── [IDE-specific dirs]/
```

### 6. AI Agents System
Implement 6 specialized agents:
- **@planner** - Project planning, PRD generation
- **@designer** - Design specifications following DPR
- **@coder** - Implementation
- **@tester** - Test creation and execution
- **@reviewer** - Code review
- **@devops** - Deployment and infrastructure

Each agent should have:
- Agent definition file (`.agent.md`)
- Role and responsibilities
- Communication protocols
- Workflow sequence enforcement

### 7. Design System (DPR)
- **Generated automatically in Phase 2 of `/generate` command (after `/plan`)**
- **Why after plan:** After `/plan`, we know all pages, sections, components, and cards, so we can use this data to create a more detailed DPR
- Reads all `doplan/plan/01-phases/*/plan.md` files to extract:
  - Pages
  - Sections
  - Components
  - Cards
  - UI elements
- Generate `doplan/design/DPR.md` with complete design specifications based on extracted plan data
- Generate `doplan/design/design-tokens.json` (colors, typography, spacing)
- Generate `.doplan/ai/rules/design_rules.mdc` for AI agents
- **Note:** DPR is generated AFTER planning because plan.md files contain all the design information (pages, sections, components, cards)

### 8. API Keys Management & SOPS
- **SOPS (Service Operating Procedures) generated automatically in Phase 2 of `/generate` command (after `/plan`)**
- **Why in Phase 2:** After `/plan`, we have a complete understanding of the project structure and can detect all required services
- Detect required services from:
  - `package.json` dependencies
  - `Cargo.toml` dependencies
  - `PRD.md` mentions
- Generate `doplan/SOPS/{category}/{service}.md` for each detected service
- Generate `RAKD.md` (Required API Keys Document) in Phase 2
- Validate API keys in `.env` files
- TUI for managing API keys (optional, can be accessed via dashboard)

### 9. Deployment Support
- Multi-platform deployment (Vercel, Netlify, Railway, Render, Coolify, Docker)
- Auto-detection of suitable platforms
- Interactive wizards for configuration

### 10. Package Publishing
- Publish to npm, Homebrew, Scoop, Winget
- Interactive wizards for configuration

### 11. Security & Auto-Fix
- Security scanning (npm audit, cargo audit, git-secrets, trufflehog)
- Auto-fix for common issues (npm audit fix, cargo update, cargo fmt, ESLint)
- Severity reporting

### 12. GitHub Integration
- Automatic branch creation for features
- Auto-PR creation when features complete
- Commit tracking
- PR status monitoring
- Branch status tracking

### 13. Dashboard System
- Markdown dashboard (`doplan/dashboard.md`)
- HTML dashboard (`doplan/dashboard.html`)
- Real-time progress tracking
- Phase and feature progress bars
- GitHub activity display
- Next action recommendations

### 14. Configuration Management
- YAML-based configuration (`.doplan/config.yaml`)
- Configuration commands (`config show`, `config set`, `config reset`)
- Settings for GitHub, checkpoints, workflow

### 15. Migration System
- Detect old DoPlan structure
- Migrate to new structure
- Backup before migration
- Validation after migration

## Implementation Details

### Main Entry Point (`src/main.rs`)
```rust
// Context-aware root command
// Detects project state and shows appropriate UI
// Supports ONLY 4 main commands: install, dashboard, server, github
// Flags: -v/--version, --tui, -h/--help
```

### Command Structure
- Use `clap` with derive macros for CLI framework
- **Minimal CLI - ONLY these 4 commands:**
  - `doplan install` - Install DoPlan in your project
  - `doplan dashboard` - Show project dashboard
  - `doplan server` - Run development server
  - `doplan github` - Sync GitHub data
- **Supported Flags:**
  - `-v` or `--version` - Show version information
  - `--tui` - Launch TUI interface
  - `-h` or `--help` - Show help information
- All other functionality via TUI or IDE commands
- Running `doplan` without arguments shows minimal TUI menu (context-aware)

### TUI Implementation
- Use `ratatui` (formerly `tui-rs`) for TUI framework
- Use `crossterm` for cross-platform terminal manipulation
- **Minimal, non-fullscreen design** (not fullscreen)
- Context-aware menu system with two states:

  **For New Folders (DoPlan not installed):**
  ```
  ──────────────────────
  1- Install workflow
  
  Transform your idea into a structured project. 
  DoPlan will guide you through planning, design, 
  and development to make your project real.
  ──────────────────────
  ```

  **For Installed Projects:**
  ```
  ──────────────────────
  1- Run Server
  2- Settings
     You can add other IDE or CLI like Gemini 
     if you decided to change or use more than one
  3- Dashboard
  ──────────────────────
  ```

- Simple, clean interface (not fullscreen)
- Real-time updates
- Progress visualization (in dashboard view)

### IDE Integration
- Generate IDE-specific command files
- Support multiple IDE formats
- Symlink agent definitions and rules
- Store configuration in IDE-specific directories

### Document Generation (Context-Aware `/generate` Command)

The `/generate` command is context-aware and operates in two phases:

**First Use: Generate Foundational Documents (Phase 1)**
- Triggered when `doplan/PRD.md` doesn't exist
- Generates:
  - `doplan/PRD.md` - Product Requirements Document
  - `doplan/structure.md` - Project structure
  - `doplan/contracts/api-spec.json` - API specifications (OpenAPI format)
  - `doplan/contracts/data-model.md` - Data model documentation
  - Templates (plan, design, tasks)

**Second Use: Generate Detailed Documents (Phase 2)**
- Triggered when `doplan/PRD.md` exists AND `doplan/plan/01-phases/` exists (after `/plan` command)
- Generates:
  - `doplan/design/DPR.md` - Design Preferences & Requirements (extracted from plan.md files)
  - `doplan/design/design-tokens.json` - Design tokens
  - `.doplan/ai/rules/design_rules.mdc` - Design rules for AI agents
  - `doplan/SOPS/{category}/{service}.md` - Service Operating Procedures (one per detected service)
  - `doplan/RAKD.md` - Required API Keys Document
  - `CONTEXT.md` - Tech stack and documentation links
  - `README.md` - Project README with features and setup

### Progress Tracking
- Parse `tasks.md` files for completion status
- Calculate phase and feature progress
- Update dashboard automatically
- Track GitHub activity

### Build System
- Cargo build with version/commit/date in `Cargo.toml`
- Makefile for common tasks
- npm package wrapper for cross-platform distribution
- `cargo-dist` or custom scripts for multi-platform releases

### Testing
- Unit tests using Rust's built-in test framework
- Integration tests in `tests/` directory
- Test scripts for CLI operations
- Production readiness check script

## Build Configuration

### Cargo.toml Configuration
```toml
[package]
name = "doplan"
version = "0.0.20-beta"
edition = "2021"
authors = ["DoPlan Team <support@doplan.dev>"]
license = "MIT"
description = "Project workflow automation tool that transforms app ideas into well-structured, documented, and trackable development projects"
repository = "https://github.com/DoPlan-dev/CLI"

[[bin]]
name = "doplan"
path = "src/main.rs"

[dependencies]
# See Key Dependencies section above

[dev-dependencies]
# See Development Dependencies section above

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
```

### Makefile Targets
```makefile
.PHONY: build install test clean release help

build: ## Build the binary
	cargo build --release

install: build ## Install the binary
	cargo install --path .

test: ## Run tests
	cargo test

test-coverage: ## Run tests with coverage
	cargo test --all-features
	cargo tarpaulin --out Html

fmt: ## Format code
	cargo fmt

clippy: ## Run clippy linter
	cargo clippy -- -D warnings

lint: fmt clippy ## Run linters

clean: ## Clean build artifacts
	cargo clean
	rm -rf target/

check-production: ## Check if ready for production release
	./scripts/check-production-ready.sh

release: ## Build release binaries
	cargo build --release
	# Use cargo-dist or custom release script
```

### npm Package Configuration
```json
{
  "name": "@doplan-dev/cli",
  "version": "0.0.17-beta",
  "description": "Project workflow automation tool that transforms app ideas into well-structured, documented, and trackable development projects",
  "bin": {
    "doplan": "./bin/doplan.js"
  },
  "scripts": {
    "postinstall": "node scripts/postinstall.js",
    "build": "node scripts/build.js",
    "prepublishOnly": "npm run build && node scripts/prepublish.js"
  },
  "files": [
    "bin/",
    "scripts/",
    "README.md",
    "LICENSE"
  ],
  "keywords": [
    "cli",
    "project-management",
    "workflow",
    "automation",
    "development",
    "ide-integration",
    "github",
    "productivity"
  ],
  "author": "DoPlan Team <support@doplan.dev>",
  "license": "MIT",
  "repository": {
    "type": "git",
    "url": "https://github.com/DoPlan-dev/CLI.git"
  },
  "engines": {
    "node": ">=14.0.0"
  },
  "os": [
    "darwin",
    "linux",
    "win32"
  ],
  "cpu": [
    "x64",
    "arm64"
  ]
}
```

### Binary Wrapper (`bin/doplan.js`)
- Detect platform (darwin, linux, windows)
- Detect architecture (amd64, arm64)
- Download binary from GitHub releases if not present
- Execute binary with passed arguments

## Key Implementation Files

### Core Commands
- `src/commands/install.rs` - Installation logic
- `src/commands/dashboard.rs` - Dashboard generation
- `src/commands/server.rs` - Development server
- `src/commands/github.rs` - GitHub sync
- `src/commands/executor.rs` - TUI command executor

### Context Detection
- `src/context/detector.rs` - Project state detection
- `src/context/analyzer.rs` - Project analysis

### TUI
- `src/tui/app.rs` - Main TUI application
- `src/tui/screens/menu.rs` - Menu screen
- `src/tui/screens/dashboard.rs` - Dashboard screen
- `src/wizard/wizard.rs` - Interactive wizards

### Generators
- `src/generators/prd.rs` - PRD generator
- `src/generators/dpr.rs` - DPR generator
- `src/generators/sops.rs` - SOPS generator
- `src/generators/rakd.rs` - RAKD generator
- `src/generators/context.rs` - CONTEXT generator

### Integration
- `src/integration/ide.rs` - IDE integration logic

## Rust-Specific Implementation Patterns

### Error Handling
- Use `anyhow` for application errors
- Use `thiserror` for library errors
- Implement custom error types with `#[derive(thiserror::Error)]`

### Async Operations
- Use `tokio` for async runtime
- Use `reqwest` for HTTP requests
- Use async/await patterns throughout

### Configuration
- Use `serde` and `serde_yaml` for YAML config
- Implement `Serialize` and `Deserialize` for config structs

### Git Operations
- Use `git2-rs` crate for Git operations
- Handle Git errors appropriately
- Support both SSH and HTTPS remotes

### File Operations
- Use `std::fs` for file operations
- Use `walkdir` for directory traversal
- Handle file I/O errors gracefully

### Testing
- Unit tests in same file with `#[cfg(test)]`
- Integration tests in `tests/` directory
- Use `assert_cmd` for CLI testing
- Use `tempfile` for temporary test directories

## Documentation Structure

Create comprehensive documentation:
- `README.md` - Main project documentation
- `docs/guides/` - User guides (installation, usage, troubleshooting)
- `docs/implementation/` - Implementation details
- `docs/references/` - API references
- `docs/releases/` - Release notes

## Production Readiness

Implement production readiness check that verifies:
- Dependencies (Rust, Cargo, Git)
- Code formatting (`cargo fmt`)
- Linting (`cargo clippy`)
- Build and test execution
- Test coverage (CLI-appropriate thresholds)
- TODO/FIXME comments review
- Debug code detection
- Git status
- Version information
- Documentation completeness
- License file
- Build configuration
- Test infrastructure
- Dependency verification

## Release Process

- Use `cargo-dist` or custom scripts for multi-platform releases
- Support: darwin (amd64, arm64), linux (amd64, arm64), windows (amd64, arm64)
- Generate tar.gz archives
- Generate .deb and .rpm for Linux
- Create Homebrew formula
- Publish to npm
- GitHub Actions for CI/CD

### Release Script Example
```bash
#!/bin/bash
# Build for all platforms
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu
cargo build --release --target x86_64-pc-windows-gnu
cargo build --release --target aarch64-pc-windows-msvc

# Package and upload
# ... (custom packaging logic)
```

## Testing Strategy

- Unit tests for core functionality (using Rust's built-in test framework)
- Integration tests for CLI operations (in `tests/` directory)
- Test scripts for installation scenarios
- Manual testing checklists
- Performance tests
- Use `cargo test --all-features` for comprehensive testing

## Key Design Principles

1. **Context-Aware**: Adapt to project state automatically
2. **Minimal CLI**: Only 4 essential commands exposed (install, dashboard, server, github)
3. **Minimal TUI**: Simple, non-fullscreen interface that adapts to project state
4. **Context-Aware Generation**: `/generate` command detects project state and generates appropriate documents in two phases
5. **Workflow Sequence**: Enforce proper sequence (discuss → generate → plan → generate → design → implement)
6. **DPR After Plan**: DPR is generated after `/plan` because plan.md files contain all design information (pages, sections, components, cards)
7. **SOPS After Plan**: SOPS is generated in Phase 2 after `/plan` when we have complete project understanding
8. **IDE Integration**: Seamless workflow with AI IDEs through IDE-specific commands
9. **Automation**: Reduce manual project management overhead
10. **Best Practices**: Enforce through automated rules
11. **Extensible**: Template system and customizable workflows
12. **Rust Best Practices**: Follow Rust conventions, use idiomatic Rust code

## Version Information

- Current version: 0.0.20-beta
- Rust version: 1.70+
- Edition: 2021
- License: MIT
- Repository: github.com/DoPlan-dev/CLI

## Additional Requirements

1. **Error Handling**: Comprehensive error handling with `anyhow` and `thiserror`
2. **Logging**: Use `tracing` or `log` crate for structured logging
3. **Validation**: Input validation and project structure validation
4. **Backup**: Automatic backups before destructive operations
5. **Migration**: Support for migrating from old versions
6. **Internationalization**: Prepare for i18n (if needed)
7. **Accessibility**: Ensure TUI is accessible
8. **Performance**: Optimize for fast startup and execution
9. **Memory Safety**: Leverage Rust's memory safety guarantees
10. **Concurrency**: Use async/await for concurrent operations

## Rust-Specific Considerations

### Memory Management
- Use owned types where appropriate
- Use references (`&str`, `&Path`) to avoid unnecessary allocations
- Use `String` and `PathBuf` for owned strings and paths
- Leverage Rust's ownership system for resource management

### Error Handling Patterns
```rust
use anyhow::{Context, Result};

fn example() -> Result<()> {
    let content = std::fs::read_to_string("file.txt")
        .context("Failed to read file")?;
    Ok(())
}
```

### Async Patterns
```rust
use tokio::fs;

async fn example() -> anyhow::Result<()> {
    let content = fs::read_to_string("file.txt").await?;
    Ok(())
}
```

### Configuration Patterns
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    github: GitHubConfig,
    checkpoint: CheckpointConfig,
}
```

## Implementation Order Suggestion

1. Set up project structure and Cargo.toml
2. Implement core CLI framework (clap) with 4 minimal commands
3. Implement context detection
4. Implement minimal TUI (non-fullscreen, context-aware menu) using ratatui
5. Implement installation system
6. Implement context-aware `/generate` command (Phase 1: foundational docs - PRD, structure, contracts, templates)
7. Implement `/plan` command (creates phase/feature structure with plan.md files)
8. Implement context-aware `/generate` command (Phase 2: DPR from plan.md files, SOPS from detected services, RAKD, CONTEXT, README)
9. Implement document generators:
   - Phase 1: PRD, structure, contracts, templates
   - Phase 2: DPR (extracts from plan.md), SOPS (detects from dependencies), RAKD, CONTEXT, README
10. Implement GitHub integration using git2-rs
11. Implement dashboard system
12. Implement IDE integration
13. Implement advanced features (agents, deployment, etc.)
14. Add testing and documentation
15. Set up CI/CD and release process

---

## Usage Instructions for AI Assistant

When using this prompt with an AI coding assistant:

1. **Start with structure**: Ask to create the Cargo project structure first
2. **Build incrementally**: Implement features one at a time
3. **Test frequently**: Test each feature as it's implemented using `cargo test`
4. **Follow Rust patterns**: Use idiomatic Rust code, leverage ownership system
5. **Document as you go**: Add documentation comments and doc tests
6. **Refactor when needed**: Keep code clean and maintainable
7. **Use Rust tooling**: Leverage `cargo fmt`, `cargo clippy`, `cargo test`
8. **Handle errors properly**: Use `anyhow` and `thiserror` for error handling
9. **Write tests**: Include unit tests and integration tests
10. **Optimize builds**: Use release profile for production builds

The assistant should create a fully functional CLI tool that matches the specifications above, with all features implemented, tested, and documented using Rust best practices.

