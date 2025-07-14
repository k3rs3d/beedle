//! Product detail page route for /products/{product_id}

use crate::config::Config;
use crate::db::{products::load_product_by_id, DbPool};
use crate::errors::BeedleError;
use crate::session::{create_base_context, SessionInfo};
use crate::views::ProductView;
use actix_csrf::extractor::CsrfToken;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use tera::Tera;

/// Path extractor for /products/{product_id}
#[derive(Deserialize)]
pub struct ProductPath {
    pub product_id: i32,
}

/// Displays the product detail page for a single product by numeric id.
/// Renders 404 if not found.
async fn product_detail(
    pool: web::Data<DbPool>,
    tera: web::Data<Tera>,
    config: web::Data<Config>,
    path: web::Path<ProductPath>,
    csrf_token: CsrfToken,
    session: SessionInfo,
) -> Result<HttpResponse, BeedleError> {
    let mut conn = pool.get()?;

    let product_id = path.product_id;
    log::debug!("Loading product detail for id {}", product_id);

    let dbproduct = load_product_by_id(&mut conn, product_id)?;

    match dbproduct {
        Some(db_prod) => {
            let product = ProductView::from(&db_prod);
            let mut ctx = create_base_context(&session, config.get_ref());
            ctx.insert("product", &product);
            ctx.insert("csrf_token", &csrf_token.get());

            let rendered = tera.render("product.html", &ctx).map_err(|e| {
                log::error!("Tera render failed for product {}: {e}", product_id);
                BeedleError::TemplateError(e)
            })?;
            Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
        }
        None => {
            log::warn!("Product not found: id {}", product_id);

            let ctx = crate::session::create_base_context(&session, &config);
            //ctx.insert("message", &format!("Product not found (id {})", product_id));

            // Try to render the 404.html template.
            let rendered = tera.render("404.html", &ctx).unwrap_or_else(|e| {
                log::error!("404.html render error: {e}");
                "404 Not Found".to_string()
            });

            Ok(HttpResponse::NotFound()
                .content_type("text/html")
                .body(rendered))
        }
    }
}

/// Registers route /products/{product_id} with Actix.
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/products/{product_id}").route(web::get().to(product_detail)));
}
