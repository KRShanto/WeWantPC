-- Your SQL goes here
-- id
-- names_values
-- product_id

CREATE TABLE specifications (
    id SERIAL PRIMARY KEY,
    names_values JSONB,
    product_id INTEGER NOT NULL REFERENCES products(id) ON DELETE CASCADE
)