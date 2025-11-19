#!/usr/bin/env node

/**
 * Build script for DoPlan CLI
 * Builds the Rust binary for the current platform
 */

const { spawn } = require('child_process');
const path = require('path');

console.log('Building DoPlan CLI...');

const buildProcess = spawn('cargo', ['build', '--release'], {
  stdio: 'inherit',
  cwd: path.join(__dirname, '..')
});

buildProcess.on('error', (err) => {
  console.error('Build failed:', err);
  process.exit(1);
});

buildProcess.on('exit', (code) => {
  if (code !== 0) {
    console.error('Build failed with exit code:', code);
    process.exit(code);
  }
  console.log('Build completed successfully!');
});

