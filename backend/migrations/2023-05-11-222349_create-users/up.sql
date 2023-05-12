-- Your SQL goes here

CREATE TABLE users (
  id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL ,
  email VARCHAR(255) NOT NULL UNIQUE, 
  password_hash VARCHAR(255) NOT NULL,
  salt VARCHAR(255) NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);


CREATE index users_email_index ON users (email);