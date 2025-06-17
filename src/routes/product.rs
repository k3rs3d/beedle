use actix_web::{web, HttpResponse};
use tera::{Tera, Context};
use crate::config::Config;
use crate::db::{DbPool, load_product_by_id};
use crate::errors::BeedleError;
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct ProductPath {
    product_id: i32,
}

async fn product_detail(
    pool: web::Data<DbPool>,
    tera: web::Data<Tera>,
    config: web::Data<Config>,
    path: web::Path<ProductPath>
) -> Result<HttpResponse, BeedleError> {
    let conn = pool.get()?;
    let product = load_product_by_id(&conn, path.product_id)?;

    if let Some(product) = product {
        let mut ctx = Context::new();
        ctx.insert("product", &product);
        ctx.insert("site_name", &config.site_name);
        ctx.insert("root_domain", &config.root_domain);

        let rendered = tera.render("product.html", &ctx)?;
        Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
    } else {
        Ok(HttpResponse::NotFound().body("Product not found"))
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/products/{product_id}").route(web::get().to(product_detail)));
}

