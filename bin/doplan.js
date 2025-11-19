#!/usr/bin/env node

/**
 * DoPlan CLI - Node.js Wrapper
 * 
 * This wrapper script detects the platform and architecture,
 * then executes the appropriate Rust binary.
 * 
 * Binary Resolution Order:
 * 1. Check for local development binary (target/release/doplan)
 * 2. Check for binary in npm package directory
 * 3. Check for cached binary in user's home directory
 * 4. Download from GitHub Releases (if installed from npm)
 * 5. Fallback to cargo run (development mode only)
 */

const { spawn } = require('child_process');
const https = require('https');
const http = require('http');
const path = require('path');
const fs = require('fs');
const os = require('os');
const crypto = require('crypto');

// Get package version
const packageJsonPath = path.join(__dirname, '..', 'package.json');
let packageVersion = '1.0.0';
try {
  const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8'));
  packageVersion = packageJson.version;
} catch (err) {
  // Fallback to default version
}

// Repository information
const GITHUB_REPO = 'DoPlan-dev/CLI-new';
const GITHUB_RELEASES_URL = `https://github.com/${GITHUB_REPO}/releases`;

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
  console.error('Supported platforms: macOS, Linux, Windows (x64, arm64)');
  process.exit(1);
}

const target = `${rustArch}-${rustPlatform}`;
const binaryName = platform === 'win32' ? 'doplan.exe' : 'doplan';

// Binary location priorities
function getBinaryPaths() {
  const paths = [];
  
  // 1. Local development binary
  paths.push(path.join(__dirname, '..', 'target', 'release', binaryName));
  
  // 2. npm package directory (if installed via npm)
  if (__dirname.includes('node_modules')) {
    paths.push(path.join(__dirname, '..', 'bin', binaryName));
  }
  
  // 3. Cached binary in user's home directory
  const homeDir = os.homedir();
  const cacheDir = path.join(homeDir, '.doplan', 'bin', packageVersion);
  paths.push(path.join(cacheDir, binaryName));
  
  return paths;
}

// Check if binary exists and is executable
function binaryExists(binaryPath) {
  try {
    if (!fs.existsSync(binaryPath)) {
      return false;
    }
    
    // Make executable on Unix-like systems
    if (platform !== 'win32') {
      fs.chmodSync(binaryPath, '755');
    }
    
    return true;
  } catch (err) {
    return false;
  }
}

// Calculate SHA256 checksum of a file
function calculateChecksum(filePath) {
  const fileBuffer = fs.readFileSync(filePath);
  const hashSum = crypto.createHash('sha256');
  hashSum.update(fileBuffer);
  return hashSum.digest('hex');
}

// Download a file from URL
function downloadFile(downloadUrl, destinationPath) {
  return new Promise((resolve, reject) => {
    const file = fs.createWriteStream(destinationPath);
    const protocol = downloadUrl.startsWith('https') ? https : http;
    
    protocol.get(downloadUrl, (response) => {
      if (response.statusCode === 302 || response.statusCode === 301) {
        // Follow redirect
        file.close();
        if (fs.existsSync(destinationPath)) {
          fs.unlinkSync(destinationPath);
        }
        return downloadFile(response.headers.location, destinationPath)
          .then(resolve)
          .catch(reject);
      }
      
      if (response.statusCode !== 200) {
        file.close();
        if (fs.existsSync(destinationPath)) {
          fs.unlinkSync(destinationPath);
        }
        reject(new Error(`Failed to download: ${response.statusCode} ${response.statusMessage}`));
        return;
      }
      
      const totalSize = parseInt(response.headers['content-length'] || '0', 10);
      let downloadedSize = 0;
      
      response.on('data', (chunk) => {
        downloadedSize += chunk.length;
        if (totalSize > 0) {
          const percent = ((downloadedSize / totalSize) * 100).toFixed(1);
          process.stderr.write(`\rDownloading... ${percent}%`);
        }
      });
      
      response.pipe(file);
      
      file.on('finish', () => {
        file.close();
        resolve(destinationPath);
      });
    }).on('error', (err) => {
      file.close();
      if (fs.existsSync(destinationPath)) {
        fs.unlinkSync(destinationPath);
      }
      reject(err);
    });
  });
}

// Download and verify checksum file
async function downloadChecksums(checksumsUrl) {
  const tempChecksumsPath = path.join(os.tmpdir(), `doplan-checksums-${Date.now()}.txt`);
  
  try {
    await downloadFile(checksumsUrl, tempChecksumsPath);
    const checksumsContent = fs.readFileSync(tempChecksumsPath, 'utf8');
    fs.unlinkSync(tempChecksumsPath);
    return checksumsContent;
  } catch (err) {
    if (fs.existsSync(tempChecksumsPath)) {
      fs.unlinkSync(tempChecksumsPath);
    }
    throw err;
  }
}

// Verify binary checksum
function verifyChecksum(binaryPath, expectedChecksum, assetName) {
  try {
    const actualChecksum = calculateChecksum(binaryPath);
    
    if (actualChecksum.toLowerCase() !== expectedChecksum.toLowerCase()) {
      console.error('\n✗ Checksum verification failed!');
      console.error(`  Expected: ${expectedChecksum}`);
      console.error(`  Actual:   ${actualChecksum}`);
      console.error('\nThis may indicate a corrupted or tampered file.');
      console.error('Please try downloading again or report this issue.');
      return false;
    }
    
    console.log(`✓ Checksum verified: ${assetName}`);
    return true;
  } catch (err) {
    console.error(`Error verifying checksum: ${err.message}`);
    return false;
  }
}

