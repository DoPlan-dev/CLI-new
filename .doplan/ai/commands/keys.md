# Keys

## Overview
Manage API keys and service configuration. Detect required keys, validate them, and manage service setup guides.

## Workflow
1. Detect required services from dependencies
2. Check .env file for configured keys
3. Show RAKD status:
   - Configured keys
   - Pending keys
   - Optional keys
4. Allow actions:
   - Validate all keys
   - Check for missing keys
   - Sync .env.example
   - Launch setup wizard for a service
   - Test API connections
5. Generate/update RAKD.md
6. Generate SOPS guides for services

## Features
- Auto-detect required services
- Validate API keys
- Generate service setup guides
- Manage .env files
