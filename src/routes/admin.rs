use crate::config::Config;
use crate::db::{delete_product, load_products, save_product, DbPool};
use crate::errors::BeedleError;
use crate::models::Product;
use actix_web::{web, HttpResponse};
use serde::{Serialize, Deserialize};
use tera::Tera;

#[derive(Debug, Deserialize)]
pub struct ProductForm {
    pub name: String,
    pub price: f64,
    pub inventory: u32,

    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
    pub thumbnail_url: Option<String>,
    pub gallery_urls: Option<Vec<String>>,
    pub tagline: Option<String>,
    pub description: Option<String>,
    pub discount_percent: Option<f64>,
}

async fn list_products(
    pool: web::Data<DbPool>,
    tera: web::Data<Tera>,
    config: web::Data<Config>,
) -> Result<HttpResponse, BeedleError> {
    let conn = pool.get()?;
    let products = load_products(&conn)?;

    let mut ctx = tera::Context::new();
    ctx.insert("products", &products);
    ctx.insert("site_name", &config.site_name); // TODO: make a generic context insertion as a base 

    let rendered = tera.render("admin/products.html", &ctx)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

async fn add_product_form(
    tera: web::Data<Tera>,
    config: web::Data<Config>,
) -> Result<HttpResponse, BeedleError> {
    let mut ctx = tera::Context::new();
    ctx.insert("site_name", &config.site_name);

    let rendered = tera.render("admin/add_product.html", &ctx)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

async fn add_product(
    pool: web::Data<DbPool>,
    form: web::Form<ProductForm>,
) -> Result<HttpResponse, BeedleError> {
    log::info!("Received add product form data: {:?}", form);
    let mut conn = pool.get()?;

    let new_product = Product {
        id: None,
        name: form.name.clone(),
        price: form.price,
        inventory: form.inventory,
        category: form.category.clone().unwrap_or_else(|| "Uncategorized".to_string()),
        tags: form.tags.clone().unwrap_or_default(),
        keywords: vec![],
        thumbnail_url: form.thumbnail_url.clone(),
        gallery_urls: form.gallery_urls.clone().unwrap_or_default(),
        tagline: form.tagline.clone().unwrap_or_default(),
        description: form.description.clone(),
        discount_percent: form.discount_percent,
    };

    if let Err(e) = save_product(&mut conn, &new_product) {
        log::error!("Failed to save product: {:?}", e);
        return Err(e);
    } else {
        log::info!("Product saved successfully: {:?}", form);
    }

    Ok(HttpResponse::SeeOther()
        .append_header(("Location", "/admin/products"))
        .finish())
}

async fn remove_product(
    pool: web::Data<DbPool>,
    product_id: web::Path<i32>,
) -> Result<HttpResponse, BeedleError> {
    let product_id = product_id.into_inner();
    log::info!(
        "Received request to delete product with ID: {:?}",
        product_id
    );
    let mut conn = pool.get()?;

    if let Err(e) = delete_product(&mut conn, product_id) {
        log::error!("Failed to delete product: {:?}", e);
        return Err(e);
    } else {
        log::info!("Product with ID: {:?} deleted successfully", product_id);
    }

    Ok(HttpResponse::SeeOther()
        .append_header(("Location", "/admin/products"))
        .finish())
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/admin/products").route(web::get().to(list_products)))
        .service(web::resource("/admin/add_product").route(web::get().to(add_product_form)))
        .service(web::resource("/admin/add").route(web::post().to(add_product)))
        .service(web::resource("/admin/delete/{product_id}").route(web::post().to(remove_product)));
}
