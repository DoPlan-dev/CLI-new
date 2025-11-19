# Publish

## Overview
Launch a package publishing wizard to publish your project to package registries.

## Workflow
1. Detect package type (npm, Homebrew, Scoop, Winget, etc.)
2. Show publishing options:
   - npm (for Node.js packages)
   - Homebrew (for macOS/Linux CLI tools)
   - Scoop (for Windows CLI tools)
   - Winget (for Windows apps)
3. Validate package configuration
4. Build package
5. Publish to registry
6. Provide installation instructions

## Package Types
- npm packages
- Homebrew formulas
- Scoop manifests
- Winget manifests
