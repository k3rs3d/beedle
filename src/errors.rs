use actix_web::{HttpResponse, ResponseError};
use actix_web::http::StatusCode;
use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
pub enum BeedleError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] rusqlite::Error),
    
    #[error("Template error: {0}")]
    TemplateError(#[from] tera::Error),
    
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Inventory error: {0}")]
    InventoryError(String),

    #[error("I/O error: {0}")]
    IOError(#[from] io::Error),

    #[error("Response error: {0}")]
    ResponseError(#[from] actix_web::Error),

    #[error("Pool error: {0}")]
    PoolError(#[from] r2d2::Error),
    
    //#[error("Unknown error")]
    //Unknown,
}

// Actix response error 
impl ResponseError for BeedleError {
    fn status_code(&self) -> StatusCode {
        match *self {
            BeedleError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            BeedleError::TemplateError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            BeedleError::HttpError(_) => StatusCode::BAD_GATEWAY,
            BeedleError::ConfigError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            BeedleError::InventoryError(_) => StatusCode::BAD_REQUEST,
            BeedleError::IOError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            BeedleError::ResponseError(_) => StatusCode::TOO_MANY_REQUESTS, // ????
            BeedleError::PoolError(_) => StatusCode::LOCKED, // ????
            //BeedleError::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}