# Project Context

**Project:** DoPlan CLI

**Date:** 2025-11-19

---

## Technology Stack

- Frontend: React/Next.js
- Backend: Rust/Axum
- Database: PostgreSQL
- Deployment: Docker

## Project Structure

```
project-root/
├── src/                    # Source code
├── tests/                  # Test files
├── docs/                   # Documentation
├── doplan/                 # DoPlan project files
└── README.md               # Project README
```

## Documentation

### DoPlan Documents
- [PRD](./doplan/PRD.md) - Product Requirements Document
- [Structure](./doplan/structure.md) - Project structure and architecture
- [DPR](./doplan/design/DPR.md) - Design Preferences & Requirements
- [RAKD](./doplan/RAKD.md) - Required API Keys Document
- [SOPS](./doplan/SOPS/) - Service Operating Procedures

### Contracts
- [API Specification](./doplan/contracts/api-spec.json) - OpenAPI specification
- [Data Model](./doplan/contracts/data-model.md) - Data models and schemas

## Development Workflow

1. **Planning**: Review plans in `doplan/plan/`
2. **Design**: Follow DPR guidelines
3. **Implementation**: Write code following structure guidelines
4. **Testing**: Write tests alongside implementation
5. **Review**: Code review before merging
6. **Deployment**: Follow deployment procedures

## Key Resources

### Design
- [Design Tokens](./doplan/design/design-tokens.json)
- [Design Rules](./.doplan/ai/rules/design_rules.mdc)

### Templates
- [Plan Template](./doplan/templates/plan-template.md)
- [Design Template](./doplan/templates/design-template.md)
- [Tasks Template](./doplan/templates/tasks-template.md)

