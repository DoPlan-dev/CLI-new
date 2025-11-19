#!/usr/bin/env node

/**
 * Pre-publish validation script
 * Runs before npm publish to ensure everything is ready
 */

const fs = require('fs');
const path = require('path');

console.log('Validating package for publish...');

// Check required files
const requiredFiles = [
  'package.json',
  'bin/doplan.js',
  'README.md',
  'LICENSE',
  'Cargo.toml'
];

let hasErrors = false;

for (const file of requiredFiles) {
  const filePath = path.join(__dirname, '..', file);
  if (!fs.existsSync(filePath)) {
    console.error(`Missing required file: ${file}`);
    hasErrors = true;
  }
}

// Check if binary exists (optional, as it may be downloaded at runtime)
const binaryPath = path.join(__dirname, '..', 'target', 'release', 'doplan');
if (!fs.existsSync(binaryPath)) {
  console.warn('Warning: Binary not found. Users will need to build it or download from releases.');
}

// Check version in package.json matches Cargo.toml
try {
  const packageJson = JSON.parse(fs.readFileSync(path.join(__dirname, '..', 'package.json'), 'utf8'));
  const cargoToml = fs.readFileSync(path.join(__dirname, '..', 'Cargo.toml'), 'utf8');
  const versionMatch = cargoToml.match(/version = "([^"]+)"/);
  
  if (versionMatch && versionMatch[1] !== packageJson.version) {
    console.error(`Version mismatch: package.json has ${packageJson.version}, Cargo.toml has ${versionMatch[1]}`);
    hasErrors = true;
  }
} catch (err) {
  console.error('Error checking versions:', err.message);
  hasErrors = true;
}

if (hasErrors) {
  console.error('Pre-publish validation failed!');
  process.exit(1);
}

console.log('Pre-publish validation passed!');

