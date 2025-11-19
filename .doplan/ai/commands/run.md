# Run

## Overview
Auto-detect and run the development server for your project. Supports multiple frameworks and package managers.

## Workflow
1. Detect project type (Next.js, React, Vue, Go, Python, etc.)
2. Detect package manager (npm, yarn, pnpm, go, python, etc.)
3. Run appropriate dev command:
   - Next.js/React: `npm run dev`
   - Go: `go run .`
   - Python: `python -m uvicorn main:app --reload`
   - etc.
4. Display server URL and status
5. Monitor for errors and restart if needed

## Supported Frameworks
- Next.js, React, Vue, Svelte
- Express, FastAPI, Flask
- Go applications
- Python applications
- And more (auto-detected)
