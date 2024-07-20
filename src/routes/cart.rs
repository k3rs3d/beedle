use actix_session::Session;
use actix_web::{web, HttpResponse};
use tera::{Tera, Context};
use crate::errors::BeedleError;
use crate::config::Config;
use crate::db::{DbPool, load_products};
use crate::models::Product;
use crate::session::{get_cart, add_to_cart as session_add_to_cart, remove_from_cart as session_remove_from_cart};

async fn add_to_cart(session: Session, product_id: web::Path<u32>) -> Result<HttpResponse, BeedleError> {
    session_add_to_cart(&session, *product_id);
    Ok(HttpResponse::SeeOther().append_header(("Location", "/products")).finish())
}

async fn remove_from_cart(session: Session, product_id: web::Path<u32>) -> Result<HttpResponse, BeedleError> {
    session_remove_from_cart(&session, *product_id);
    Ok(HttpResponse::SeeOther().append_header(("Location", "/cart")).finish())
}

async fn view_cart(
    pool: web::Data<DbPool>,
    tera: web::Data<Tera>,
    config: web::Data<Config>,
    session: Session,
) -> Result<HttpResponse, BeedleError> {
    let conn = pool.get()?;
    let cart = get_cart(&session);
    let products = load_products(&conn)?;
   
    let cart_items: Vec<(Product, u32)> = cart.iter()
        .map(|item| {
            products.iter() // TODO: Better handling for Some(id):
                .find(|product| product.id == Some(item.product_id))
                .map(|product| (product.clone(), item.quantity))
        })
        .filter_map(|item| item)
        .collect();
    let mut ctx = Context::new();
    ctx.insert("cart_items", &cart_items);
    ctx.insert("site_name", &config.site_name);
   
    let rendered = tera.render("cart.html", &ctx).map_err(|e| {
        eprintln!("Template rendering error: {}", e);
        BeedleError::TemplateError(e) // tera::Error
    })?;
   
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

async fn test_page(
    tera: web::Data<Tera>,
    config: web::Data<Config>,
) -> Result<HttpResponse, BeedleError> {
    let mut ctx = Context::new();
    ctx.insert("site_name", &config.site_name);
    let rendered = tera.render("test.html", &ctx).map_err(|e| {
        eprintln!("Template rendering error: {}", e);
        BeedleError::TemplateError(e) 
    })?;
   
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/add_to_cart/{product_id}").route(web::post().to(add_to_cart)))
        .service(web::resource("/remove_from_cart/{product_id}").route(web::post().to(remove_from_cart)))
        .service(web::resource("/cart").route(web::get().to(view_cart)))
        .service(web::resource("/test").route(web::get().to(test_page)));
}