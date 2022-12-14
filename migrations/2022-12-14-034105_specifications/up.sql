-- Your SQL goes here

CREATE TABLE specifications (
    id SERIAL PRIMARY KEY,
    names_values JSONB,
    product_id INTEGER NOT NULL REFERENCES products(id) ON DELETE CASCADE
)
