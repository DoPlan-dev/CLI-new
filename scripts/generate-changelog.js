#!/usr/bin/env node

/**
 * Automated Changelog Generation Script
 * 
 * Generates CHANGELOG.md from git commits and tags
 * 
 * Usage:
 *   node scripts/generate-changelog.js [version] [output-file]
 *   If version is provided, generates changelog for that version
 *   If not provided, generates full changelog
 */

const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

const ROOT = path.join(__dirname, '..');
const CHANGELOG_PATH = path.join(ROOT, 'CHANGELOG.md');

// Semantic commit types and their labels
const COMMIT_TYPES = {
  'feat': '✨ Features',
  'fix': '🐛 Bug Fixes',
  'docs': '📚 Documentation',
  'style': '💎 Styles',
  'refactor': '♻️ Code Refactoring',
  'perf': '⚡ Performance Improvements',
  'test': '✅ Tests',
  'build': '🔨 Build System',
  'ci': '👷 CI/CD',
  'chore': '🔧 Chores',
  'revert': '⏪ Reverts'
};

function exec(command, options = {}) {
  try {
    return execSync(command, {
      cwd: ROOT,
      encoding: 'utf8',
      stdio: options.silent ? 'pipe' : 'inherit',
      ...options
    }).trim();
  } catch (err) {
    if (options.silent) {
      return '';
    }
    throw err;
  }
}

function getTags() {
  try {
    const tags = exec('git tag --sort=-v:refname', { silent: true });
    return tags.split('\n').filter(tag => tag.trim()).map(tag => tag.trim());
  } catch (err) {
    return [];
  }
}

function getCommitsSinceTag(tag) {
  try {
    if (tag) {
      return exec(`git log ${tag}..HEAD --pretty=format:"%h|%s|%b"`, { silent: true });
    } else {
      return exec('git log --pretty=format:"%h|%s|%b"', { silent: true });
    }
  } catch (err) {
    return '';
  }
}

function getCommitsBetweenTags(fromTag, toTag) {
  try {
    return exec(`git log ${fromTag}..${toTag} --pretty=format:"%h|%s|%b"`, { silent: true });
  } catch (err) {
    return '';
  }
}

function parseCommit(commitLine) {
  const [hash, ...rest] = commitLine.split('|');
  const subject = rest[0] || '';
  const body = rest.slice(1).join('|').trim();
  
  // Parse conventional commit format: type(scope): description
  const match = subject.match(/^(\w+)(?:\(([^)]+)\))?: (.+)$/);
  
  if (match) {
    const [, type, scope, description] = match;
    return {
      hash: hash.trim(),
      type: type.toLowerCase(),
      scope: scope || '',
      description: description.trim(),
      body: body,
      subject: subject.trim()
    };
  }
  
  // Fallback for non-conventional commits
  return {
    hash: hash.trim(),
    type: 'chore',
    scope: '',
    description: subject.trim(),
    body: body,
    subject: subject.trim()
  };
}

function categorizeCommits(commits) {
  const categorized = {};
  
  for (const commit of commits) {
    const type = commit.type || 'chore';
    if (!categorized[type]) {
      categorized[type] = [];
    }
    categorized[type].push(commit);
  }
  
  return categorized;
}

function formatCommits(categorized) {
  const sections = [];
  
  // Order by COMMIT_TYPES
  for (const [type, label] of Object.entries(COMMIT_TYPES)) {
    if (categorized[type] && categorized[type].length > 0) {
      sections.push(`### ${label}\n`);
      
      for (const commit of categorized[type]) {
        const scope = commit.scope ? `**${commit.scope}**: ` : '';
        sections.push(`- ${scope}${commit.description}`);
        if (commit.body) {
          sections.push(`  ${commit.body.split('\n').join('\n  ')}`);
        }
      }
      
      sections.push('');
    }
  }
  
  // Handle uncategorized commits
  const uncategorized = Object.entries(categorized).filter(([type]) => !COMMIT_TYPES[type]);
  if (uncategorized.length > 0) {
    sections.push('### Other Changes\n');
    for (const [, commits] of uncategorized) {
      for (const commit of commits) {
        sections.push(`- ${commit.description}`);
      }
    }
    sections.push('');
  }
  
  return sections.join('\n');
}

function generateVersionChangelog(version, fromTag, toTag) {
  const commitsText = toTag ? getCommitsBetweenTags(fromTag, toTag) : getCommitsSinceTag(fromTag);
  
  if (!commitsText.trim()) {
    return `## ${version}\n\nNo changes recorded.\n\n`;
  }
  
  const commits = commitsText.split('\n')
    .filter(line => line.trim())
    .map(parseCommit);
  
  const categorized = categorizeCommits(commits);
  const formatted = formatCommits(categorized);
  
  const date = new Date().toISOString().split('T')[0];
  
  return `## [${version}] - ${date}\n\n${formatted}`;
}

function generateFullChangelog() {
  const tags = getTags();
  const sections = ['# Changelog\n', 'All notable changes to this project will be documented in this file.\n', ''];
  
  if (tags.length === 0) {
    // No tags, generate from all commits
    sections.push('## Unreleased\n');
    const commitsText = getCommitsSinceTag(null);
    if (commitsText.trim()) {
      const commits = commitsText.split('\n')
        .filter(line => line.trim())
        .map(parseCommit);
      const categorized = categorizeCommits(commits);
      sections.push(formatCommits(categorized));
    } else {
      sections.push('No changes recorded.\n');
    }
  } else {
    // Generate for unreleased commits
    sections.push('## Unreleased\n');
    const unreleasedCommits = getCommitsSinceTag(tags[0]);
    if (unreleasedCommits.trim()) {
      const commits = unreleasedCommits.split('\n')
        .filter(line => line.trim())
        .map(parseCommit);
      const categorized = categorizeCommits(commits);
      sections.push(formatCommits(categorized));
      sections.push('\n');
    } else {
      sections.push('No changes recorded.\n\n');
    }
    
    // Generate for each tag
    for (let i = 0; i < tags.length; i++) {
      const tag = tags[i];
      const version = tag.replace(/^v/, '');
      const fromTag = i < tags.length - 1 ? tags[i + 1] : null;
      
      sections.push(generateVersionChangelog(version, fromTag, tag));
      sections.push('\n');
    }
  }
  
  return sections.join('\n');
}

function main() {
  const args = process.argv.slice(2);
  
  let content;
  let outputPath = CHANGELOG_PATH;
  
  if (args.length > 1) {
    outputPath = path.resolve(args[1]);
  }
  
  if (args.length > 0) {
    // Generate changelog for specific version
    const version = args[0];
    const tags = getTags();
    const versionTag = version.startsWith('v') ? version : `v${version}`;
    
    const tagIndex = tags.findIndex(tag => tag === versionTag || tag === version);
    if (tagIndex === -1) {
      console.error(`Error: Tag ${versionTag} not found`);
      process.exit(1);
    }
    
    const fromTag = tagIndex < tags.length - 1 ? tags[tagIndex + 1] : null;
    content = generateVersionChangelog(version.replace(/^v/, ''), fromTag, tags[tagIndex]);
  } else {
    // Generate full changelog
    content = generateFullChangelog();
  }
  
  // Write to file
  fs.writeFileSync(outputPath, content, 'utf8');
  console.log(`✓ Changelog generated: ${outputPath}`);
}

main();

