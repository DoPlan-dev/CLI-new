#!/usr/bin/env node

/**
 * Version Synchronization Script
 * 
 * Synchronizes version number across all files:
 * - package.json (source of truth or target)
 * - Cargo.toml
 * - src/main.rs (clap version attribute)
 * 
 * Usage:
 *   node scripts/version-sync.js [version]
 *   If version is provided, updates all files to that version
 *   If not provided, reads from package.json and ensures all files match
 */

const fs = require('fs');
const path = require('path');

const ROOT = path.join(__dirname, '..');

// Semver regex
const SEMVER_REGEX = /^\d+\.\d+\.\d+(-[a-zA-Z0-9-]+)?(\+[a-zA-Z0-9-]+)?$/;

function validateVersion(version) {
  if (!SEMVER_REGEX.test(version)) {
    console.error(`Invalid version format: ${version}`);
    console.error('Version must follow semantic versioning (e.g., 1.0.0, 1.0.0-alpha.1)');
    process.exit(1);
  }
}

function readPackageJson() {
  const filePath = path.join(ROOT, 'package.json');
  return JSON.parse(fs.readFileSync(filePath, 'utf8'));
}

function writePackageJson(data) {
  const filePath = path.join(ROOT, 'package.json');
  fs.writeFileSync(filePath, JSON.stringify(data, null, 2) + '\n');
}

function updateCargoToml(version) {
  const filePath = path.join(ROOT, 'Cargo.toml');
  let content = fs.readFileSync(filePath, 'utf8');
  
  // Update version in [package] section
  content = content.replace(/^version = "([^"]+)"/m, `version = "${version}"`);
  
  fs.writeFileSync(filePath, content);
  console.log(`✓ Updated Cargo.toml: version = "${version}"`);
}

function updateMainRs(version) {
  const filePath = path.join(ROOT, 'src', 'main.rs');
  let content = fs.readFileSync(filePath, 'utf8');
  
  // Update clap version attribute
  content = content.replace(
    /#\[command\(version = "[^"]+"\)\]/,
    `#[command(version = "${version}")]`
  );
  
  fs.writeFileSync(filePath, content);
  console.log(`✓ Updated src/main.rs: #[command(version = "${version}")]`);
}

function checkVersionsMatch() {
  const packageJson = readPackageJson();
  const packageVersion = packageJson.version;
  
  // Read Cargo.toml
  const cargoToml = fs.readFileSync(path.join(ROOT, 'Cargo.toml'), 'utf8');
  const cargoMatch = cargoToml.match(/^version = "([^"]+)"/m);
  const cargoVersion = cargoMatch ? cargoMatch[1] : null;
  
  // Read src/main.rs
  const mainRs = fs.readFileSync(path.join(ROOT, 'src', 'main.rs'), 'utf8');
  const mainRsMatch = mainRs.match(/#\[command\(version = "([^"]+)"\)\]/);
  const mainRsVersion = mainRsMatch ? mainRsMatch[1] : null;
  
  const versions = {
    'package.json': packageVersion,
    'Cargo.toml': cargoVersion,
    'src/main.rs': mainRsVersion
  };
  
  const uniqueVersions = new Set(Object.values(versions).filter(v => v !== null));
  
  if (uniqueVersions.size === 1) {
    console.log(`✓ All versions match: ${packageVersion}`);
    return true;
  } else {
    console.error('✗ Version mismatch detected:');
    Object.entries(versions).forEach(([file, version]) => {
      console.error(`  ${file}: ${version || 'NOT FOUND'}`);
    });
    return false;
  }
}

function syncVersions(targetVersion) {
  validateVersion(targetVersion);
  
  console.log(`Syncing version to ${targetVersion}...`);
  
  // Update package.json
  const packageJson = readPackageJson();
  packageJson.version = targetVersion;
  writePackageJson(packageJson);
  console.log(`✓ Updated package.json: version = "${targetVersion}"`);
  
  // Update Cargo.toml
  updateCargoToml(targetVersion);
  
  // Update src/main.rs
  updateMainRs(targetVersion);
  
  console.log(`\n✓ All files synchronized to version ${targetVersion}`);
}

// Main execution
const args = process.argv.slice(2);

if (args.length === 0) {
  // Check mode: verify all versions match
  const allMatch = checkVersionsMatch();
  process.exit(allMatch ? 0 : 1);
} else if (args.length === 1) {
  // Sync mode: update all files to specified version
  const targetVersion = args[0];
  syncVersions(targetVersion);
} else {
  console.error('Usage: node scripts/version-sync.js [version]');
  console.error('  If version is provided, updates all files to that version');
  console.error('  If not provided, checks that all versions match');
  process.exit(1);
}

