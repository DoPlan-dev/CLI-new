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

// Check version synchronization across all files
try {
  const packageJson = JSON.parse(fs.readFileSync(path.join(__dirname, '..', 'package.json'), 'utf8'));
  const packageVersion = packageJson.version;
  
  // Check Cargo.toml
  const cargoToml = fs.readFileSync(path.join(__dirname, '..', 'Cargo.toml'), 'utf8');
  const cargoMatch = cargoToml.match(/^version = "([^"]+)"/m);
  const cargoVersion = cargoMatch ? cargoMatch[1] : null;
  
  if (!cargoVersion) {
    console.error('Could not find version in Cargo.toml');
    hasErrors = true;
  } else if (cargoVersion !== packageVersion) {
    console.error(`Version mismatch: package.json has ${packageVersion}, Cargo.toml has ${cargoVersion}`);
    hasErrors = true;
  }
  
  // Check src/main.rs
  const mainRsPath = path.join(__dirname, '..', 'src', 'main.rs');
  if (fs.existsSync(mainRsPath)) {
    const mainRs = fs.readFileSync(mainRsPath, 'utf8');
    const mainRsMatch = mainRs.match(/#\[command\(version = "([^"]+)"\)\]/);
    const mainRsVersion = mainRsMatch ? mainRsMatch[1] : null;
    
    if (!mainRsVersion) {
      console.error('Could not find version in src/main.rs');
      hasErrors = true;
    } else if (mainRsVersion !== packageVersion) {
      console.error(`Version mismatch: package.json has ${packageVersion}, src/main.rs has ${mainRsVersion}`);
      hasErrors = true;
    }
  }
  
  if (!hasErrors) {
    console.log(`✓ All versions synchronized: ${packageVersion}`);
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

