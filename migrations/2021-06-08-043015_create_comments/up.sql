-- Your SQL goes here
CREATE TABLE threads (
  id SERIAL PRIMARY KEY,
  author VARCHAR,
  content TEXT NOT NULL,
  thread INT NOT NULL,
  published_at TIMESTAMP NOT NULL DEFAULT now(),
  updated_at TIMESTAMP NOT NULL DEFAULT now()
  
)