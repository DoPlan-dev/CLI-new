#!/usr/bin/env node

/**
 * Release Automation Script
 * 
 * Streamlines the release process by:
 * 1. Checking current version
 * 2. Prompting for new version
 * 3. Syncing versions across all files
 * 4. Running tests
 * 5. Building release binary
 * 6. Creating git commit and tag
 * 7. Preparing for push
 * 
 * Usage:
 *   node scripts/release.js [version]
 *   If version is provided, uses that version
 *   If not provided, prompts for version
 */

const { spawn, execSync } = require('child_process');
const fs = require('fs');
const path = require('path');
const readline = require('readline');

const ROOT = path.join(__dirname, '..');

// Semver regex
const SEMVER_REGEX = /^\d+\.\d+\.\d+(-[a-zA-Z0-9-]+)?(\+[a-zA-Z0-9-]+)?$/;

function exec(command, options = {}) {
  try {
    return execSync(command, { 
      cwd: ROOT,
      stdio: options.silent ? 'pipe' : 'inherit',
      encoding: 'utf8',
      ...options
    });
  } catch (err) {
    console.error(`Error executing: ${command}`);
    throw err;
  }
}

function readPackageJson() {
  const filePath = path.join(ROOT, 'package.json');
  return JSON.parse(fs.readFileSync(filePath, 'utf8'));
}

function promptVersion(currentVersion) {
  return new Promise((resolve) => {
    const rl = readline.createInterface({
      input: process.stdin,
      output: process.stdout
    });

    console.log(`\nCurrent version: ${currentVersion}`);
    console.log('Enter new version (or press Enter to abort):');
    rl.question('> ', (answer) => {
      rl.close();
      if (!answer.trim()) {
        console.log('Aborted.');
        process.exit(0);
      }
      resolve(answer.trim());
    });
  });
}

function validateVersion(version) {
  if (!SEMVER_REGEX.test(version)) {
    console.error(`Invalid version format: ${version}`);
    console.error('Version must follow semantic versioning (e.g., 1.0.0, 1.0.1, 1.1.0, 2.0.0)');
    process.exit(1);
  }
}

function checkGitStatus() {
  try {
    const status = exec('git status --porcelain', { silent: true });
    if (status.trim()) {
      console.error('Error: You have uncommitted changes.');
      console.error('Please commit or stash your changes before releasing.');
      console.error('\nUncommitted changes:');
      console.log(status);
      process.exit(1);
    }
  } catch (err) {
    console.error('Error checking git status:', err.message);
    process.exit(1);
  }
}

function checkGitBranch() {
  try {
    const branch = exec('git branch --show-current', { silent: true }).trim();
    if (branch !== 'master' && branch !== 'main') {
      console.warn(`Warning: You are not on master/main branch (current: ${branch})`);
      const rl = readline.createInterface({
        input: process.stdin,
        output: process.stdout
      });
      rl.question('Continue anyway? (y/N): ', (answer) => {
        rl.close();
        if (answer.toLowerCase() !== 'y') {
          console.log('Aborted.');
          process.exit(0);
        }
      });
    }
  } catch (err) {
    console.error('Error checking git branch:', err.message);
    process.exit(1);
  }
}

