CREATE TABLE product (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    price BIGINT NOT NULL,
    inventory INT NOT NULL,
    category TEXT NOT NULL,
    tags TEXT,
    keywords TEXT,
    thumbnail_url TEXT,
    gallery_urls TEXT,
    tagline TEXT,
    description TEXT,
    discount_percent REAL,
    added_date TIMESTAMP NOT NULL DEFAULT now(),
    restock_date TIMESTAMP
);