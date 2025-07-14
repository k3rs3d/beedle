//! `main.rs` - Application entrypoint. Sets up logging, configuration, DB, routes, and runs Actix server.

use actix_csrf::CsrfMiddleware;
use actix_files::Files;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, middleware, web::to, web::Data, App, HttpServer};
use tera::Tera;

mod config;
mod db;
mod errors;
mod models;
mod pay;
mod routes;
mod schema;
mod session;
mod views;

use crate::errors::BeedleError;

fn init_environment() {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
}

/// Loads secret key for cookies/sessions from env or fails clearly.
fn get_secret_key() -> Result<Key, BeedleError> {
    let key_hex = std::env::var("SESSION_KEY")
        .map_err(|e| BeedleError::ConfigError(format!("SESSION_KEY is missing: {e}")))?;
    let key_bytes = hex::decode(key_hex)
        .map_err(|e| BeedleError::ConfigError(format!("SESSION_KEY isn't valid hex: {e}")))?;
    Ok(Key::derive_from(&key_bytes))
}

/// Loads and validates global configuration (JSON + env vars).
fn load_config() -> Result<config::Config, BeedleError> {
    config::Config::from_file("config.json").map_err(|e| BeedleError::ConfigError(e.to_string()))
}

fn load_tera_templates() -> Result<Tera, BeedleError> {
    Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"))
        .map_err(|e| BeedleError::ConfigError(format!("Template load failed: {e}")))
}

/// Returns a tuple (host, port) as strings (filled from env or defaults).
fn get_server_bind() -> (String, String) {
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_owned());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_owned());
    (host, port)
}

/// Everything needed for setting up (or recovering) the database.
fn setup_database() -> Result<db::DbPool, BeedleError> {
    let pool = db::establish_connection()?;
    let mut conn = pool
        .get()
        .map_err(|_| BeedleError::DatabaseError("Pool get failed".into()))?;
    db::init_db(&mut conn)?;
    Ok(pool)
}

#[actix_web::main]
async fn main() -> Result<(), BeedleError> {
    init_environment();

    let config = load_config()?;
    let tera = load_tera_templates()?;
    let secret_key = get_secret_key()?;
    let (host, port) = get_server_bind();

    // arc so threads/tasks dont have to think about ownership/lifetime
    let config = async_std::sync::Arc::new(config);

    log::info!("Starting on http://{}:{}", host, port);

    let pool = setup_database()?;

    let server = HttpServer::new(move || {
        let csrf = CsrfMiddleware::with_rng(rand::rngs::OsRng)
            .set_cookie(actix_web::http::Method::GET, "/cart")
            .set_cookie(actix_web::http::Method::GET, "/products")
            .set_cookie(actix_web::http::Method::GET, "/products/{product_id}");

        App::new()
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(config.clone()))
            .app_data(Data::new(tera.clone()))
            .configure(routes::init)
            .default_service(
            to(crate::routes::not_found_handler)
            )
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            ))
            .wrap(csrf)
            .wrap(middleware::Logger::default())
            .service(Files::new("/static", "./static").show_files_listing())
    })
    .bind(format!("{}:{}", host, port))
    .map_err(BeedleError::from)?
    .run();

    // Graceful shutdown 
    let shutdown_signal = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to listen for shutdown signal");
        log::warn!("Received Ctrl-C or SIGTERM, shutting down...");
    };
    tokio::select! {
        res = server => {
            if let Err(e) = res {
                log::error!("HttpServer exited with error: {e}");
                return Err(BeedleError::from(e));
            }
        }
        _ = shutdown_signal => {
            // call .stop() to expedite
            log::info!("Actix server shutting down gracefully.");
        }
    }

    Ok(())
}
