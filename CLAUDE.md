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
├── main.rs              # Application entry point with Axum server and OpenAPI setup
├── api/                 # API layer
│   ├── mod.rs          # API module definitions
│   └── v1/             # Version 1 endpoints
│       ├── mod.rs      # V1 module definitions
│       ├── auth.rs     # Authentication endpoints (phone/telegram)
│       ├── bookings.rs # Booking management endpoints
│       └── admin/      # Admin endpoints module
│           ├── mod.rs      # Admin module with dashboard endpoint
│           ├── branch.rs   # Branch CRUD operations
│           ├── employee.rs # Employee CRUD operations
│           ├── service.rs  # Service CRUD operations
│           └── timetable.rs # Timetable and schedule management
├── models/             # Data models and DTOs
│   ├── mod.rs          # Module definitions
│   ├── error.rs        # ApiError type definition
│   ├── validation.rs   # Validation utilities with garde
│   ├── auth/          # Auth request/response models
│   ├── booking/       # Booking request/response models
│   ├── branch/        # Branch request/response models
│   ├── dashboard/     # Dashboard response models
│   ├── employee/      # Employee request/response models
│   ├── master/        # Master response models
│   ├── organization/  # Organization response models
│   ├── service/       # Service request/response models
│   └── timetable/     # Timetable request/response models
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
- JSONB fields for flexible schedule data storage (`schedule_days.day_data`, `day_redefinitions.day_data`)
- Enum types: `booking_status` (confirmed, cancelled), `notify_method` (sms, telegram)
- Core tables: organizations, users, user_profiles, branches, employees, services, customers, bookings
- Schedule tables: timetables, schedule_days, day_redefinitions
- Authentication tables: phone_verify_codes, telegram_verify_hashes
- Composite primary keys in schedule tables (master_id + day_ordinal/date)
- Organization-scoped customer authentication (unique constraint on user_id + organization_id)

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
- `POST /login/phone` - Initiate phone login (with organization_name in path)
- `POST /verify/phone` - Verify phone code (with organization_name in path)
- `POST /login/telegram` - Initiate Telegram login (with organization_name in path)
- `POST /verify/telegram` - Verify Telegram auth
- `POST /refresh` - Refresh access token

### Bookings API (`/api/v1/bookings`)
- `GET /organizations/{organization_name}` - Get organization by name
- `GET /services` - Get available services (query params: organization_id, master_ids[], branch_ids[])
- `GET /masters` - Get masters (query params: organization_id, branch_ids[])
- `GET /masters/{master_id}` - Get master by ID
- `GET /branches` - Get branches (query params: organization_id, master_ids[])
- `GET /windows` - Get available time windows (query params: organization_id, master_id, service_id, date)
- `POST /bookings` - Create new booking

### Admin API (`/api/v1/admin`)

#### Dashboard
- `GET /dashboard/{organization_id}` - Get organization dashboard

#### Branches CRUD
- `GET /branches?organization_id={id}` - List all branches for an organization
- `GET /branches/{branch_id}` - Get specific branch details
- `POST /branches` - Create new branch
- `PUT /branches/{branch_id}` - Update branch (supports partial updates)
- `DELETE /branches/{branch_id}` - Delete branch

#### Employees CRUD
- `GET /employees?organization_id={id}` - List all employees for an organization
- `GET /employees/{employee_id}` - Get specific employee details
- `POST /employees` - Create new employee
- `PUT /employees/{employee_id}` - Update employee (supports partial updates)
- `DELETE /employees/{employee_id}` - Delete employee

#### Services CRUD
- `GET /services?organization_id={id}` - List all services for an organization
- `GET /services/{service_id}` - Get specific service details
- `POST /services` - Create new service
- `PUT /services/{service_id}` - Update service (supports partial updates)
- `DELETE /services/{service_id}` - Delete service

#### Timetables & Schedules
- `GET /timetables?organization_id={id}` - List all timetables for an organization
- `GET /timetable/{master_id}` - Get timetable with schedule days and redefinitions
- `POST /timetable` - Create new timetable
- `PUT /timetable/{master_id}` - Update timetable
- `DELETE /timetable/{master_id}` - Delete timetable
- `POST /timetable/redefinitions` - Create day redefinition
- `DELETE /timetable/redefinitions/{master_id}/{date}` - Delete day redefinition

## Key Dependencies

- **Runtime**: tokio 1.47 (async runtime)
- **Web Framework**: axum 0.8 (web framework) + utoipa 5.4 (OpenAPI generation)
- **OpenAPI UI**: utoipa-scalar 0.3 (Scalar UI for API documentation)
- **OpenAPI Router**: utoipa-axum 0.2 (OpenAPI-aware routing)
- **Database**: sqlx 0.8 with PostgreSQL (compile-time checked queries)
- **Time**: chrono 0.4 for date/time handling
- **Validation**: garde 0.22 for input validation
- **Error Handling**: anyhow 1.0 for error management
- **IDs**: uuid 1.18 for unique identifiers
- **Phone Validation**: phonenumber 0.3 for international phone number validation
- **Serialization**: serde 1.0 + serde_json for JSON handling
- **HTTP Middleware**: tower 0.5 + tower-http 0.6 (CORS, tracing)
- **Tracing**: tracing 0.1 + tracing-subscriber 0.3 for logging
- **Environment**: dotenv 0.15 for environment variables
- **Utilities**: derive_more 2.0 for custom derives

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
- **Host**: Binds to `0.0.0.0:3222` (configurable via SERVER_ADDR env var)
- **API Base Path**: `/api/v1`
- **Documentation**: Scalar UI at `/docs/scalar`
- **OpenAPI Spec**: Auto-generated and available at `/api/v1/openapi.json`
- **CORS**: Permissive CORS policy enabled
- **Tracing**: HTTP request tracing enabled

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
- **Strong Typing**: Separate request/response models for each endpoint
- **Error Consistency**: Unified `ApiError` structure across all endpoints
- **OpenAPI First**: Full specification with automatic documentation via utoipa
- **Validation Layer**: garde validators with custom ValidationExt trait
- **Router Architecture**: utoipa-axum OpenApiRouter for automatic OpenAPI generation
- **State Management**: Arc<AppState> for shared application state
- **Partial Updates**: Update endpoints use `Option<Option<T>>` pattern for nullable fields:
  - `None` (field omitted) → keep existing value
  - `Some(None)` (field explicitly null) → clear the value
  - `Some(Some(value))` → update to new value

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
