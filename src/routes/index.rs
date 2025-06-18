use actix_web::{web, HttpResponse};
use actix_session::Session;
use tera::Tera;
use crate::config::Config;
use crate::db::{DbPool, cache};
use crate::errors::BeedleError;
use crate::session::create_base_context;

async fn index(
    tera: web::Data<Tera>,
    config: web::Data<Config>,
    session: Session
) -> Result<HttpResponse, BeedleError> { 
    // Load front page information
    let categories = cache::CategoriesCache::get_categories().to_vec();
    //let featured_product = load_featured_product(&conn)?; // TODO
    //let sales_events = load_sales_events(&conn)?; // TODO
    
    let mut ctx = create_base_context(&session, config.get_ref());
    ctx.insert("categories", &categories);
    //ctx.insert("featured_product", &featured_product);
    //ctx.insert("sales_events", &sales_events);

    let rendered = tera.render("index.html", &ctx)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(index)));
}
