# `beedle`

This is a dead simple ecommerce platform built with Rust. It uses `actix` for the web server, `tera` for templating, and `rusqlite` for the database. 

This is a test project, and probably shouldn't be used. I only made this to practice developing web applications in Rust. 

## Current Features

- Lists available products 
- Rudimentary admin control panel for inventory management 
- Cart funtionality, saved with a session cookie 

## Planned Features

- Better templates 
- Quantity features (add x items to cart)
- Basic checkout process with (simulated) payment handling.
- Replace simulated payment with actual payment integration?
- Email: Receipts, order confirmations, etc.
- Live update / auto refresh when template files are modified? 
- Dockerizing?

## Unplanned Features

While most ecommerce systems might include these features, here's what I *don't* plan to add: 

- User registration & authentication
- Saving payment info
- Ads/marketing features