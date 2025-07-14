pub mod admin;
pub mod index;
pub mod product;
pub mod products;
pub mod cart;
pub mod checkout;

use actix_web::{web, HttpResponse};
use tera::Tera;

pub fn init(cfg: &mut web::ServiceConfig) {
    admin::init(cfg);
    index::init(cfg);
    product::init(cfg);
    products::init(cfg);
    cart::init(cfg);
    checkout::init(cfg);
}

// 404 handler
pub async fn not_found_handler(
    tera: web::Data<Tera>,
) -> HttpResponse {
    let ctx = tera::Context::new();
    //ctx.insert("message", &format!("Route not found: {}", req.path()));
    // TODO: Pre-render 404 page? 
    let rendered = tera.render("404.html", &ctx)
        .unwrap_or_else(|e| {
            log::error!("Tera 404 render error: {}", e);
            "404 Not Found".to_string()
        });
    HttpResponse::NotFound().content_type("text/html").body(rendered)
}