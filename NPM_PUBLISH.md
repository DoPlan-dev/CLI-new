# Publishing DoPlan CLI to npm

This guide explains how to publish DoPlan CLI to npm.

## Prerequisites

1. **npm account**: Create an account at https://www.npmjs.com/
2. **Login to npm**: Run `npm login` in your terminal
3. **Organization setup** (optional): If publishing as `@doplan-dev/cli`, ensure you have access to the `@doplan-dev` organization

## Pre-Publish Checklist

Before publishing, ensure:

- [ ] Version numbers match in `package.json` and `Cargo.toml`
- [ ] All tests pass: `cargo test`
- [ ] Build succeeds: `cargo build --release`
- [ ] README.md is up to date
- [ ] LICENSE file exists
- [ ] RELEASE_NOTES.md is updated

## Publishing Steps

### 1. Build the Release Binary

```bash
cargo build --release
```

This creates the binary at `target/release/doplan`.

### 2. Test the npm Package Locally

```bash
# Pack the package without publishing
npm pack

# Test install from the packed tarball
npm install -g ./doplan-dev-cli-1.0.0.tgz

# Test the command
doplan --version

# Uninstall test version
npm uninstall -g @doplan-dev/cli
```

### 3. Verify Package Contents

```bash
# Check what will be published
npm pack --dry-run
```

This shows exactly what files will be included in the npm package.

### 4. Publish to npm

#### First Time Publishing

```bash
# Make sure you're logged in
npm login

# Publish (will run prepublish script automatically)
npm publish --access public
```

#### Updating an Existing Package

```bash
# Update version in package.json
npm version patch   # for 1.0.0 -> 1.0.1
npm version minor   # for 1.0.0 -> 1.1.0
npm version major   # for 1.0.0 -> 2.0.0

# Or manually edit package.json and Cargo.toml, then:
git add package.json Cargo.toml src/main.rs
git commit -m "chore: bump version to X.Y.Z"
git tag vX.Y.Z
git push origin master --tags

# Then publish
npm publish
```

## Package Configuration

### Current Package Info

- **Name**: `@doplan-dev/cli`
- **Version**: `1.0.0`
- **Registry**: `https://registry.npmjs.org/`
- **Access**: `public`

### Files Included

The following files are included in the npm package (defined in `package.json`):

- `bin/doplan.js` - Node.js wrapper script
- `scripts/` - Build and install scripts
- `README.md` - Documentation
- `LICENSE` - License file
- `RELEASE_NOTES.md` - Release notes

### Files Excluded

The following are excluded via `.npmignore`:

- `target/` - Rust build artifacts
- `src/` - Source code (not needed in npm package)
- `tests/` - Test files
- `doplan/` - Project-specific files
- `.doplan/` - Configuration files

## Installation for Users

After publishing, users can install with:

```bash
# Global installation
npm install -g @doplan-dev/cli

# Or as a project dependency
npm install @doplan-dev/cli
```

## Binary Distribution Strategy

### Current Approach (Development)

The wrapper script (`bin/doplan.js`) currently:
1. Checks for a pre-built binary at `target/release/doplan`
2. Falls back to `cargo run` if binary not found

### Production Approach (Recommended)

For production, you should:

1. **Build binaries for all platforms** using `cargo-dist` or GitHub Actions
2. **Upload binaries to GitHub Releases** for each platform
3. **Update wrapper script** to download binaries from GitHub Releases

Example wrapper logic:
```javascript
// Download binary from GitHub releases if not present
const downloadUrl = `https://github.com/DoPlan-dev/CLI-new/releases/download/v${version}/doplan-${target}`;
```

## Version Management

### Synchronizing Versions

Keep these in sync:
- `package.json` → `version`
- `Cargo.toml` → `version`
- `src/main.rs` → `#[command(version = "...")]`
- Git tags → `v1.0.0`

### Version Bump Workflow

```bash
# 1. Update version in all files
# Edit: package.json, Cargo.toml, src/main.rs

# 2. Commit version bump
git add package.json Cargo.toml src/main.rs
git commit -m "chore: bump version to X.Y.Z"

# 3. Create git tag
git tag vX.Y.Z
git push origin master --tags

# 4. Publish to npm
npm publish
```

## Troubleshooting

### "Package name already exists"

If the package name is taken:
- Choose a different name in `package.json`
- Or request access to the organization

### "You do not have permission to publish"

- Ensure you're logged in: `npm whoami`
- Check organization permissions if using scoped package
- Verify you own the package name

### "Pre-publish validation failed"

- Run `npm run build` manually
- Check that all required files exist
- Verify version numbers match

### Binary not found after install

- Users need Rust installed to build from source
- Or provide pre-built binaries via GitHub Releases
- Update wrapper script to download binaries automatically

## CI/CD Integration

### GitHub Actions Example

```yaml
name: Publish to npm

on:
  release:
    types: [created]

jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
        with:
          node-version: '18'
          registry-url: 'https://registry.npmjs.org'
      - run: npm ci
      - run: npm publish
        env:
          NODE_AUTH_TOKEN: ${{secrets.NPM_TOKEN}}
```

## Post-Publish

After publishing:

1. **Verify installation**:
   ```bash
   npm install -g @doplan-dev/cli
   doplan --version
   ```

2. **Update documentation** with npm installation instructions

3. **Announce the release** on:
   - GitHub Releases
   - Project README
   - Social media / community channels

## Useful Commands

```bash
# Check npm login status
npm whoami

# View package info
npm view @doplan-dev/cli

# Unpublish (within 72 hours)
npm unpublish @doplan-dev/cli@1.0.0

# Deprecate a version
npm deprecate @doplan-dev/cli@1.0.0 "Use v1.0.1 instead"
```

## Next Steps

1. Set up GitHub Actions for automated publishing
2. Create pre-built binaries for all platforms
3. Update wrapper script to download binaries
4. Add installation instructions to README
5. Set up automated version bumping

---

**Ready to publish?** Run `npm publish --access public` when ready!