async function main() {
  console.log('🚀 DoPlan CLI Release Automation\n');

  // Check git status
  console.log('1. Checking git status...');
  checkGitStatus();
  checkGitBranch();
  console.log('✓ Git status clean\n');

  // Get current version
  const packageJson = readPackageJson();
  const currentVersion = packageJson.version;
  console.log(`Current version: ${currentVersion}`);

  // Get target version
  const args = process.argv.slice(2);
  let targetVersion;
  
  if (args.length > 0) {
    targetVersion = args[0];
  } else {
    targetVersion = await promptVersion(currentVersion);
  }

  validateVersion(targetVersion);

  if (targetVersion === currentVersion) {
    console.error(`Error: New version (${targetVersion}) must be different from current version (${currentVersion})`);
    process.exit(1);
  }

  console.log(`\n2. Syncing version to ${targetVersion}...`);
  exec('node scripts/version-sync.js ' + targetVersion);
  console.log('✓ Versions synchronized\n');

  // Run tests
  console.log('3. Running tests...');
  try {
    exec('cargo test --verbose');
    console.log('✓ All tests passed\n');
  } catch (err) {
    console.error('✗ Tests failed. Aborting release.');
    process.exit(1);
  }

  // Build release binary
  console.log('4. Building release binary...');
  try {
    exec('cargo build --release --verbose');
    const binaryPath = path.join(ROOT, 'target', 'release', 'doplan');
    if (!fs.existsSync(binaryPath)) {
      throw new Error('Binary not found after build');
    }
    console.log('✓ Release binary built\n');
  } catch (err) {
    console.error('✗ Build failed. Aborting release.');
    process.exit(1);
  }

  // Run pre-publish validation
  console.log('5. Running pre-publish validation...');
  try {
    exec('node scripts/prepublish.js');
    console.log('✓ Pre-publish validation passed\n');
  } catch (err) {
    console.error('✗ Pre-publish validation failed. Aborting release.');
    process.exit(1);
  }

  // Generate changelog
  console.log('6. Generating changelog...');
  try {
    exec(`node scripts/generate-changelog.js ${targetVersion}`);
    console.log('✓ Changelog generated\n');
  } catch (err) {
    console.warn('⚠ Warning: Could not generate changelog:', err.message);
    console.warn('Continuing anyway...\n');
  }

  // Package npm bundle (dry-run)
  console.log('7. Verifying npm package...');
  try {
    exec('npm pack --dry-run');
    console.log('✓ Package contents verified\n');
  } catch (err) {
    console.error('✗ Package verification failed. Aborting release.');
    process.exit(1);
  }

  // Show summary
  console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
  console.log('Release Summary:');
  console.log(`  Version: ${currentVersion} → ${targetVersion}`);
  console.log(`  Git tag: v${targetVersion}`);
  console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n');

  // Prompt for confirmation
  const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout
  });

  rl.question('Create git commit and tag? (y/N): ', async (answer) => {
    rl.close();
    
    if (answer.toLowerCase() !== 'y') {
      console.log('\nRelease preparation complete, but no commit/tag created.');
      console.log('To create commit and tag manually:');
      console.log(`  git add package.json Cargo.toml src/main.rs`);
      console.log(`  git commit -m "chore: bump version to ${targetVersion}"`);
      console.log(`  git tag v${targetVersion}`);
      console.log(`  git push origin master --tags`);
      process.exit(0);
    }

    // Create git commit
    console.log('\n8. Creating git commit...');
    try {
      const filesToAdd = ['package.json', 'Cargo.toml', 'src/main.rs'];
      if (fs.existsSync(path.join(ROOT, 'CHANGELOG.md'))) {
        filesToAdd.push('CHANGELOG.md');
      }
      exec(`git add ${filesToAdd.join(' ')}`);
      exec(`git commit -m "chore: bump version to ${targetVersion}"`);
      console.log('✓ Commit created\n');
    } catch (err) {
      console.error('✗ Failed to create commit:', err.message);
      process.exit(1);
    }

    // Create git tag
    console.log('9. Creating git tag...');
    try {
      exec(`git tag v${targetVersion}`);
      console.log('✓ Tag created\n');
    } catch (err) {
      console.error('✗ Failed to create tag:', err.message);
      process.exit(1);
    }

    // Show next steps
    console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━');
    console.log('✅ Release preparation complete!\n');
    console.log('Next steps:');
    console.log(`  1. Push to GitHub: git push origin master --tags`);
    console.log(`  2. Create GitHub Release for v${targetVersion}`);
    console.log(`  3. GitHub Actions will automatically publish to npm`);
    console.log('\n  Or publish manually:');
    console.log(`    npm publish --access public`);
    console.log('━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n');
  });
}

main().catch((err) => {
  console.error('Fatal error:', err);
  process.exit(1);
});

