-- Your SQL goes here

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    password VARCHAR(255) NOT NULL,
    role VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    address VARCHAR(255) ,
    phone VARCHAR(255),
    verified BOOLEAN DEFAULT FALSE,
    img_url VARCHAR(255)
)