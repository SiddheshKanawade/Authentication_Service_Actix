-- Add migration script here


DROP EXTENSION IF EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
    id uuid default uuid_generate_v4() primary key,
    username varchar not null unique,
    email varchar not null unique,
    password_hash varchar not null,
    full_name varchar null,
    bio varchar null,
    image varchar null,
    --email verification
    --user active or not
    created_at timestamp not null default current_timestamp, 
    updated_at timestamp not null default current_timestamp
)


