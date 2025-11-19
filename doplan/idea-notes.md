# DoPlan CLI - npm Package Release

## Overview
Comprehensive strategy for publishing DoPlan CLI as an npm package (`@doplan-dev/cli`) with automated workflows, multi-platform binary distribution, and streamlined release process.

## Current State Analysis

### ✅ Completed
- `package.json` configured with proper metadata
- Node.js wrapper script (`bin/doplan.js`) for cross-platform execution
- Pre-publish validation script (`scripts/prepublish.js`)
- Build script (`scripts/build.js`)
- Post-install script (`scripts/postinstall.js`)
- `.npmignore` properly configured
- `NPM_PUBLISH.md` documentation exists
- Version synchronized in `package.json` (1.0.0) and `Cargo.toml` (1.0.0)
- `RELEASE_NOTES.md` updated with v1.0.0 content

### ❌ Missing/Gaps
1. **LICENSE file** - Required for npm publishing
2. **Automated version sync** - Manual process, error-prone
3. **CI/CD for npm publishing** - No GitHub Actions workflow
4. **Multi-platform binary distribution** - No cargo-dist or automated builds
5. **Binary download mechanism** - Wrapper doesn't download from GitHub releases
6. **Automated release workflow** - Manual version bumping and publishing
7. **npm installation instructions** - Missing from README.md
8. **Version sync script** - No automated way to sync versions across files
9. **Binary verification** - No checksums or signature verification
10. **Release automation** - No automated changelog generation

## Enhanced Features & Improvements

### 1. Automated Version Management
**Goal**: Single source of truth for version, automatic sync across all files

**Implementation**:
- Create `scripts/version-sync.js` to sync versions across:
  - `package.json`
  - `Cargo.toml`
  - `src/main.rs` (clap version)
  - Generate version from git tags or allow manual input
- Add npm script: `npm run version:sync X.Y.Z`
- Integrate with `npm version` command hooks

**Benefits**:
- Eliminates version mismatch errors
- Reduces manual work
- Prevents publishing with wrong versions

### 2. Multi-Platform Binary Distribution
**Goal**: Distribute pre-built binaries for all supported platforms

**Implementation**:
- Use `cargo-dist` for automated binary builds
- Or use GitHub Actions matrix builds:
  - macOS (x86_64, arm64)
  - Linux (x86_64, arm64)
  - Windows (x86_64)
- Upload binaries to GitHub Releases
- Generate checksums for verification
- Optionally sign binaries with GPG

**Platforms**:
- `x86_64-apple-darwin` (macOS Intel)
- `aarch64-apple-darwin` (macOS Apple Silicon)
- `x86_64-unknown-linux-gnu` (Linux x64)
- `aarch64-unknown-linux-gnu` (Linux ARM64)
- `x86_64-pc-windows-msvc` (Windows x64)

**Benefits**:
- Users don't need Rust installed
- Faster installation
- Professional distribution

### 3. Enhanced Binary Download Wrapper
**Goal**: Automatically download correct binary for user's platform

**Implementation**:
- Update `bin/doplan.js` to:
  1. Check for local binary (development)
  2. Check npm package location for pre-installed binary
  3. Download from GitHub Releases if missing
  4. Verify checksum
  5. Cache downloaded binary
  6. Fallback to `cargo run` only in development mode

**Features**:
- Progress bar during download
- Checksum verification
- Automatic retry on failure
- Platform detection
- Version-aware downloads

**Benefits**:
- Seamless user experience
- Automatic binary updates
- Secure downloads

### 4. CI/CD Publishing Workflow
**Goal**: Automated npm publishing on releases

**Implementation**:
- Create `.github/workflows/publish-npm.yml`:
  - Trigger on GitHub release creation
  - Build binaries for all platforms
  - Create GitHub Release with artifacts
  - Publish to npm
  - Verify installation

**Workflow Steps**:
1. Checkout code
2. Setup Rust and Node.js
3. Sync versions (verify consistency)
4. Run tests
5. Build release binaries (all platforms)
6. Package npm bundle
7. Create GitHub Release
8. Upload binaries to release
9. Publish to npm
10. Verify published package

**Benefits**:
- Zero-touch releases
- Consistent builds
- Automated testing

### 5. Pre-Release Validation
**Goal**: Comprehensive checks before publishing

**Enhancements to `scripts/prepublish.js`**:
- Verify LICENSE file exists
- Check version sync across all files
- Validate binary exists or can be downloaded
- Check all required files present
- Verify git tag exists for version
- Check CHANGELOG.md updated
- Run test suite
- Check for uncommitted changes

**Benefits**:
- Prevents publishing broken packages
- Ensures quality standards

### 6. Enhanced Documentation
**Goal**: Clear installation and usage instructions

**README.md Updates**:
- Add npm installation section
- Add quick start guide
- Add troubleshooting section
- Link to full documentation
- Show version badge
- Add npm package badge

**NPM_PUBLISH.md Enhancements**:
- Add automated workflow guide
- Add cargo-dist setup
- Add version management guide
- Add troubleshooting for common issues

**Benefits**:
- Better developer experience
- Reduced support burden

### 7. Release Automation Scripts
**Goal**: Streamline release process

**Scripts to Create**:
- `scripts/release.js` - Full release workflow:
  1. Check current version
  2. Prompt for new version
  3. Update versions across files
  4. Run tests
  5. Build binaries
  6. Create git commit and tag
  7. Push to GitHub
  8. Trigger release workflow

**Benefits**:
- One-command releases
- Reduced human error

### 8. Package Metadata Enhancement
**Goal**: Better discoverability on npm

