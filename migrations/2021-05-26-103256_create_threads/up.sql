-- Your SQL goes here
CREATE TABLE threads (
  id SERIAL PRIMARY KEY,
  slug VARCHAR NOT NULL,
  title VARCHAR NOT NULL,
  primary_tag_id INTEGER NOT NULL,
  content TEXT NOT NULL,
  published BOOLEAN NOT NULL DEFAULT 'f',
  is_open BOOLEAN NOT NULL DEFAULT 't',
  published_at TIMESTAMP NOT NULL DEFAULT now(),
  updated_at TIMESTAMP NOT NULL DEFAULT now()
)