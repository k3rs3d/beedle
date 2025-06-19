CREATE TABLE product (
   id SERIAL PRIMARY KEY,
   name TEXT NOT NULL,
   price NUMERIC NOT NULL,
   inventory INTEGER NOT NULL,
   category TEXT NOT NULL,
   tags TEXT,
   keywords TEXT,
   thumbnail_url TEXT,
   gallery_urls TEXT,
   tagline TEXT,
   description TEXT,
   discount_percent REAL
);
