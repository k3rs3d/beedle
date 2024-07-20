use crate::config::Config;
use crate::db::{delete_product, load_products, save_product, DbPool};
use crate::errors::BeedleError;
use crate::models::Product;
use actix_web::{web, HttpResponse};
use tera::Tera;

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
    form: web::Form<Product>,
) -> Result<HttpResponse, BeedleError> {
    log::info!("Received add product form data: {:?}", form);
    let mut conn = pool.get()?;

    let new_product = Product {
        id: None, // Ensure new product has no id
        name: form.name.clone(),
        price: form.price,
        inventory: form.inventory,
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
