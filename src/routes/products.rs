//! Product listing ("Browse") page: /products, with filters/pagination.

use actix_web::{web, HttpResponse};
use actix_csrf::extractor::CsrfToken;
use std::collections::HashMap;
use tera::Tera;
use crate::config::Config;
use crate::db::{cache, DbPool, products::filter_products, products::count_filtered_products};
use crate::errors::BeedleError;
use crate::session::{create_base_context, ensure_session_cookie, SessionInfo};
use crate::views::ProductView;
use serde::{Serialize, Deserialize};

const PER_PAGE: usize = 4;

#[derive(Deserialize, Serialize, Debug)]
struct ListParams {
    pub page: Option<usize>,
    pub category: Option<String>,
    pub tag: Option<String>,
    pub search: Option<String>,
    pub sort: Option<String>,
}

/// Build a urlencoded query string 
/// Example: {category: "Fruit", search: "Apple"} -> "category=Fruit&search=Apple"
pub fn build_query_string(params: &HashMap<&str, String>) -> String {
    params.iter()
        .filter(|(_k, v)| !v.trim().is_empty())
        .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
        .collect::<Vec<_>>()
        .join("&")
}

/// Product listing page.
/// Handles filtering+sorting+pagination and passes to Tera.
async fn browse_products(
    pool: web::Data<DbPool>,
    tera: web::Data<Tera>,
    config: web::Data<Config>,
    query: web::Query<ListParams>,
    csrf_token: CsrfToken,
    session: SessionInfo,
) -> Result<HttpResponse, BeedleError> {
    let page = query.page.unwrap_or(1).max(1);
    let offset = (page - 1) * PER_PAGE;

    log::debug!(
        "browse_products: page={} category={:?} tag={:?} search={:?} sort={:?}",
        page, query.category, query.tag, query.search, query.sort
    );

    // Get DB connection
    let mut conn = pool.get().map_err(|e| {
        log::error!("DB pool error (products): {}", e);
        BeedleError::DatabaseError(e.to_string())
    })?;

    // Total number of items for these filters
    let total_items = count_filtered_products(
        &mut conn,
        query.category.as_deref(),
        query.tag.as_deref(),
        query.search.as_deref(),
    )?;

    let total_pages = if total_items == 0 {
        1
    } else {
        // Rust-style ceil division lol 
        ((total_items + (PER_PAGE as i64) - 1) / (PER_PAGE as i64)) as usize
    };

    // Fetch filtered products
    let productlist = filter_products(
        &mut conn,
        query.category.as_deref(),
        query.tag.as_deref(),
        query.search.as_deref(),
        query.sort.as_deref(),
        PER_PAGE,
        offset,
    ).map_err(|e| {
        log::error!("Product filter query failed: {}", e);
        e
    })?;



    // Convert Product models to renderable ProductView
    let products: Vec<ProductView> = productlist.iter().map(ProductView::from).collect();

    // Load all unique categories for sidebar/category selection
    let categories = cache::CategoriesCache::get_categories().to_vec();

    // Rebuild filter params for keeping query params when paginating/filtering in the template
    let mut params = HashMap::new();
    if let Some(s) = query.category.as_ref().filter(|s| !s.trim().is_empty()) {
        params.insert("category", s.clone());
    }
    if let Some(s) = query.sort.as_ref().filter(|s| !s.trim().is_empty()) {
        params.insert("sort", s.clone());
    }
    if let Some(s) = query.search.as_ref().filter(|s| !s.trim().is_empty()) {
        params.insert("search", s.clone());
    }
    // TODO: add tag filter 

    let filter_query = build_query_string(&params);

    // For building other URLs within the template 
    let request_args = serde_json::json!({
        "category": query.category.clone().unwrap_or_default(),
        "search": query.search.clone().unwrap_or_default(),
        "sort": query.sort.clone().unwrap_or_default(),
        // "tag": query.tag.clone().unwrap_or_default(),
    });

    let mut ctx = create_base_context(&session, config.get_ref());
    ctx.insert("products", &products);
    ctx.insert("categories", &categories);

    ctx.insert("filter_query", &filter_query);
    ctx.insert("request_args", &request_args);
    ctx.insert("current_page", &page);
    ctx.insert("total_pages", &total_pages);
    ctx.insert("csrf_token", &csrf_token.get());

    // Render catalog template
    let rendered = tera.render("products.html", &ctx).map_err(|e| {
        log::error!("Tera render error (products): {}", e);
        BeedleError::TemplateError(e)
    })?;

    // Set session cookie if new, before sending response
    let response = HttpResponse::Ok().content_type("text/html").body(rendered);
    if session.was_created {
        Ok(ensure_session_cookie(response, session.session_id))
    } else {
        Ok(response)
    }
}

/// Register Actix route at /products
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/products")
            .route(web::get().to(browse_products))
    );
}