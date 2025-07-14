# Beedle

A lightweight, simple Rust-based e-commerce platform focused on customizable HTML templates and minimal overhead.

I found all existing e-commerce platforms to be inefficient, overly complicated, subscription-based, or insufficiently customizable. This is my attempt to make a modern, "just works", single-binary, no-nonsense store platform for small business and personal use. HTML should be the primary interface for deep customizability.

## Benefits

- **Customizable with Only HTML:** Shop appearance and most user-facing logic is customizable via templates. Anyone familiar with HTML (and optionally Tera's syntax) can fully rebrand, redesign, and change the store experience. You do **not** have to use Rust or touch backend code!
- **No Subscription, No Vendor Lock-In:** Truly free & open-source. Run your own store, own your data.
- **Fast, Efficient, Modern:** Built with Rust for maximum safety, speed, and maintainability.
- **Scalable Enough**: Designed for small and medium businesses. Easily supports thousands of concurrent users given Actix, database transactions, and zero shared state. Not for Amazon or million-user shops, unfortunately. 

## Limitations

- **Not enterprise ready:** No distributed deployments, complex microservices, or support contracts.
- **No built-in automated digital fulfillment/instant delivery (yet)** built for shops where a human reviews every order.
- **Early security:** Not recommended for sensitive/high-value automated orders (like digital software) until further code review and feature hardening. Designed to be run behind a secure, rate-limited reverse proxy like nginx.
- **No fancy cart gadgets or reporting out-of-the-box:** Feature set kept minimal by design!
- **Operator must set up a reverse proxy and payments:** Infrastructure is up to you.

## Getting Started

1. Install [Rust](https://rustup.rs/), [Postgres](https://www.postgresql.org/), and a reverse proxy (like Nginx) if you want SSL/rate limits.
2. Set up the project: install cargo, clone repo, compile the application with cargo. 
3. Edit templates under `templates/` to control appearance and functionality.
4. Add products directly in the DB for now. A GUI browser for postgres is currently recommended. Admin panel coming soon! 

## Roadmap & TODO

Vaguely:

- Better browsing: Product "sections", more product metadata, better filtering, and more.
- User accounts... wishlists, order history...? 
- More accounting-friendly features, such as transaction records, etc.
- Even more Tera context options for deeper customization.
- Admin control panel for all edits and initial setup.

## Security Notes

**Not yet hardened for automated digital sales!** Usable for "real" shops, with a human fulfilling orders. All database actions are logged, all input is checked, but you should still audit before trusting it for critical use. Intended to run behind reverse proxy such as NGINX. Session data is server side. Only a UUID session cookie for users. 

Further security hardening welcome!

---

**Contributors welcome!**