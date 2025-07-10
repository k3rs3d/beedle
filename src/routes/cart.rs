use actix_session::Session;
use actix_web::{web, HttpResponse, http::header};
use actix_csrf::extractor::{Csrf, CsrfToken, CsrfGuarded};
use tera::Tera;
use serde::Serialize;
use crate::errors::BeedleError;
use crate::config::Config;
use crate::db::{DbPool, load_products};
use crate::session::{get_cart, update_cart_quantity, create_base_context};

#[derive(serde::Deserialize)]
struct CartActionForm {
    product_id: i32,
    quantity: i32,
    csrf_token: CsrfToken,
}

impl CsrfGuarded for CartActionForm {
    fn csrf_token(&self) -> &CsrfToken { &self.csrf_token }
}

#[derive(Serialize)]
struct CartProductView {
    product: crate::models::Product,
    quantity: u32,
    max_quantity: i32,
}

async fn add_to_cart(
    pool: web::Data<DbPool>,
    session: Session,
    form: Csrf<web::Form<CartActionForm>>,
) -> Result<HttpResponse, BeedleError> {
    let mut conn = pool.get()?;
    let form = form.into_inner().into_inner();
    update_cart_quantity(&session, form.product_id, form.quantity, &mut conn)?;
    Ok(HttpResponse::SeeOther().append_header((header::LOCATION, "/products")).finish())
}

async fn remove_from_cart(
    pool: web::Data<DbPool>,
    session: Session,
    form: Csrf<web::Form<CartActionForm>>,
) -> Result<HttpResponse, BeedleError> {
    let mut conn = pool.get()?;
    let form = form.into_inner().into_inner();
    update_cart_quantity(&session, form.product_id, -1, &mut conn)?;
    Ok(HttpResponse::SeeOther().append_header((header::LOCATION, "/cart")).finish())
}

async fn update_cart_quantity_handler(
    pool: web::Data<DbPool>,
    session: Session,
    form: Csrf<web::Form<CartActionForm>>,
) -> Result<HttpResponse, BeedleError> {
    let mut conn = pool.get()?;
    let form = form.into_inner().into_inner();
    // If quantity == 0, remove from cart
    let delta = form.quantity;
    update_cart_quantity(&session, form.product_id, delta, &mut conn)?;
    Ok(HttpResponse::SeeOther().append_header((header::LOCATION, "/cart")).finish())
}

async fn view_cart(
    pool: web::Data<DbPool>,
    tera: web::Data<Tera>,
    config: web::Data<Config>,
    session: Session,
    csrf_token: CsrfToken,
) -> Result<HttpResponse, BeedleError> {
    let mut conn = pool.get()?;
    let cart = get_cart(&session);
    let products = load_products(&mut conn)?;

    let cart_items: Vec<CartProductView> = cart
        .into_iter()
        .filter_map(|item| {
            products.iter()
                .find(|p| p.id == item.product_id)
                .map(|p| {
                    let max_per_order = 99;
                    let max_quantity = p.inventory.min(max_per_order);
                    CartProductView {
                        product: p.clone(),
                        quantity: item.quantity,
                        max_quantity,
                    }
                })
        })
        .collect();

    let mut ctx = create_base_context(&session, config.get_ref());
    ctx.insert("cart_items", &cart_items);
    ctx.insert("csrf_token", &csrf_token.get());
    let rendered = tera.render("cart.html", &ctx)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::resource("/add_to_cart/").route(web::post().to(add_to_cart)))
        .service(web::resource("/remove_from_cart/").route(web::post().to(remove_from_cart)))
        .service(web::resource("/update_cart_quantity/").route(web::post().to(update_cart_quantity_handler)))
        .service(web::resource("/cart").route(web::get().to(view_cart)));
}