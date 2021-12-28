-- Your SQL goes here
CREATE TABLE tasks (
    id serial PRIMARY KEY,
    description VARCHAR NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT 'f',
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL
)