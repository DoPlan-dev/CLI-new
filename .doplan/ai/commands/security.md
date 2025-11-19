# Security

## Overview
Run comprehensive security scans on your project to detect vulnerabilities and security issues.

## Workflow
1. Detect project type and dependencies
2. Run security scans:
   - npm audit (for Node.js projects)
   - trufflehog (secrets scanning)
   - git-secrets (Git history scanning)
   - gosec (Go security scanner)
   - dive (Docker image analysis)
3. Display security report with:
   - Vulnerabilities found
   - Severity levels
   - Affected packages/files
   - Fix recommendations
4. Offer to auto-fix issues (if available)

## Scanners Used
- npm audit, trufflehog, git-secrets
- gosec, dive, and more
