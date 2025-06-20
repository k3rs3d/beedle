pub mod admin;
pub mod index;
pub mod product;
pub mod products;
pub mod cart;
pub mod checkout;

use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    admin::init(cfg);
    index::init(cfg);
    product::init(cfg);
    products::init(cfg);
    //cfg.service(web::resource("/products").route(web::get().to(browse_products)));
    cart::init(cfg);
    checkout::init(cfg);
}
