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
├── config.rs            # Application configuration management
├── api/                 # API layer
│   ├── mod.rs          # API module definitions
│   └── v1/             # Version 1 endpoints
│       ├── mod.rs      # V1 module definitions
│       ├── auth.rs     # Authentication endpoints (phone/telegram)
│       ├── bookings.rs # Booking management endpoints
│       └── admin/      # Admin endpoints module
│           ├── mod.rs          # Admin module with dashboard endpoint
│           ├── branch.rs       # Branch CRUD operations
│           ├── employee.rs     # Employee CRUD operations
│           ├── notification.rs # Notification settings and templates management
│           ├── service.rs      # Service CRUD operations
│           └── timetable.rs    # Timetable and schedule management
├── models/             # Data models and DTOs
│   ├── mod.rs          # Module definitions
│   ├── error.rs        # ApiError type definition
│   ├── validation.rs   # Validation utilities with garde
│   ├── auth/           # Auth request/response models
│   │   ├── mod.rs      # Auth module definitions
│   │   ├── request.rs  # Auth request DTOs
│   │   ├── response.rs # Auth response DTOs
│   │   └── db.rs       # Auth database models for SQLx
│   ├── booking/        # Booking request/response models
│   ├── branch/         # Branch request/response models
│   ├── dashboard/      # Dashboard response models
│   ├── employee/       # Employee request/response models
│   ├── master/         # Master response models
│   ├── notification/   # Notification request/response models
│   ├── organization/   # Organization response models
│   ├── service/        # Service request/response models
│   └── timetable/      # Timetable request/response models
├── repositories/       # Data access layer with traits
│   ├── mod.rs          # Repository module definitions
│   └── auth.rs         # Auth repository implementation
├── services/           # Business logic layer
│   ├── mod.rs          # Service module definitions
│   ├── auth.rs         # Auth service implementation
│   ├── errors.rs       # Service-specific error types (using thiserror)
│   ├── jwt.rs          # JWT utilities for token management
│   └── providers.rs    # External service providers (SMS, Telegram)
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

4. **Mocking**: Use `mockall` for mocking external dependencies

## Key Dependencies

- **Runtime**: tokio 1.47 (async runtime)
- **Web Framework**: axum 0.8 (web framework) + utoipa 5.4 (OpenAPI generation)
- **OpenAPI UI**: utoipa-scalar 0.3 (Scalar UI for API documentation)
- **OpenAPI Router**: utoipa-axum 0.2 (OpenAPI-aware routing)
- **Database**: sqlx 0.8 with PostgreSQL (compile-time checked queries)
- **Time**: chrono 0.4 for date/time handling, time 0.3 for cookie timestamps
- **Validation**: garde 0.22 for input validation
- **Error Handling**: anyhow 1.0 for error management, thiserror 2.0 for service errors
- **IDs**: uuid 1.18 for unique identifiers
- **Phone Validation**: phonenumber 0.3 for international phone number validation
- **Serialization**: serde 1.0 + serde_json for JSON handling
- **HTTP Middleware**: tower 0.5 + tower-http 0.6 (CORS, tracing)
- **HTTP Extras**: axum-extra 0.10 for cookie handling and extractors
- **Tracing**: tracing 0.1 + tracing-subscriber 0.3 for logging
- **Environment**: dotenv 0.15 for environment variables
- **Authentication**: jsonwebtoken 9.3 for JWT tokens
- **Cryptography**: sha2 0.10 for hashing, hex 0.4 for encoding
- **Random**: rand 0.8 for generating secure tokens
- **Testing**: mockall 0.13 for mocking in tests
- **Utilities**: derive_more 2.0 for custom derives, async-trait 0.1 for async traits

### Missing Test Dependencies (to be added)
```toml
[dev-dependencies]
yare = "3.0"      # For parameterized testing
bon = "4.0"       # For test data builders
```

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
- Supports SMS and Telegram bots for customer communication
- Configurable timing with quiet hours and blackout periods
- Customizable templates per organization for different notification types
- Bulk notification capabilities for marketing and announcements
- Scheduled notifications for booking reminders and follow-ups

## Application Configuration

### Configuration Management
The application uses a centralized `Config` struct in `src/config.rs` to manage all configuration settings:

