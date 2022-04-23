-- Your SQL goes here
CREATE TABLE tags (
  id SERIAL PRIMARY KEY,
  tag TEXT NOT NULL,
  icon TEXT NOT NULL,
  create_at TIMESTAMP NOT NULL DEFAULT now()
)