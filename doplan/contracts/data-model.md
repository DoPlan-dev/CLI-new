# Data Model & Schemas

**Project:** Untitled Project

**Date:** 2025-11-19

---

## Overview

This document defines the data models, schemas, and database structure for the project.

---

## Core Entities

### User

Represents a user in the system.

```typescript
interface User {
  id: string;              // Unique identifier
  email: string;            // User email
  name: string;             // User name
  createdAt: Date;          // Account creation date
  updatedAt: Date;          // Last update date
  status: 'active' | 'inactive' | 'suspended';
}
```

## Database Schema

### Tables

#### users
| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | UUID | PRIMARY KEY | Unique identifier |
| email | VARCHAR(255) | UNIQUE, NOT NULL | User email |
| name | VARCHAR(255) | NOT NULL | User name |
| created_at | TIMESTAMP | NOT NULL | Creation timestamp |
| updated_at | TIMESTAMP | NOT NULL | Update timestamp |
| status | VARCHAR(20) | NOT NULL | User status |

## Relationships

### Entity Relationships

- **User** has many **Items** (one-to-many)
- **User** belongs to **Organization** (many-to-one)
- **Items** can have many **Tags** (many-to-many)

## Data Validation Rules

### User Entity
- Email must be valid format
- Email must be unique
- Name must be between 2-100 characters
- Status must be one of: active, inactive, suspended

## Database Indexes

### Recommended Indexes
- `users.email` - For fast email lookups
- `users.created_at` - For sorting by creation date
- `users.status` - For filtering by status

## Data Migration Strategy

### Version Control
- Use migration files for schema changes
- Version all schema changes
- Test migrations on staging before production

### Migration Best Practices
- Always backup before migrations
- Test rollback procedures
- Monitor migration performance
- Document breaking changes

## API Data Contracts

### Request/Response Formats

All API requests and responses should follow these formats:

#### Standard Response
```json
{
  "success": true,
  "data": {},
  "message": "Operation successful"
}
```

#### Error Response
```json
{
  "success": false,
  "error": {
    "code": "ERROR_CODE",
    "message": "Error description"
  }
}
```

