use actix_web::{web, HttpResponse};
use tera::{Tera, Context};
use crate::config::Config;
use crate::db::{DbPool};
use crate::errors::BeedleError;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
struct ListParams {
    page: Option<usize>,
    category: Option<String>,
    tag: Option<String>,
    search: Option<String>,
    sort: Option<String>,
}

async fn browse_products(
    pool: web::Data<DbPool>,
    tera: web::Data<Tera>,
    config: web::Data<Config>,
    query: web::Query<ListParams>
) -> Result<HttpResponse, BeedleError> {
    let page = query.page.unwrap_or(1); // /products?page=N
    let per_page = 2;
    let offset = (page - 1) * per_page;

    let conn = pool.get()?;
    let total_products = crate::db::count_products(&conn)?;
    let total_pages = (total_products as f64 / per_page as f64).ceil() as usize;
    let products = crate::db::filter_products(
        &conn,
        query.category.as_deref(),
        query.tag.as_deref(),
        query.search.as_deref(),
        query.sort.as_deref(),
        per_page,
        offset,
    )?;

    let mut ctx = Context::new();
    ctx.insert("products", &products);
    ctx.insert("site_name", &config.site_name);
    ctx.insert("root_domain", &config.root_domain);
    ctx.insert("current_page", &page);
    ctx.insert("total_pages", &total_pages);
    //ctx.insert("params", &query);
   
    let rendered = tera.render("products.html", &ctx)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/products").route(web::get().to(browse_products)));
}

// UNIT TESTING
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web::Data, App};
    use crate::db::establish_connection;

    #[actix_rt::test]
    async fn test_browse_products() {
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
