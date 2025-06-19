use actix_web::{middleware, App, cookie::Key, HttpServer, web::Data};
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_csrf::CsrfMiddleware;
use actix_files::Files;
use dotenv::dotenv;
use std::env;
use tera::Tera;

mod config;
mod db; mod schema;
//mod email;
mod errors;
mod models;
mod pay;
mod routes;
mod session;
mod views;

use errors::BeedleError;

fn init_environment() {
    dotenv().ok();
}

fn get_secret_key() -> Key {
    let key_hex = env::var("SESSION_KEY").expect("SESSION_KEY must be set in .env!");
    let key_bytes = hex::decode(key_hex).expect("SESSION_KEY must be valid hex");
    Key::derive_from(&key_bytes)
}

#[actix_web::main]
async fn main() -> Result<(), BeedleError> {
    init_environment();

    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    // Load config settings 
    let config = config::Config::from_file("config.json").map_err(|e| errors::BeedleError::ConfigError(e.to_string()))?;

    // TODO: Make these configurable via config file?
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    let secret_key = get_secret_key();
    let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"))?;

    // Establish connection and initialize the database
    let pool = crate::db::establish_connection().expect("Failed to create pool.");
    let mut conn = pool.get().unwrap();
    // Initialize database
    crate::db::init_db(&mut conn).expect("Failed to initialize database.");

    HttpServer::new(move || {
        let csrf = CsrfMiddleware::with_rng(rand::rngs::OsRng)
        .set_cookie(actix_web::http::Method::GET, "/cart")
        .set_cookie(actix_web::http::Method::GET, "/products")
        .set_cookie(actix_web::http::Method::GET, "/products/{product_id}");

        App::new()
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(config.clone())) // Add config to app data
            .app_data(Data::new(tera.clone()))   // Add Tera to app data
            .configure(routes::init) // Init routes
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            ))
            .wrap(csrf)
            .wrap(middleware::Logger::default()) // Logger            
            .service(Files::new("/static", "./static").show_files_listing()) // Serve files from `static` directory
            // TODO: Make the static directory configurable 
    })
    .bind(format!("{}:{}", host, port)).map_err(BeedleError::from)?
    .run()
    .await.map_err(BeedleError::from)?;
    Ok(())
}
