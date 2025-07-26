use crate::config::Config;
use crate::db::{products, DbPool};
use crate::errors::BeedleError;
use crate::models::NewProduct;
use crate::session::{create_base_context, SessionInfo};
use actix_web::{web, HttpResponse};
use diesel::RunQueryDsl;
use serde::{Deserialize};
use tera::Tera;

#[derive(Debug,Deserialize)]
pub struct ProductForm {
    pub name: String,
    pub price: i64,
    pub inventory: i32,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
    pub keywords: Option<Vec<String>>,
    pub thumbnail_url: Option<String>,
    pub gallery_urls: Option<Vec<String>>,
    pub tagline: Option<String>,
    pub description: Option<String>,
    pub discount_percent: Option<f32>,
    pub date_added: Option<chrono::NaiveDateTime>,
    pub date_restock_expected: Option<chrono::NaiveDateTime>
}

async fn list_products(
    pool: web::Data<DbPool>,
    tera: web::Data<Tera>,
    session: SessionInfo,
    config: web::Data<Config>
) -> Result<HttpResponse, BeedleError> {
    let mut conn = pool.get()?;
    let products = products::load_products(&mut conn)?;

    let mut ctx = create_base_context(&session, config.get_ref());
    ctx.insert("products", &products);
    ctx.insert("site_name", &config.site_name); // TODO: make a generic context insertion as a base 

    let rendered = tera.render("admin/products.html", &ctx)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

async fn add_product_form(
    tera: web::Data<Tera>,
    session: SessionInfo,
    config: web::Data<Config>
) -> Result<HttpResponse, BeedleError> {
    let ctx = create_base_context(&session, config.get_ref());

    let rendered = tera.render("admin/add_product.html", &ctx)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

async fn add_product(
    pool: web::Data<DbPool>,
    form: web::Form<ProductForm>,
) -> Result<HttpResponse, BeedleError> {
    log::info!("Received add product form data: {:?}", form);

    let mut conn = pool.get()?;

    // Convert Vec<String> to comma-separated String (for CSV storage)
    let tags_csv = form.tags.as_ref().map(|v| v.join(","));
    let keywords_csv = form.keywords.as_ref().map(|v| v.join(","));
    let gallery_urls_csv = form.gallery_urls.as_ref().map(|v| v.join(","));

    let new_product = NewProduct {
        name: form.name.clone(),
        price: form.price.clone(),
        inventory: form.inventory,
        category: form.category.clone().unwrap_or_else(|| "Uncategorized".to_string()),
        tags: tags_csv,
        keywords: keywords_csv,
        thumbnail_url: form.thumbnail_url.clone(),
        gallery_urls: gallery_urls_csv,
        tagline: form.tagline.clone(),
        description: form.description.clone(),
        discount_percent: form.discount_percent,
        added_date: form.date_added,
        restock_date: form.date_restock_expected
    };

    use crate::schema::product::dsl::*;
    match diesel::insert_into(product)
        .values(&new_product)
        .execute(&mut conn)
    {
        Ok(_) => {
            log::info!("Product saved successfully: {:?}", new_product);
            Ok(HttpResponse::SeeOther()
                .append_header(("Location", "/admin/products"))
                .finish())
        }
        Err(e) => {
            log::error!("Failed to save product: {:?}", e);
            Err(BeedleError::DatabaseError(e.to_string()))
        }
    }
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

    if let Err(e) = products::delete_product(&mut conn, product_id) {
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