// Parse checksums file to find expected checksum for asset
function parseChecksums(checksumsContent, assetName) {
  const lines = checksumsContent.split('\n');
  
  for (const line of lines) {
    // Skip comments and empty lines
    if (line.trim().startsWith('#') || !line.trim()) {
      continue;
    }
    
    // Format: <checksum>  <filename> or <checksum><spaces><filename>
    const match = line.match(/^([a-fA-F0-9]{64})\s+(.+)$/);
    if (match) {
      const checksum = match[1];
      const filename = match[2].trim();
      
      // Match exact filename or just the asset name
      if (filename === assetName || filename.endsWith(assetName) || assetName.endsWith(filename)) {
        return checksum;
      }
    }
  }
  
  return null;
}

// Download binary from GitHub Releases with checksum verification
async function downloadBinary(downloadUrl, destinationPath, assetName) {
  console.log(`Downloading binary from GitHub Releases...`);
  console.log(`Platform: ${target}`);
  
  try {
    // Download the binary
    await downloadFile(downloadUrl, destinationPath);
    
    // Try to download and verify checksum
    try {
      const checksumsUrl = `${GITHUB_RELEASES_URL}/download/v${packageVersion}/checksums.txt`;
      console.log('\nDownloading checksums...');
      const checksumsContent = await downloadChecksums(checksumsUrl);
      
      const expectedChecksum = parseChecksums(checksumsContent, assetName);
      
      if (expectedChecksum) {
        if (!verifyChecksum(destinationPath, expectedChecksum, assetName)) {
          // Checksum verification failed - delete the file
          if (fs.existsSync(destinationPath)) {
            fs.unlinkSync(destinationPath);
          }
          throw new Error('Checksum verification failed');
        }
      } else {
        console.warn(`⚠ Warning: Checksum not found for ${assetName}, skipping verification`);
      }
    } catch (checksumErr) {
      // If checksum download/verification fails, warn but don't fail
      console.warn(`⚠ Warning: Could not verify checksum: ${checksumErr.message}`);
      console.warn('Binary downloaded but not verified. Use at your own risk.');
    }
    
    // Make executable on Unix-like systems
    if (platform !== 'win32') {
      fs.chmodSync(destinationPath, '755');
    }
    
    console.log('✓ Binary downloaded successfully');
    return destinationPath;
  } catch (err) {
    // Clean up on error
    if (fs.existsSync(destinationPath)) {
      fs.unlinkSync(destinationPath);
    }
    throw err;
  }
}

// Try to find or download binary
async function getBinary() {
  const paths = getBinaryPaths();
  
  // Check existing binaries
  for (const binaryPath of paths) {
    if (binaryExists(binaryPath)) {
      return binaryPath;
    }
  }
  
  // If in npm package context, try to download from GitHub Releases
  // Only attempt download if we're in node_modules (installed from npm)
  const isNpmInstall = __dirname.includes('node_modules');
  
  if (isNpmInstall) {
    try {
      // Try to download from GitHub Releases
      const cacheDir = path.join(os.homedir(), '.doplan', 'bin', packageVersion);
      if (!fs.existsSync(cacheDir)) {
        fs.mkdirSync(cacheDir, { recursive: true });
      }
      
      const cachePath = path.join(cacheDir, binaryName);
      
      // Try different possible release asset names
      const possibleNames = [
        `doplan-${target}`,
        `doplan-${target}.exe`,
        `doplan-${target}-${packageVersion}`,
        `doplan-${target}-${packageVersion}.exe`
      ];
      
      for (const assetName of possibleNames) {
        const downloadUrl = `${GITHUB_RELEASES_URL}/download/v${packageVersion}/${assetName}`;
        
        try {
          await downloadBinary(downloadUrl, cachePath, assetName);
          if (binaryExists(cachePath)) {
            return cachePath;
          }
        } catch (err) {
          // Try next possible name
          continue;
        }
      }
      
      throw new Error('Could not download binary from GitHub Releases');
    } catch (err) {
      console.error(`\nWarning: Could not download binary: ${err.message}`);
      console.error(`Falling back to development mode...`);
    }
  }
  
  // Fallback to cargo run (development mode)
  // Only allow this if we're in development (not in node_modules)
  if (!isNpmInstall) {
    console.warn('Binary not found. Running with cargo run (development mode)...');
    return null; // Signal to use cargo run
  }
  
  // If we're in npm context and download failed, show error
  console.error('\nError: Binary not found and could not be downloaded.');
  console.error(`Please ensure you have Rust installed, or download the binary manually from:`);
  console.error(`${GITHUB_RELEASES_URL}`);
  console.error(`\nSupported target: ${target}`);
  process.exit(1);
}

// Execute binary
function executeBinary(binaryPath, args) {
  const child = spawn(binaryPath, args, {
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
}

// Execute with cargo run (development mode)
function executeWithCargo(args) {
  const child = spawn('cargo', ['run', '--', ...args], {
    stdio: 'inherit',
    env: process.env,
    cwd: path.join(__dirname, '..')
  });
  
  child.on('error', (err) => {
    console.error('Failed to start doplan:', err);
    console.error('Make sure Rust and Cargo are installed: https://rustup.rs/');
    process.exit(1);
  });
  
  child.on('exit', (code) => {
    process.exit(code || 0);
  });
}

// Main execution
(async () => {
  const args = process.argv.slice(2);
  const binary = await getBinary();
  
  if (binary) {
    executeBinary(binary, args);
  } else {
    executeWithCargo(args);
  }
})();

