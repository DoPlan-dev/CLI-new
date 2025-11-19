#!/usr/bin/env node

/**
 * Post-install script
 * Runs after npm install to set up the binary
 */

const fs = require('fs');
const path = require('path');
const os = require('os');

const binPath = path.join(__dirname, '..', 'bin', 'doplan.js');

// Make the script executable on Unix-like systems
if (os.platform() !== 'win32') {
  try {
    fs.chmodSync(binPath, '755');
  } catch (err) {
    console.warn('Could not make doplan.js executable:', err.message);
  }
}

console.log('DoPlan CLI installed successfully!');
console.log('Run "doplan --help" to get started.');

