pub mod admin;
pub mod products;
pub mod cart;
pub mod checkout;

use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    admin::init(cfg);
    products::init(cfg);
    cart::init(cfg);
    checkout::init(cfg);
}