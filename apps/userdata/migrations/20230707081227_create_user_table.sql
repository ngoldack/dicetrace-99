-- Creating a table for users
CREATE TABLE users (
    id CHAR(12) PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL
);

