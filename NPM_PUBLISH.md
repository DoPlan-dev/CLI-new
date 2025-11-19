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

### Automated Version Synchronization

We provide automated scripts to keep versions synchronized across all files.

**Check version sync:**
```bash
npm run version:check
# or
node scripts/version-sync.js
```

**Sync versions to a specific version:**
```bash
npm run version:sync 1.0.1
# or
node scripts/version-sync.js 1.0.1
```

This automatically updates:
- `package.json` → `version`
- `Cargo.toml` → `version`
- `src/main.rs` → `#[command(version = "...")]`

### Automated Release Workflow

Use the release script for a streamlined release process:

```bash
npm run release
# or
node scripts/release.js [version]
```

The release script will:
1. ✅ Check git status (ensures clean working directory)
2. ✅ Prompt for new version (or use provided version)
3. ✅ Sync versions across all files
4. ✅ Run test suite
5. ✅ Build release binary
6. ✅ Run pre-publish validation
7. ✅ Verify npm package contents
8. ✅ Create git commit and tag (with confirmation)

**Example:**
```bash
# Interactive release
npm run release

# Or specify version directly
npm run release 1.0.1
```

### Manual Version Bump Workflow

If you prefer to do it manually:

```bash
# 1. Sync version in all files
npm run version:sync X.Y.Z

# 2. Commit version bump
git add package.json Cargo.toml src/main.rs
git commit -m "chore: bump version to X.Y.Z"

# 3. Create git tag
git tag vX.Y.Z
git push origin master --tags

# 4. Publish to npm (or let GitHub Actions handle it)
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

### Automated Publishing with GitHub Actions

We have a GitHub Actions workflow (`.github/workflows/publish-npm.yml`) that automatically publishes to npm when a GitHub Release is created.

**Workflow Features:**
- ✅ Triggers on GitHub Release creation
- ✅ Verifies version synchronization
- ✅ Runs test suite
- ✅ Builds release binary
- ✅ Validates package before publishing
- ✅ Publishes to npm
- ✅ Verifies published package

**Setup:**

1. **Create npm token:**
   - Go to https://www.npmjs.com/settings/YOUR_USERNAME/tokens
   - Create a new "Automation" token
   - Copy the token

2. **Add GitHub secret:**
   - Go to your GitHub repository settings
   - Navigate to Secrets → Actions
   - Add a new secret: `NPM_TOKEN` with your npm token

3. **Release workflow:**
   ```bash
   # Create release using our script
   npm run release 1.0.1
   
   # Push commit and tag
   git push origin master --tags
   
   # Create GitHub Release (via GitHub UI or CLI)
   gh release create v1.0.1 --title "v1.0.1" --notes "Release notes..."
   ```

   The GitHub Actions workflow will automatically:
   - Detect the release
   - Run all validations
   - Publish to npm
   - Verify the publication

**Manual Trigger:**

You can also manually trigger the workflow:
- Go to Actions tab in GitHub
- Select "Publish to npm" workflow
- Click "Run workflow"

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

## Binary Download Mechanism

The npm package includes an enhanced wrapper script (`bin/doplan.js`) that automatically handles binary distribution:

**Binary Resolution Order:**
1. Local development binary (`target/release/doplan`)
2. Binary in npm package directory
3. Cached binary in user's home directory (`~/.doplan/bin/VERSION/doplan`)
4. **Download from GitHub Releases** (automatic, if installed from npm)
5. Fallback to `cargo run` (development mode only)

When users install from npm, the wrapper script will:
- Automatically detect their platform (macOS, Linux, Windows)
- Download the correct binary from GitHub Releases
- Cache it for future use
- Verify and execute the binary

**For this to work:**
- GitHub Releases must contain binaries for all supported platforms
- Binary names should follow pattern: `doplan-{target-triple}` (e.g., `doplan-x86_64-apple-darwin`)
- Releases should be tagged as `v{version}` (e.g., `v1.0.0`)
- `checksums.txt` file must be included in the release for verification

**Automatic Binary Builds:**

The `.github/workflows/build-binaries.yml` workflow automatically:
- Builds binaries for all platforms when a release is created
- Generates SHA256 checksums for all binaries
- Creates a combined `checksums.txt` file
- Uploads everything to GitHub Releases

**Checksum Verification:**

When downloading binaries, the wrapper script:
1. Downloads the binary from GitHub Releases
2. Downloads the `checksums.txt` file
3. Verifies the binary's checksum matches the expected value
4. Warns if verification fails but continues (graceful degradation)

## Next Steps

✅ **Completed:**
- LICENSE file created
- Version sync script implemented
- Enhanced pre-publish validation
- npm installation instructions added to README
- GitHub Actions workflow for npm publishing
- Enhanced binary download wrapper
- Release automation script

**Completed Enhancements:**
1. ✅ Multi-platform binary builds (GitHub Actions matrix)
2. ✅ Checksum verification for downloaded binaries
3. ✅ Automated changelog generation
4. ✅ GitHub secrets configuration documentation

**Optional Future Enhancements:**
- Add beta/alpha release channels
- Binary GPG signing
- Advanced monitoring and analytics

---

**Ready to publish?** 

**Recommended workflow:**
```bash
# 1. Run automated release script (generates changelog automatically)
npm run release 1.0.1

# 2. Push to GitHub
git push origin master --tags

# 3. Create GitHub Release (triggers automated builds and npm publishing)
# Via GitHub CLI:
gh release create v1.0.1 --title "v1.0.1" --notes "$(cat CHANGELOG.md | sed -n '/## \[1.0.1\]/,/## \[/p' | head -n -1)"

# Or via GitHub UI:
# - Go to Releases → Draft a new release
# - Select tag: v1.0.1
# - Title: v1.0.1
# - Description: Copy from CHANGELOG.md
# - Publish release

# The workflows will automatically:
# 1. Build binaries for all platforms (build-binaries.yml)
# 2. Create GitHub Release with binaries and checksums
# 3. Publish to npm (publish-npm.yml)
```

**Generating Changelog:**
```bash
# Generate full changelog
npm run changelog

# Generate changelog for specific version
npm run changelog 1.0.1

# Generate to custom file
node scripts/generate-changelog.js 1.0.1 CHANGELOG.md
```

**Generating Checksums:**
```bash
# Generate checksum for a file
npm run checksum path/to/binary
```

**Manual publish:**
```bash
npm publish --access public
```

