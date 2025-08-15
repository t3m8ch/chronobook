-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE organizations (
    id UUID PRIMARY KEY,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    name VARCHAR(255) UNIQUE NOT NULL,
    display_name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL
);

CREATE TABLE users (
    id UUID PRIMARY KEY,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    phone VARCHAR(255) UNIQUE,
    telegram_id BIGINT UNIQUE,
    phone_verified_at TIMESTAMP,
    telegram_verified_at TIMESTAMP
);

CREATE TABLE user_profiles (
    user_id UUID PRIMARY KEY,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    first_name VARCHAR(255) NOT NULL,
    last_name VARCHAR(255) NOT NULL,
    patronymic VARCHAR(255),

    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE TABLE branches (
    id UUID PRIMARY KEY,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    display_name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    timezone VARCHAR(255) NOT NULL,
    street VARCHAR(255) NOT NULL,
    house_number VARCHAR(255) NOT NULL,
    apartment_number VARCHAR(255),
    city VARCHAR(255) NOT NULL,
    region VARCHAR(255) NOT NULL,
    country VARCHAR(255) NOT NULL,
    address_info VARCHAR(255),

    organization_id UUID NOT NULL,
    FOREIGN KEY (organization_id) REFERENCES organizations(id)
);

CREATE TABLE employees (
    id UUID PRIMARY KEY,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    contact_phone VARCHAR(255),
    contact_email VARCHAR(255),
    contact_telegram VARCHAR(255),
    is_owner BOOLEAN NOT NULL,
    is_manager BOOLEAN NOT NULL,
    is_master BOOLEAN NOT NULL,

    organization_id UUID NOT NULL,
    manager_branch_id UUID,
    user_id UUID NOT NULL,
    FOREIGN KEY (organization_id) REFERENCES organizations(id),
    FOREIGN KEY (manager_branch_id) REFERENCES branches(id),
    FOREIGN KEY (user_id) REFERENCES users(id),
    UNIQUE (user_id, organization_id)
);

CREATE TABLE services (
    id UUID PRIMARY KEY,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    display_name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    duration_minutes INTEGER,
    price NUMERIC(19, 2) NOT NULL,

    master_id UUID,
    FOREIGN KEY (master_id) REFERENCES employees(id)
);

CREATE TABLE timetables (
    master_id UUID PRIMARY KEY,
    recurrence_cycle_start DATE NOT NULL,
    FOREIGN KEY (master_id) REFERENCES employees(id)
);

CREATE TABLE schedule_days (
    master_id UUID NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    day_ordinal INTEGER NOT NULL,
    day_data JSONB NOT NULL,

    FOREIGN KEY (master_id) REFERENCES employees(id),
    PRIMARY KEY (master_id, day_ordinal)
);

CREATE TABLE day_redefinitions (
    master_id UUID NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    date DATE NOT NULL,
    day_data JSONB NOT NULL,

    FOREIGN KEY (master_id) REFERENCES employees(id),
    PRIMARY KEY (master_id, date)
);

CREATE TABLE customers (
    id UUID PRIMARY KEY,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    organization_id UUID NOT NULL,

    user_id UUID NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (organization_id) REFERENCES organizations(id),
    UNIQUE (user_id, organization_id)
);

CREATE TYPE notify_method AS ENUM ('sms', 'telegram');

CREATE TABLE bookings (
    id UUID PRIMARY KEY,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    customer_id UUID NOT NULL,
    service_id UUID NOT NULL,
    master_id UUID NOT NULL,
    branch_id UUID NOT NULL,
    started_at TIMESTAMP NOT NULL,
    ended_at TIMESTAMP NOT NULL,
    notify_methods notify_method[] NOT NULL,
    FOREIGN KEY (customer_id) REFERENCES customers(id),
    FOREIGN KEY (service_id) REFERENCES services(id),
    FOREIGN KEY (master_id) REFERENCES employees(id),
    FOREIGN KEY (branch_id) REFERENCES branches(id)
);

CREATE TYPE booking_status AS ENUM ('confirmed', 'cancelled');

CREATE TABLE booking_status_change (
    id UUID PRIMARY KEY,
    created_at TIMESTAMP NOT NULL,
    who_id UUID NOT NULL,
    status booking_status NOT NULL,
    reason TEXT NOT NULL,
    FOREIGN KEY (who_id) REFERENCES employees(id)
);

CREATE TABLE phone_verify_codes (
    id UUID PRIMARY KEY,
    created_at TIMESTAMP NOT NULL,
    code INTEGER NOT NULL,
    expire_at TIMESTAMP NOT NULL,
    used BOOLEAN NOT NULL,

    user_id UUID NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);

CREATE TABLE telegram_verify_hashes (
    id UUID PRIMARY KEY,
    created_at TIMESTAMP NOT NULL,
    hash BYTEA NOT NULL,
    expire_at TIMESTAMP NOT NULL,
    used BOOLEAN NOT NULL,

    user_id UUID NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id)
);
