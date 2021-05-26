-- Your SQL goes here
CREATE TABLE threads (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 'f',
  is_open BOOLEAN NOT NULL DEFAULT 't',
  published_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL
)