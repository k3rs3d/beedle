use actix_session::Session;
use actix_web::{web, HttpResponse};
use crate::db::{DbPool, update_inventory};
use crate::pay::process_payment;
use crate::session::get_cart;
use crate::errors::BeedleError;

async fn checkout(
    pool: web::Data<DbPool>,
    session: Session,
) -> Result<HttpResponse, BeedleError> {
    let cart = get_cart(&session);
    if cart.is_empty() {
        return Ok(HttpResponse::BadRequest().body("Cart is empty"));
    }
    match process_payment(1, "hi").await {
        Ok(_) => {
            let mut conn = pool.get()?;
            update_inventory(&mut conn, &cart)?;
            session.purge();
            Ok(HttpResponse::Ok().body("Checkout completed"))
        }
        Err(e) => Ok(HttpResponse::InternalServerError().body(format!("Payment failed: {}", e))),
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/checkout").route(web::post().to(checkout)));
}