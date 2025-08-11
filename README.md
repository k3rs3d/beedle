# Beedle

A simple e-commerce platform written in Rust. Customizable HTML templates and minimal overhead.

I found all existing e-commerce platforms to be inefficient, overly complicated, subscription-based, or insufficiently customizable. This is my attempt to make a modern, "just works", single-binary, no-nonsense store platform for small business and personal use. 

HTML should be the primary interface for customizability. Shop appearance and most user-facing logic is customizable via templates. Anyone familiar with HTML (and optionally Tera's syntax) can fully redesign the store experience. You do *not* have to use Rust or touch backend code!

Designed for small and medium businesses. Easily supports thousands of concurrent users given Actix, database transactions, and zero shared state. Probably not suitable for Amazon or million-user shops, unfortunately. 

## Getting Started

1. Install [Rust](https://rustup.rs/), [Postgres](https://www.postgresql.org/), and a reverse proxy (like Nginx) if you want SSL/rate limits.
2. Set up the project: clone repo & compile the application with `cargo build`. 
3. Edit templates under `templates/` to control appearance and functionality.
4. Add products directly in the DB for now. A GUI browser for postgres is currently recommended. Admin panel coming soon! 

## TODO

Vaguely:

- Better browsing: Product "sections", more product metadata, better filtering, and more.
- User accounts... wishlists, order history, product recommendations...? 
- More accounting-friendly features, such as transaction records, etc.
- Even more Tera context options for deeper customization.
- Admin control panel for all edits and initial setup.