**Improvements**:
- Add `homepage` field
- Add `bugs` field
- Add `funding` field (optional)
- Enhance `keywords` for better search
- Add `browser` field if applicable
- Add `types` field if TypeScript definitions added

**Benefits**:
- Better npm search ranking
- More professional appearance

### 9. Security & Verification
**Goal**: Ensure secure binary distribution

**Implementation**:
- Generate SHA256 checksums for all binaries
- Optionally GPG sign binaries
- Verify checksums in wrapper before execution
- Add security advisories workflow

**Benefits**:
- User trust
- Security compliance
- Prevents tampering

### 10. Monitoring & Analytics
**Goal**: Track package usage and issues

**Implementation**:
- Add npm package analytics (built-in)
- Monitor download statistics
- Track error reports (if implemented)
- Version adoption tracking

**Benefits**:
- Understand user base
- Prioritize improvements

## Technical Specifications

### File Structure
```
.
├── bin/
│   └── doplan.js (enhanced with download logic)
├── scripts/
│   ├── version-sync.js (NEW)
│   ├── release.js (NEW)
│   ├── build-binary.js (NEW - for GitHub Actions)
│   ├── download-binary.js (NEW - extracted logic)
│   ├── build.js (existing)
│   ├── postinstall.js (existing)
│   └── prepublish.js (enhanced)
├── .github/
│   └── workflows/
│       ├── publish-npm.yml (NEW)
│       ├── build-binaries.yml (NEW)
│       └── rust.yml (existing)
├── LICENSE (NEW - MIT)
├── package.json (enhanced)
├── CHANGELOG.md (NEW - optional)
└── README.md (enhanced)
```

### Version Sync Script Specification
- Reads version from `package.json` as source of truth
- Updates `Cargo.toml` version field
- Updates `src/main.rs` clap version attribute
- Can also read from git tag or user input
- Validates version format (semver)

### Binary Download Logic
```javascript
1. Check for binary at: node_modules/@doplan-dev/cli/bin/doplan-<target>
2. If not found, check cache: ~/.doplan/bin/<version>/doplan-<target>
3. If not found, download from:
   https://github.com/DoPlan-dev/CLI-new/releases/download/v<version>/doplan-<target>
4. Verify checksum from checksums.txt
5. Cache binary for future use
6. Make executable
7. Execute binary
```

### CI/CD Workflow Specification

#### publish-npm.yml
```yaml
Triggers:
  - release: created
  - workflow_dispatch (manual)

Jobs:
  1. build-binaries:
     - Matrix: [macos, ubuntu, windows] x [x64, arm64]
     - Build release binary
     - Generate checksum
     - Upload artifact
  
  2. create-release:
     - Download all binaries
     - Create GitHub Release
     - Upload binaries with checksums
  
  3. publish-npm:
     - Package npm bundle
     - Validate package
     - Publish to npm
     - Verify installation
```

## Implementation Priority

### Phase 1: Critical (Required for First Release)
1. ✅ Create LICENSE file
2. ✅ Enhanced prepublish validation
3. ✅ Version sync script
4. ✅ Update README with npm instructions
5. ✅ Basic npm publishing workflow

### Phase 2: Important (Professional Release)
6. ✅ Multi-platform binary builds
7. ✅ Binary download mechanism
8. ✅ Enhanced wrapper script
9. ✅ CI/CD publishing workflow
10. ✅ Release automation script

### Phase 3: Enhancement (Future)
11. Binary signing
12. Advanced monitoring
13. Automated changelog
14. Beta/alpha release channels

## Risk Mitigation

### Identified Risks
1. **Version Mismatch**: Automated sync script prevents this
2. **Missing Binaries**: Download fallback handles gracefully
3. **Publishing Broken Package**: Comprehensive validation prevents
4. **Platform Compatibility**: Matrix testing ensures compatibility
5. **npm Publishing Failures**: Manual fallback process documented

### Mitigation Strategies
- Pre-release testing checklist
- Dry-run publish testing (`npm pack`)
- Version pinning in CI/CD
- Rollback procedures documented
- Communication plan for issues

## Success Metrics

### Release Readiness Checklist
- [ ] LICENSE file exists
- [ ] All tests passing
- [ ] Version synced across all files
- [ ] Binaries built for all platforms
- [ ] README updated with npm instructions
- [ ] Pre-publish validation passing
- [ ] CI/CD workflow tested
- [ ] Release notes updated
- [ ] Package tested locally with `npm pack`

### Post-Release Validation
- [ ] Package appears on npm
- [ ] Installation works: `npm install -g @doplan-dev/cli`
- [ ] Binary downloads correctly
- [ ] Command executes: `doplan --version`
- [ ] All platforms verified
- [ ] GitHub Release created with binaries

## Next Steps

1. **Immediate Actions**:
   - Create LICENSE file (MIT)
   - Create version-sync script
   - Update README.md with npm instructions
   - Enhance prepublish.js validation

2. **Short Term**:
   - Set up cargo-dist or GitHub Actions for binaries
   - Enhance bin/doplan.js with download logic
   - Create publish-npm.yml workflow
   - Test full release workflow

3. **Before First Release**:
   - Test `npm pack` locally
   - Verify installation on all platforms
   - Create comprehensive test plan
   - Document release process

## References

- [npm Publishing Guide](https://docs.npmjs.com/packages-and-modules/contributing-packages-to-the-registry)
- [cargo-dist Documentation](https://opensource.axo.dev/cargo-dist/)
- [GitHub Actions for npm](https://docs.github.com/en/actions/publishing-packages/publishing-nodejs-packages)
- [Semantic Versioning](https://semver.org/)
- [Node.js Binary Wrapper Pattern](https://github.com/vercel/pkg)

