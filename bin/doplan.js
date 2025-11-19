#!/usr/bin/env node

/**
 * DoPlan CLI - Node.js Wrapper
 * 
 * This wrapper script detects the platform and architecture,
 * then executes the appropriate Rust binary.
 */

const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');
const os = require('os');

// Get platform and architecture
const platform = os.platform();
const arch = os.arch();

// Map Node.js platform names to Rust target names
const platformMap = {
  'darwin': 'apple-darwin',
  'linux': 'unknown-linux-gnu',
  'win32': 'pc-windows-msvc'
};

// Map Node.js architecture names to Rust target names
const archMap = {
  'x64': 'x86_64',
  'arm64': 'aarch64'
};

// Get Rust target triple
const rustPlatform = platformMap[platform];
const rustArch = archMap[arch];

if (!rustPlatform || !rustArch) {
  console.error(`Unsupported platform: ${platform} ${arch}`);
  process.exit(1);
}

const target = `${rustArch}-${rustPlatform}`;

// Path to binary (for now, we'll use cargo run or installed binary)
// In production, this would download from GitHub releases or use pre-built binary
const binaryPath = path.join(__dirname, '..', 'target', 'release', 'doplan');

// Check if binary exists, otherwise try cargo run
if (fs.existsSync(binaryPath)) {
  // Use pre-built binary
  const child = spawn(binaryPath, process.argv.slice(2), {
    stdio: 'inherit',
    env: process.env
  });
  
  child.on('error', (err) => {
    console.error('Failed to start doplan:', err);
    process.exit(1);
  });
  
  child.on('exit', (code) => {
    process.exit(code || 0);
  });
} else {
  // Fallback to cargo run (development mode)
  console.warn('Binary not found. Running with cargo run (development mode)...');
  const child = spawn('cargo', ['run', '--', ...process.argv.slice(2)], {
    stdio: 'inherit',
    env: process.env,
    cwd: path.join(__dirname, '..')
  });
  
  child.on('error', (err) => {
    console.error('Failed to start doplan:', err);
    console.error('Make sure Rust and Cargo are installed.');
    process.exit(1);
  });
  
  child.on('exit', (code) => {
    process.exit(code || 0);
  });
}

