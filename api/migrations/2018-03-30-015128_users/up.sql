CREATE TABLE users (
  id INTEGER UNSIGNED NOT NULL AUTO_INCREMENT UNIQUE PRIMARY KEY,
  email VARCHAR(255) NOT NULL UNIQUE,
  token VARCHAR(255) NOT NULL
);