- **Config Structure**: All configuration is loaded through `Config::from_env()` in main.rs
- **Environment Variables**: Configuration read from environment variables with sensible defaults
- **Validation**: Comprehensive validation with detailed error messages for missing/invalid values
- **Centralized**: All objects are configured in main.rs and passed to other components

### Environment Variables
- `SERVER_ADDR` - Server binding address (default: `0.0.0.0:3222`)
- `JWT_ACCESS_SECRET` - Secret for signing access tokens (required)
- `JWT_REFRESH_SECRET` - Secret for signing refresh tokens (required)  
- `JWT_ACCESS_EXPIRATION_MINUTES` - Access token lifetime in minutes (default: 15)
- `JWT_REFRESH_EXPIRATION_DAYS` - Refresh token lifetime in days (default: 7)
- `DATABASE_URL` - PostgreSQL connection string (required)

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
- **Token Management**: Access/refresh token mechanism for both user types
- **Secure Cookies**: Refresh tokens stored in HTTP-only cookies for security

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

### Layered Architecture
- **API Layer**: Axum handlers that depend on services via traits
- **Service Layer**: Business logic implementation with trait-based interfaces
- **Repository Layer**: Data access layer with trait-based interfaces
- **Dependency Injection**: All layers use `Arc<dyn Trait>` for testability
- **Error Management**: Service-specific errors using `thiserror`
- **Role-Based Security**: Axum extractors for endpoint protection

### Data Modeling Decisions
- **Multi-Tenancy**: Organization-based data isolation
- **Flexible Scheduling**: JSONB for complex schedule data
- **Role-Based Access**: Boolean flags for employee roles with extractor validation
- **Service Flexibility**: Optional duration for services
- **Database Models**: Separate `db.rs` modules for SQLx-specific models

## Mermaid ER Diagram Notes

When creating Mermaid ER diagrams, avoid these common mistakes:
1. **DO NOT use PK-FK or PK_FK notation** - Mermaid doesn't support compound annotations like "PK-FK". Use separate PK and FK annotations.
2. **Composite primary keys** - When a table has composite primary keys, list each field with PK annotation separately
3. **Syntax rules**:
   - After field type, only PK, FK, or UK annotations are allowed
   - No hyphens or underscores in annotations
   - Each field should be on its own line with proper spacing

## Implementation Guidelines

### Current Implementation Status
The project is actively being developed with the following layers implemented:

**✅ Completed:**
- Basic project structure with layered architecture
- Authentication service and repository layers
- JWT token utilities
- Service error handling with thiserror
- Database models in separate `db.rs` modules
- HTTP-only cookie support for refresh tokens
- Centralized configuration management with Config struct

**🔄 In Progress:**
- Full CRUD operations for all entities
- Role-based access control extractors
- Complete test coverage
- External service providers (SMS, Telegram)

**📋 TODO:**
- Add missing test dependencies (yare, bon)
- Complete repository implementations
- Add comprehensive integration tests

### Backend Implementation Guidelines

1. **Layered Architecture**
   - Create separate repository and service layers
   - Use trait-based interfaces for all layers
   - Wrap traits in `Arc<dyn ...>` for dependency injection

2. **Data Transfer Objects**
   - Reuse API DTOs in services (avoid service-specific DTOs for now)
   - Create database models in `models/{entity}/db.rs` modules
   - Keep request/response models separate from database models

3. **Error Handling**
   - Use `thiserror` for service-specific custom errors
   - Maintain consistent `ApiError` structure for API responses
   - Propagate errors properly through layers

4. **Security**
   - Store refresh tokens ONLY in HTTP-only cookies
   - Never return refresh tokens in response bodies
   - Use Axum extractors for role-based endpoint protection
   - Implement proper CORS and security headers

5. **Testing Strategy**
   - Write comprehensive unit tests for all layers
   - Use `mockall` for mocking dependencies in tests
   - Apply `yare` for parameterized testing
   - Use `bon` for test data builders with sensible defaults

6. **External Services**
   - Define traits for SMS and Telegram providers
   - Use mocks for external services in tests
   - Keep provider implementations separate and swappable

7. **Code Organization**
   - Follow the established directory structure
   - Use async-trait for all async trait definitions
   - Apply proper module visibility and encapsulation

8. **Configuration Management**
   - Use the centralized `Config` struct for all configuration
   - Load configuration only in main.rs using `Config::from_env()`
   - Pass configuration values to components, never read environment variables directly in services/repositories
   - Components should receive configuration through constructors, not environment access
