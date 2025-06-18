use actix_web::{web, HttpResponse};
use actix_session::Session;
use actix_csrf::extractor::CsrfToken;
use tera::Tera;
use crate::config::Config;
use crate::db::{DbPool, load_product_by_id};
use crate::errors::BeedleError;
use crate::session::create_base_context;
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct ProductPath {
    product_id: i32,
}

async fn product_detail(
    pool: web::Data<DbPool>,
    tera: web::Data<Tera>,
    config: web::Data<Config>,
    path: web::Path<ProductPath>,
    csrf_token: CsrfToken,
    session: Session
) -> Result<HttpResponse, BeedleError> {
    let conn = pool.get()?;
    let product = load_product_by_id(&conn, path.product_id)?;

    if let Some(product) = product {
        let mut ctx = create_base_context(&session, config.get_ref());
        ctx.insert("product", &product);
        ctx.insert("csrf_token", &csrf_token);

        let rendered = tera.render("product.html", &ctx)?;
        Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
    } else {
        Ok(HttpResponse::NotFound().body("Product not found"))
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/products/{product_id}").route(web::get().to(product_detail)));
}

