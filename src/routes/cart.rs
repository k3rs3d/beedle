use crate::config::Config;
use crate::db::{products, DbPool};
use crate::errors::BeedleError;
use crate::models::CartItem;
use crate::session::{create_base_context, ensure_session_cookie, SessionInfo};
use crate::views::ProductView;
use actix_csrf::extractor::{Csrf, CsrfGuarded, CsrfToken};
use actix_web::{http::header, web, HttpResponse};
use serde::Serialize;
use std::cmp::Ordering;
use tera::Tera;

#[derive(serde::Deserialize)]
struct CartActionForm {
    product_id: i32,
    quantity: i32,
    csrf_token: CsrfToken,
}

#[derive(serde::Deserialize)]
struct CartQuery {
    undo_id: Option<i32>,
    undo_qty: Option<u32>,
}

impl CsrfGuarded for CartActionForm {
    fn csrf_token(&self) -> &CsrfToken {
        &self.csrf_token
    }
}

#[derive(Serialize)]
struct CartProductView {
    product: ProductView,
    quantity: u32,
    max_quantity: i32,
}

fn update_cart_quantity(cart: &mut Vec<CartItem>, product_id: i32, delta: i32, max_allowed: i32) {
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
    let prev_qty = session
        .cart
        .iter()
        .find(|item| item.product_id == form.product_id)
        .map(|i| i.quantity);

    update_cart_quantity(
        &mut session.cart,
        form.product_id,
        form.quantity,
        max_allowed,
    );
    crate::db::session::update_session_cart(&mut conn, session.session_id, &session.cart)?;

    // If this was a remove (set to zero), redirect with undo params
    let location = if let Some(qty) = prev_qty {
        let found = session.cart.iter().any(|i| i.product_id == form.product_id);
        if !found && form.quantity <= 0 {
            if qty > 1 {
                format!("/cart?undo_id={}&undo_qty={}", form.product_id, qty)
            } else {
                format!("/cart?undo_id={}", form.product_id)
            }
        } else {
            "/cart".to_owned()
        }
    } else {
        "/cart".to_owned()
    };
    let resp = HttpResponse::SeeOther()
        .header(header::LOCATION, location)
        .finish();
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
    query: web::Query<CartQuery>,
) -> Result<HttpResponse, BeedleError> {
    let mut conn = pool.get()?;
    let cart = &session.cart;
    let products = products::load_products(&mut conn)?;

    let cart_items: Vec<CartProductView> = cart
        .iter()
        .filter_map(|item| {
            products.iter().find(|p| p.id == item.product_id).map(|p| {
                let max_per_order = 99; // HACK: arbitrary maximum
                let max_quantity = p.inventory.min(max_per_order);
                CartProductView {
                    product: ProductView::from(p),
                    quantity: item.quantity,
                    max_quantity,
                }
            })
        })
        .collect();

    let mut ctx = create_base_context(&session, config.get_ref());

    if let Some(undo_id) = query.undo_id {
        // TODO: lookup product info instead of just passing ID?
        let undo_qty = query.undo_qty.unwrap_or(1);
        ctx.insert("undo_id", &undo_id);
        ctx.insert("undo_qty", &undo_qty);

        // insert undo product name
        if let Some(product) = products.iter().find(|p| p.id == undo_id) {
            ctx.insert("undo_product_name", &product.name);
        }
    }

    ctx.insert("cart_items", &cart_items);
    ctx.insert("csrf_token", &csrf_token.get());
    let rendered = tera.render("cart.html", &ctx)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/update_cart_quantity/").route(web::post().to(update_cart_quantity_handler)),
    )
    .service(web::resource("/cart").route(web::get().to(view_cart)));
}
