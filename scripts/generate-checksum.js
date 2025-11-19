#!/usr/bin/env node

/**
 * Generate checksum script
 * 
 * Generates SHA256 checksum for a binary file
 * 
 * Usage:
 *   node scripts/generate-checksum.js <file>
 */

const crypto = require('crypto');
const fs = require('fs');
const path = require('path');

function generateChecksum(filePath) {
  if (!fs.existsSync(filePath)) {
    console.error(`Error: File not found: ${filePath}`);
    process.exit(1);
  }

  const fileBuffer = fs.readFileSync(filePath);
  const hashSum = crypto.createHash('sha256');
  hashSum.update(fileBuffer);
  const checksum = hashSum.digest('hex');

  return checksum;
}

function main() {
  const args = process.argv.slice(2);

  if (args.length === 0) {
    console.error('Usage: node scripts/generate-checksum.js <file>');
    process.exit(1);
  }

  const filePath = path.resolve(args[0]);
  const fileName = path.basename(filePath);
  const checksum = generateChecksum(filePath);

  console.log(`${checksum}  ${fileName}`);
  
  // Also output in format suitable for checksums.txt
  console.log(`\nChecksum: ${checksum}`);
  console.log(`File: ${fileName}`);
}

main();

