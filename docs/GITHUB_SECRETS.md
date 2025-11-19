# GitHub Secrets Configuration

This document explains how to configure GitHub secrets for automated npm publishing.

## Required Secrets

### NPM_TOKEN

An npm authentication token with publish permissions for the `@doplan-dev/cli` package.

#### Creating the Token

1. **Log in to npm:**
   ```bash
   npm login
   ```

2. **Create an Automation Token:**
   - Go to https://www.npmjs.com/settings/YOUR_USERNAME/tokens
   - Click "Generate New Token"
   - Select token type: **"Automation"** (for CI/CD)
   - Enter a descriptive name: `DoPlan CLI - GitHub Actions`
   - Set expiration (recommended: 90 days or 1 year)
   - Click "Generate Token"

3. **Copy the token:**
   - ⚠️ **Important**: Copy the token immediately, as it won't be shown again
   - The token will look like: `npm_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx`

#### Adding to GitHub

1. **Navigate to your repository:**
   - Go to https://github.com/DoPlan-dev/CLI-new

2. **Open Settings:**
   - Click on the repository
   - Click on "Settings" tab

3. **Go to Secrets:**
   - In the left sidebar, click "Secrets and variables" → "Actions"

4. **Add New Secret:**
   - Click "New repository secret"
   - Name: `NPM_TOKEN`
   - Value: Paste your npm token
   - Click "Add secret"

#### Verifying the Secret

The secret is automatically used by the `.github/workflows/publish-npm.yml` workflow when:
- A GitHub Release is created
- The workflow is manually triggered

You can verify it's working by checking the workflow logs after a release is created.

### GITHUB_TOKEN (Optional)

The `GITHUB_TOKEN` is automatically provided by GitHub Actions and doesn't need to be configured manually. It's used for:
- Creating GitHub Releases
- Uploading release assets
- Creating release artifacts

This token is automatically available in all GitHub Actions workflows.

## Workflow Integration

### Automated Publishing

When you create a GitHub Release:

1. The `build-binaries.yml` workflow builds binaries for all platforms
2. The `publish-npm.yml` workflow publishes to npm
3. Both workflows use `NPM_TOKEN` from secrets

### Manual Testing

You can test the setup by:

1. **Triggering the workflow manually:**
   - Go to Actions tab
   - Select "Publish to npm"
   - Click "Run workflow"
   - Enter a version (e.g., `1.0.0`)

2. **Checking the logs:**
   - Watch the workflow execution
   - Verify "Publish to npm" step succeeds
   - Check npm registry: https://www.npmjs.com/package/@doplan-dev/cli

## Token Security Best Practices

1. **Use Automation Tokens:**
   - Automation tokens are designed for CI/CD
   - They have limited permissions (only publish)

2. **Set Expiration:**
   - Don't create tokens with no expiration
   - Recommended: 90 days to 1 year

3. **Rotate Regularly:**
   - Update tokens periodically
   - Remove old tokens when creating new ones

4. **Limit Scope:**
   - Use organization-level tokens if publishing under an org
   - Restrict to specific packages if possible

5. **Monitor Usage:**
   - Check npm token usage logs regularly
   - Review GitHub Actions logs for unauthorized access

## Troubleshooting

### "NPM_TOKEN secret not found"

- Verify the secret name is exactly `NPM_TOKEN` (case-sensitive)
- Check that you're adding it to the correct repository
- Ensure you have admin access to the repository

### "You do not have permission to publish"

- Verify the npm token has publish permissions
- Check that you're logged in to the correct npm account
- Ensure the package name matches: `@doplan-dev/cli`
- Verify organization access if using a scoped package

### "Invalid authentication token"

- The token may have expired
- Generate a new token and update the secret
- Check token expiration date in npm settings

### Workflow Not Triggering

- Verify the workflow file exists: `.github/workflows/publish-npm.yml`
- Check workflow trigger conditions (release creation)
- Ensure you're creating a proper GitHub Release (not just a tag)

## Organization Setup

If publishing under `@doplan-dev` organization:

1. **Create Organization:**
   - Go to https://www.npmjs.com/org/create
   - Create organization: `doplan-dev`

2. **Add Package:**
   - Create package under organization
   - Grant publish permissions to team members

3. **Organization Token:**
   - Create organization-level automation token
   - Grants access to all packages in the organization

## References

- [npm Token Documentation](https://docs.npmjs.com/about-access-tokens)
- [GitHub Secrets Documentation](https://docs.github.com/en/actions/security-guides/encrypted-secrets)
- [GitHub Actions Authentication](https://docs.github.com/en/actions/publishing-packages/publishing-nodejs-packages#authenticating-to-package-registries)

