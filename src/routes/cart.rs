use actix_web::{web, HttpResponse, http::header};
use actix_csrf::extractor::{Csrf, CsrfToken, CsrfGuarded};
use tera::Tera;
use serde::Serialize;
use std::cmp::Ordering;
use crate::config::Config;
use crate::db::{DbPool, products};
use crate::errors::BeedleError;
use crate::models::CartItem;
use crate::session::{ensure_session_cookie, create_base_context, SessionInfo};

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

fn update_cart_quantity(
    cart: &mut Vec<CartItem>,
    product_id: i32,
    delta: i32,
    max_allowed: i32,
) {
    match delta.cmp(&0) {
        Ordering::Equal => {
            // Remove from cart if delta is zero
            cart.retain(|item| item.product_id != product_id);
        }
        Ordering::Greater => {
            // Increase quantity / insert item
            match cart.iter_mut().find(|item| item.product_id == product_id) {
                Some(item) => {
                    let new_qty = (item.quantity as i32 + delta).clamp(1, max_allowed);
                    item.quantity = new_qty as u32;
                }
                None => {
                    let start_qty = delta.clamp(1, max_allowed);
                    cart.push(CartItem {
                        product_id,
                        quantity: start_qty as u32,
                    });
                }
            }
        }
        Ordering::Less => {
            // Decrease quantity 
            if let Some(idx) = cart.iter().position(|item| item.product_id == product_id) {
                let item = &mut cart[idx];
                let new_qty = item.quantity as i32 + delta; // (delta is negative)
                if new_qty < 1 {
                    cart.remove(idx); // remove item
                } else {
                    item.quantity = new_qty as u32;
                }
            }
        }
    }
}



async fn update_cart_quantity_handler(
    pool: web::Data<DbPool>,
    mut session: SessionInfo,
    form: Csrf<web::Form<CartActionForm>>,
) -> Result<HttpResponse, BeedleError> {
    let mut conn = pool.get()?;
    let form = form.into_inner().into_inner();
    
    // verify product exists and get actual allowable max
    let product = products::load_product_by_id(&mut conn, form.product_id)?
        .ok_or_else(|| BeedleError::InventoryError("Product not found".into()))?;

    let max_per_order = 99; // TODO: use product.max_per_order after I add that field 
    let max_allowed = product.inventory.min(max_per_order);

    update_cart_quantity(&mut session.cart, form.product_id, form.quantity, max_allowed);

    crate::db::session::update_session_cart(&mut conn, session.session_id, &session.cart)?;

    let resp = HttpResponse::SeeOther().header(header::LOCATION, "/cart").finish();
    if session.was_created {
        Ok(ensure_session_cookie(resp, session.session_id))
    } else {
        Ok(resp)
    }
}

async fn view_cart(
    pool: web::Data<DbPool>,
    tera: web::Data<Tera>,
    config: web::Data<Config>,
    session: SessionInfo,
    csrf_token: CsrfToken,
) -> Result<HttpResponse, BeedleError> {
    let mut conn = pool.get()?;
    let cart = &session.cart;
    let products = products::load_products(&mut conn)?;

    let cart_items: Vec<CartProductView> = cart
        .iter()
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
        .service(web::resource("/update_cart_quantity/").route(web::post().to(update_cart_quantity_handler)))
        .service(web::resource("/cart").route(web::get().to(view_cart)));
}