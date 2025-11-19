# DoPlan CLI

A comprehensive CLI tool that automates project workflow from idea to deployment

[![npm version](https://img.shields.io/npm/v/@doplan-dev/cli.svg)](https://www.npmjs.com/package/@doplan-dev/cli)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

---

## Features

- **Project Planning** (high) - Interactive project planning with phases and features
- **Document Generation** (high) - Automated generation of PRD, structure, and contracts
- **Phase Management** (medium) - Create and manage development phases
- **Feature Tracking** (medium) - Track features with plans, designs, and tasks
- **Dashboard** (low) - Visual dashboard for project progress

## Technology Stack

- **CLI Framework**: Rust (clap, tokio)
- **Language**: Rust
- **Distribution**: npm (Node.js wrapper)
- **Build**: Cargo

## Getting Started

### Prerequisites

- **Node.js** >= 14.0.0 (for npm package installation)
- **Rust & Cargo** (optional, only if building from source)

### Installation

#### Install from npm (Recommended)

```bash
# Global installation
npm install -g @doplan-dev/cli

# Or as a project dependency
npm install @doplan-dev/cli
```

After installation, verify it works:

```bash
doplan --version
```

#### Install from Source

```bash
# Clone the repository
git clone https://github.com/DoPlan-dev/CLI-new.git
cd CLI-new

# Build the release binary
cargo build --release

# Install globally
cargo install --path .

# Or run directly
./target/release/doplan --version
```

### Quick Start

1. **Initialize DoPlan in your project:**
   ```bash
   doplan install
   ```

2. **Use IDE integration commands:**
   - `/discuss` - Refine app idea, get tech stack recommendations
   - `/generate` - Generate project documents
   - `/plan` - Create phase and feature structure
   - `/implement` - Start feature implementation
   - `/next` - Get next recommended action
   - `/progress` - Update progress tracking
   - `/phase` - Manage phases
   - `/feature` - Manage features

3. **View project dashboard:**
   ```bash
   doplan dashboard
   ```

### Configuration

DoPlan stores configuration in `.doplan/state.json` and project files in `doplan/` directory.

For API keys and external services, see [RAKD](./doplan/RAKD.md).

## Project Structure

```
project-root/
├── src/                    # Source code
├── tests/                  # Test files
├── doplan/                 # DoPlan project files
│   ├── PRD.md              # Product Requirements
│   ├── structure.md         # Project structure
│   ├── design/             # Design documents
│   ├── plan/               # Phase and feature plans
│   └── contracts/          # API and data contracts
└── README.md               # This file
```

## Documentation

- [PRD](./doplan/PRD.md) - Product Requirements Document
- [Structure](./doplan/structure.md) - Project architecture
- [DPR](./doplan/design/DPR.md) - Design Preferences & Requirements
- [CONTEXT](./CONTEXT.md) - Project context and resources
- [RAKD](./doplan/RAKD.md) - Required API Keys Document

## Development Phases

### Phase 1: Foundation

Set up project foundation and core infrastructure

**Features:**
- Project Planning
- Document Generation

### Phase 2: Core Features

Implement core functionality and features

**Features:**
- Phase Management
- Feature Tracking

### Phase 3: Enhancement

Add enhancements and polish

**Features:**
- Dashboard

## Platform Support

DoPlan CLI supports the following platforms:

- **macOS**: Intel (x86_64) and Apple Silicon (arm64)
- **Linux**: x86_64 and ARM64
- **Windows**: x86_64

Binaries are automatically downloaded for your platform when installing from npm.

## Troubleshooting

### Binary not found

If you see "Binary not found" errors:

1. **For npm installations**: The binary should download automatically. If it doesn't:
   ```bash
   npm uninstall -g @doplan-dev/cli
   npm install -g @doplan-dev/cli
   ```

2. **For source installations**: Make sure you built the release binary:
   ```bash
   cargo build --release
   ```

### Version mismatch

If you encounter version synchronization issues:

```bash
# Check version sync across all files
node scripts/version-sync.js

# Or sync to a specific version
node scripts/version-sync.js 1.0.0
```

### Rust/Cargo not found

If you're building from source and get Rust errors:

1. Install Rust: https://rustup.rs/
2. Verify installation: `cargo --version`

## Contributing

Contributions are welcome! Please see our contributing guidelines (coming soon).

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `cargo test`
5. Submit a pull request

## License

MIT License - see [LICENSE](./LICENSE) file for details

## Links

- **npm Package**: https://www.npmjs.com/package/@doplan-dev/cli
- **Repository**: https://github.com/DoPlan-dev/CLI-new
- **Issues**: https://github.com/DoPlan-dev/CLI-new/issues
- **Releases**: https://github.com/DoPlan-dev/CLI-new/releases

## Development

### Building from Source

```bash
# Clone repository
git clone https://github.com/DoPlan-dev/CLI-new.git
cd CLI-new

# Build release binary
cargo build --release

# Run tests
cargo test

# Generate checksum
npm run checksum target/release/doplan
```

### Release Process

See [NPM_PUBLISH.md](./NPM_PUBLISH.md) for detailed release instructions.

Quick release:
```bash
npm run release 1.0.1
git push origin master --tags
# Create GitHub Release via UI or CLI
```

### Configuration

For GitHub secrets setup, see [docs/GITHUB_SECRETS.md](./docs/GITHUB_SECRETS.md).

