-- Your SQL goes here
CREATE TABLE users (
  id SERIAL PRIMARY KEY ,
  username VARCHAR(100) NOT NULL,
  email VARCHAR(100) UNIQUE NOT NULL,
  password VARCHAR(64) NOT NULL,
  avatar VARCHAR NULL,
  biography VARCHAR NULL,
  created_at TIMESTAMP NOT NULL  DEFAULT NOW() 
);

CREATE INDEX users_email_username_idx ON users (email , username);