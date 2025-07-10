use actix_web::{web, HttpResponse};
use actix_csrf::extractor::CsrfToken;
use std::collections::HashMap;
use tera::Tera;
use crate::config::Config;
use crate::db::{cache,DbPool,filter_products};
use crate::errors::BeedleError;
use crate::session::{create_base_context, ensure_session_cookie, SessionInfo};
use crate::views::ProductView;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
struct ListParams {
    pub page: Option<usize>,
    pub category: Option<String>,
    pub tag: Option<String>,
    pub search: Option<String>,
    pub sort: Option<String>,
}

/// Build a query string from (key, value) pairs
/// Example output: "category=Fruit&search=Apple"
pub fn build_query_string(params: &HashMap<&str, String>) -> String {
    params.iter()
        .filter(|(_k, v)| !v.trim().is_empty())
        .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
        .collect::<Vec<_>>()
        .join("&")
}

async fn browse_products(
    pool: web::Data<DbPool>,
    tera: web::Data<Tera>,
    config: web::Data<Config>,
    query: web::Query<ListParams>,
    csrf_token: CsrfToken,
    session: SessionInfo
) -> Result<HttpResponse, BeedleError> {
    let page = query.page.unwrap_or(1); // /products?page=N
    let per_page = 4;
    let offset = (page - 1) * per_page;
        
    let mut conn = pool.get()?;
    let productlist = filter_products(
        &mut conn,
        query.category.as_deref(),
        query.tag.as_deref(),
        query.search.as_deref(),
        query.sort.as_deref(),
        per_page,
        offset,
    )?;

    let products: Vec<ProductView> = productlist.iter().map(ProductView::from).collect();
    let total_pages = (productlist.len() as f64 / per_page as f64).ceil() as usize;
    let categories = cache::CategoriesCache::get_categories().to_vec();

    // build filter state
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

    let filter_query = build_query_string(&params);

    // HACK
    let request_args = serde_json::json!({
    "category": query.category.clone().unwrap_or_default(),
    "search": query.search.clone().unwrap_or_default(),
    "sort": query.sort.clone().unwrap_or_default(),
    });

    let mut ctx = create_base_context(&session, config.get_ref());
    ctx.insert("products", &products);
    ctx.insert("categories", &categories);
    ctx.insert("filter_query", &filter_query);
    // TODO: also cache tags, add them here as well 
    ctx.insert("request_args", &request_args);
    ctx.insert("current_page", &page);
    ctx.insert("total_pages", &total_pages);
    ctx.insert("csrf_token", &csrf_token.get());
   
    let rendered = tera.render("products.html", &ctx)?;
    
    let response = HttpResponse::Ok().content_type("text/html").body(rendered);

    if session.was_created {
        Ok(ensure_session_cookie(response, session.session_id))
    } else {
        Ok(response)
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/products").route(web::get().to(browse_products)));
}

// UNIT TESTING
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web::Data, App};
    use actix_csrf::CsrfMiddleware;
    use once_cell::sync::Lazy;
    use diesel::{r2d2::ConnectionManager,PgConnection};
    use crate::db::establish_connection;
    

    static POOL: Lazy<DbPool> = Lazy::new(|| {
        dotenv::dotenv().ok();

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.")
    });

    #[actix_rt::test]
    async fn test_browse_products() {
        let pool = POOL.get().expect("Failed to get a connection from the pool");
        let tera = Tera::new("templates/**/*").unwrap();
        let config = Config {
            site_name: String::from("Test Site"),
            root_domain: String::from("http://localhost"),
        };

        let csrf = CsrfMiddleware::with_rng(rand::rngs::OsRng)
        .set_cookie(actix_web::http::Method::GET, "/add_to_cart")
        .set_cookie(actix_web::http::Method::GET, "/cart")
        .set_cookie(actix_web::http::Method::GET, "/product")
        .set_cookie(actix_web::http::Method::GET, "/products");

        let mut app = test::init_service(
            App::new()
                .app_data(Data::new(pool))
                .app_data(Data::new(tera))
                .app_data(Data::new(config))
                .wrap(csrf)
                .configure(init)
        ).await;

        let req = test::TestRequest::get().uri("/products").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());

        // Check the response body
        let body = test::read_body(resp).await;
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        assert!(body_str.contains("Test Site"));  // Check if "Test Site" appears in the response
    }

    #[actix_rt::test]
    async fn test_browse_products_invalid_path() {
        let pool = establish_connection().expect("Failed to create pool.");
        let tera = Tera::new("templates/**/*").unwrap();
        let config = Config {
            site_name: String::from("Test Site"),
            root_domain: String::from("http://localhost"),
        };

        let mut app = test::init_service(
            App::new()
                .app_data(Data::new(pool.clone()))
                .app_data(Data::new(tera))
                .app_data(Data::new(config))
                .configure(init)
        ).await;

        // Invalid path test
        let req = test::TestRequest::get().uri("/invalid_path").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_client_error());  // Expect a 4xx client error
    }
}
