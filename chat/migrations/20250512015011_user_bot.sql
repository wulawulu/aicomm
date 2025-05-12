-- Add migration script here
ALTER TABLE users
    ADD COLUMN is_bot BOOLEAN NOT NULL DEFAULT FALSE;
