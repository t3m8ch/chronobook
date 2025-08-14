# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

ChronoBook is a booking management system built in Rust for beauty salons and service businesses. It provides appointment scheduling, customer management, and notification capabilities with support for multiple branches, masters, and services.

## Development Commands

### Build and Check
```bash
cargo build           # Build the project
cargo check          # Quick syntax/type check
cargo run            # Run the application
```

### Database Management
```bash
sqlx migrate run     # Run database migrations
sqlx migrate revert  # Revert last migration
sqlx migrate add <name>  # Create new migration
```

### Testing
```bash
cargo test           # Run all tests
cargo test <test_name>  # Run specific test
cargo test -- --nocapture  # Show println! output during tests
```

## Project Structure

### Directory Layout
```
/src/
├── main.rs              # Application entry point with OpenAPI service setup
├── api/                 # API layer
│   ├── v1/             # Version 1 endpoints
│   │   ├── auth.rs     # Customer and employee authentication
│   │   └── bookings.rs # Booking management endpoints
│   ├── error.rs        # Centralized error handling (ApiError)
│   └── validators.rs   # Custom validators (phone validation)
/migrations/            # SQLx database migrations
/docs/                  # Documentation and specifications
```

## Architecture

### Core Domain Model
- **Organizations**: Top-level entities with unique names that own branches and employees
- **Branches**: Physical locations with timezone and address information
- **Employees**: Users with role flags (is_owner, is_manager, is_master)
- **Services**: Offerings provided by masters with duration (optional) and pricing
- **Customers**: Clients who book services, validated per organization
- **Bookings**: Appointments linking customers, services, masters, branches, and time slots
- **Timetables & Schedules**: Master availability using recurring patterns with day-specific overrides

### Database Schema
- PostgreSQL with UUID primary keys for all entities
- JSONB fields for flexible schedule data storage (`schedule_days.day_data`)
- Enum types for status tracking (booking_status, user_type, auth_method, notify_method)
- Authentication tables (`auth`, `auth_phone_verification`, `auth_telegram_verification`)
- Composite primary keys in schedule tables (master_id + day_ordinal/date)
- Organization-scoped customer authentication

### Time Management
- All time slots use 15-minute increments for efficient background processing
- Schedule system uses recurrence cycles with day ordinals (timetables table)
- Day redefinitions allow overriding standard schedules for specific dates
- Timezone handling per branch location
- Flexible service duration (some services may have undefined duration)

## Testing Strategy

The codebase uses `yare` for parameterized testing. Key practices:

1. **Parameterized Tests with yare**:
   - For async tests: Use `#[parameterized(...)]` followed by `#[test_macro(tokio::test)]`
   - Avoid conditional logic in parameterized tests - split into separate functions instead

2. **Builder Pattern**: Use `bon` for creating test data builders with sensible defaults

3. **Test Organization**: Tests live in `#[cfg(test)]` modules within implementation files

## API Endpoints

### Authentication API (`/api/v1/auth`)

**Customer Authentication:**
- `POST /customer/{organization_name}/login/phone` - Initiate phone login
- `POST /customer/{organization_name}/verify/phone` - Verify phone code
- `POST /customer/{organization_name}/login/telegram` - Initiate Telegram login
- `POST /customer/{organization_name}/verify/telegram` - Verify Telegram auth
- `POST /customer/{organization_name}/refresh` - Refresh access token

**Employee Authentication:**
- `POST /employee/login/phone` - Employee phone login
- `POST /employee/verify/phone` - Employee phone verification
- `POST /employee/login/telegram` - Employee Telegram login
- `POST /employee/verify/telegram` - Employee Telegram verification
- `POST /employee/refresh` - Employee token refresh

### Bookings API (`/api/v1/bookings`)
- `GET /services?organization_id=<uuid>` - Get available services for organization
- `GET /masters?organization_id=<uuid>&branches=<uuid[]>` - Get masters by branches
- `GET /branches?organization_id=<uuid>&masters=<uuid[]>` - Get branches by masters

## Key Dependencies

- **Runtime**: tokio (async runtime)
- **Web Framework**: Poem + Poem OpenAPI (REST API with automatic OpenAPI docs)
- **Database**: sqlx with PostgreSQL (compile-time checked queries)
- **Time**: chrono for date/time handling
- **Validation**: Custom validators using derive_more and phonenumber crate
- **IDs**: uuid for unique identifiers
- **Builders**: bon for builder pattern
- **Testing**: yare for parameterized tests
- **Phone Validation**: phonenumber crate for international phone number validation
- **Serialization**: serde for JSON handling
- **Tracing**: Built-in instrumentation for API endpoints

## Business Logic Notes

### Booking Process
1. Customer selects branch → master → service
2. Available time slots shown based on service duration
3. Confirmation requires phone/telegram verification
4. Notifications sent at configurable intervals with blackout periods

### User Roles
- **Root**: System-wide admin access
- **Owner**: Organization management, can create branches and manage masters
- **Master**: Service provider with schedule management
- **Manager**: Branch-level administration

### Notification System
- Supports SMS and Telegram bots
- Configurable timing with quiet hours
- Templates are customizable per organization

## Application Configuration

### Server Setup
- **Host**: Binds to `0.0.0.0:3222`
- **API Base Path**: `/api/v1`
- **Documentation**: Stoplight Elements UI at `/docs/stoplight`
- **OpenAPI Spec**: Auto-generated and available at `/api/v1/openapi.json`

### Development Environment
```bash
cargo run              # Starts server on port 3222
curl http://localhost:3222/api/v1/openapi.json  # Get OpenAPI spec
```

## Architectural Patterns

### Authentication Architecture
- **Dual User Types**: Separate flows for customers vs employees
- **Multi-Method Auth**: SMS and Telegram authentication support
- **Organization Scoping**: Customer auth scoped to specific organizations
- **Token Management**: Refresh token mechanism for both user types

### API Design Patterns
- **Strong Typing**: Custom response enums for each endpoint
- **Error Consistency**: Unified `ApiError` structure across all endpoints
- **OpenAPI First**: Full specification with automatic documentation
- **Validation Layer**: Custom validators integrated with Poem framework

### Data Modeling Decisions
- **Multi-Tenancy**: Organization-based data isolation
- **Flexible Scheduling**: JSONB for complex schedule data
- **Role-Based Access**: Boolean flags for employee roles
- **Service Flexibility**: Optional duration for services

## Mermaid ER Diagram Notes

When creating Mermaid ER diagrams, avoid these common mistakes:
1. **DO NOT use PK-FK or PK_FK notation** - Mermaid doesn't support compound annotations like "PK-FK". Use separate PK and FK annotations.
2. **Composite primary keys** - When a table has composite primary keys, list each field with PK annotation separately
3. **Syntax rules**:
   - After field type, only PK, FK, or UK annotations are allowed
   - No hyphens or underscores in annotations
   - Each field should be on its own line with proper spacing
