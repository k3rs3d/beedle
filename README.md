# Beedle

A simple e-commerce platform written in Rust. Focused on customizable HTML templates and minimal overhead.

I found all existing e-commerce platforms to be inefficient, overly complicated, subscription-based, or insufficiently customizable. This is my attempt to make a modern, "just works", single-binary, no-nonsense store platform for small business and personal use. HTML should be the primary interface for customizability.

## Benefits

- **Customizable with Only HTML:** Shop appearance and most user-facing logic is customizable via templates. Anyone familiar with HTML (and optionally Tera's syntax) can fully rebrand, redesign, and change the store experience. You do *not* have to use Rust or touch backend code!
- **No Subscription or Vendor Lock-In:** Truly free & open-source. Run your own store, own your data.
- **Fast, Efficient, Modern:** Built with Rust for maximum safety, speed, and maintainability.
- **Scalable**: Designed for small and medium businesses. Easily supports thousands of concurrent users given Actix, database transactions, and zero shared state. Probably not suitable for Amazon or million-user shops, unfortunately. 

## Limitations

- **Early security:** Intended for shops where a human reviews every order. Not recommended for sensitive/high-value automated orders (like digital software) until further code review and feature hardening. Designed to be run behind a secure, rate-limited reverse proxy like nginx.
- **No fancy cart gadgets or reporting out-of-the-box:** Feature set kept minimal by design!
- **Operator must set up reverse proxy and payments:** Infrastructure is up to you.
- **Not enterprise ready:** No distributed deployments, no support.

## Getting Started

1. Install [Rust](https://rustup.rs/), [Postgres](https://www.postgresql.org/), and a reverse proxy (like Nginx) if you want SSL/rate limits.
2. Set up the project: clone repo & compile the application with cargo. 
3. Edit templates under `templates/` to control appearance and functionality.
4. Add products directly in the DB for now. A GUI browser for postgres is currently recommended. Admin panel coming soon! 

## Roadmap & TODO

Vaguely:

- Better browsing: Product "sections", more product metadata, better filtering, and more.
- User accounts... wishlists, order history...? 
- More accounting-friendly features, such as transaction records, etc.
- Even more Tera context options for deeper customization.
- Admin control panel for all edits and initial setup.

---

**Contributors welcome!**