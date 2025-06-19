use actix_session::Session;
use actix_web::{web, HttpResponse, http::header};
use actix_csrf::extractor::{Csrf, CsrfToken, CsrfGuarded};
use tera::Tera;
use crate::errors::BeedleError;
use crate::config::Config;
use crate::db::{DbPool, load_products};
use crate::models::Product;
use crate::session::{get_cart, update_cart_quantity, create_base_context};

#[derive(serde::Deserialize)]
struct AddToCartInfo {
    product_id: i32,
    quantity: i32,
    csrf_token: CsrfToken
}

impl CsrfGuarded for AddToCartInfo {
    fn csrf_token(&self) -> &CsrfToken {
        &self.csrf_token
    }
}

async fn add_to_cart(
    session: Session,
    form: Csrf<web::Form<AddToCartInfo>>,
) -> Result<HttpResponse, BeedleError> {
    let info = form.into_inner().into_inner();
    update_cart_quantity(&session, info.product_id, info.quantity);
    Ok(HttpResponse::SeeOther().append_header((header::LOCATION, "/products")).finish())
}

async fn remove_from_cart(
    session: Session, 
    form: Csrf<web::Form<AddToCartInfo>>,
) -> Result<HttpResponse, BeedleError> {
    let info = form.into_inner().into_inner();
    update_cart_quantity(&session, info.product_id, -1);
    Ok(HttpResponse::SeeOther().append_header(("Location", "/cart")).finish())
}

async fn view_cart(
    pool: web::Data<DbPool>,
    tera: web::Data<Tera>,
    config: web::Data<Config>,
    session: Session,
    csrf_token: CsrfToken
) -> Result<HttpResponse, BeedleError> {
    let mut conn = pool.get()?;
    let cart = get_cart(&session);
    let products = load_products(&mut conn)?;
   
    let cart_items: Vec<(Product, u32)> = cart.iter()
        .map(|item| {
            products.iter() // TODO: Better handling for Some(id):
                .find(|product| product.id == item.product_id)
                .map(|product| (product.clone(), item.quantity))
        })
        .filter_map(|item| item)
        .collect();
    let mut ctx = create_base_context(&session, config.get_ref());
    ctx.insert("cart_items", &cart_items);
    ctx.insert("site_name", &config.site_name);
    ctx.insert("csrf_token", &csrf_token.get());
   
    let rendered = tera.render("cart.html", &ctx).map_err(|e| {
        eprintln!("Template rendering error: {}", e);
        BeedleError::TemplateError(e) // tera::Error
    })?;
   
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/add_to_cart/").route(web::post().to(add_to_cart)))
       .service(web::resource("/remove_from_cart/").route(web::post().to(remove_from_cart)))
        .service(web::resource("/cart").route(web::get().to(view_cart)));
}