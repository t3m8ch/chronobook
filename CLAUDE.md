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

## Architecture

### Core Domain Model
- **Organizations**: Top-level entities that own branches and employees
- **Branches**: Physical locations with timezone and address information
- **Employees**: Users with roles (owner, manager, master)
- **Services**: Offerings provided by masters with duration and pricing
- **Customers**: Clients who book services
- **Bookings**: Appointments linking customers, services, masters, and time slots
- **Timetables & Schedules**: Master availability using recurring patterns with day-specific overrides

### Database Schema
- PostgreSQL with UUID primary keys
- JSONB fields for flexible schedule data storage
- Enum types for status tracking (booking_status, user_type, auth_method, notify_method)
- Authentication tables supporting SMS and Telegram verification

### Time Management
- All time slots use 15-minute increments for efficient background processing
- Schedule system uses recurrence cycles with day ordinals
- Day redefinitions allow overriding standard schedules for specific dates
- Timezone handling per branch location

## Testing Strategy

The codebase uses `yare` for parameterized testing. Key practices:

1. **Parameterized Tests with yare**:
   - For async tests: Use `#[parameterized(...)]` followed by `#[test_macro(tokio::test)]`
   - Avoid conditional logic in parameterized tests - split into separate functions instead

2. **Builder Pattern**: Use `bon` for creating test data builders with sensible defaults

3. **Test Organization**: Tests live in `#[cfg(test)]` modules within implementation files

## Key Dependencies

- **Runtime**: tokio (async runtime)
- **Web Framework**: Poem OpenAPI (when implemented)
- **Database**: sqlx with PostgreSQL
- **Time**: chrono for date/time handling
- **Validation**: garde for data validation
- **IDs**: uuid for unique identifiers
- **Builders**: bon for builder pattern
- **Testing**: yare for parameterized tests

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