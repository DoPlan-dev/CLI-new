--- Cursor Command: generate.md ---
# Generate

## Overview
Generate Product Requirements Document (PRD), project structure document, and API contracts based on the refined idea.

## Workflow
1. Read idea notes from `doplan/idea-notes.md`
2. Read state from `.doplan/state.json`
3. Generate `doplan/PRD.md` - Product Requirements Document
4. Generate `doplan/structure.md` - Project structure and architecture
5. Generate `doplan/contracts/api-spec.json` - API specification (OpenAPI/Swagger)
6. Generate `doplan/contracts/data-model.md` - Data models and schemas
7. Use templates from `doplan/templates/` directory

## Documents Created
- PRD.md - Complete product requirements
- structure.md - Project architecture
- api-spec.json - API contracts
- data-model.md - Data models

## Usage
Run `/generate` in your IDE to generate Phase 1 documents.

## Execution
This command is executed via:
```bash
doplan execute generate
```

--- End Command ---
