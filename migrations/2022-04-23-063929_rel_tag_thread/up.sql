-- Your SQL goes here
CREATE TABLE rel_tag_thread (
  id SERIAL PRIMARY KEY,
  tag INT NOT NULL,
  thread INT NOT NULL
)