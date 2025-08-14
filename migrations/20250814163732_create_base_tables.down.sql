-- Add down migration script here
DROP TABLE IF EXISTS telegram_verify_hashes;
DROP TABLE IF EXISTS phone_verify_codes;

DROP TABLE IF EXISTS booking_status_change;
DROP TABLE IF EXISTS bookings;

DROP TABLE IF EXISTS customers;

DROP TABLE IF EXISTS day_redefinitions;
DROP TABLE IF EXISTS schedule_days;
DROP TABLE IF EXISTS timetables;
DROP TABLE IF EXISTS services;

DROP TABLE IF EXISTS employees;
DROP TABLE IF EXISTS branches;
DROP TABLE IF EXISTS organizations;

DROP TABLE IF EXISTS users;

DROP TYPE IF EXISTS booking_status;
DROP TYPE IF EXISTS notify_method;

DROP EXTENSION IF EXISTS "uuid-ossp";
