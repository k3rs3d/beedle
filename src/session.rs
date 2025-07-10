use actix_session::Session;
use crate::errors::BeedleError;
use crate::models::CartItem;
use crate::db::load_product_by_id;

// A starter tera context with generic elements added 
pub fn create_base_context(session:&Session, config: &crate::config::Config) -> tera::Context {
    let mut ctx = tera::Context::new();
    ctx.insert("site_name", &config.site_name);
    ctx.insert("root_domain", &config.root_domain);
    ctx.insert("cart_item_count", &get_cart_item_count(session));
    ctx
}
 
// Num items in cart
pub fn get_cart_item_count(session: &Session) -> u32 {
    get_cart(session).iter().map(|item| item.quantity).sum()
}

// Retrieve current cart from session
pub fn get_cart(session: &Session) -> Vec<CartItem> {
    match session.get::<Vec<CartItem>>("cart") {
        Ok(Some(cart)) => cart,
        _ => Vec::new(),
    }
}

// Increase/decrease quantity of item 
pub fn update_cart_quantity(
    session: &Session,
    product_id: i32,
    delta: i32,
    conn: &mut crate::db::Conn,
) -> Result<(), BeedleError> {
    let mut cart = get_cart(session);
    let product = load_product_by_id(conn, product_id)?.ok_or_else(|| {
        BeedleError::InventoryError(format!("Product {} not found", product_id))
    })?;

    let max_per_order = 99; // HACK (arbitrary global max)
    let max_allowed = product.inventory.min(max_per_order).min(99).max(1); // never allow more than the inventory amount OR per-customer limit 

    if delta == 0 {
        // Remove item if explicit set-to-zero
        cart.retain(|item| item.product_id != product_id);
    } else if delta > 0 {
        let entry = cart.iter_mut().find(|item| item.product_id == product_id);
        if let Some(item) = entry {
            let new_qty = (item.quantity as i32 + delta).clamp(1, max_allowed);
            item.quantity = new_qty as u32;
        } else {
            let start_qty = delta.clamp(1, max_allowed);
            cart.push(CartItem {
                product_id,
                quantity: start_qty as u32,
            });
        }
    } else {
        // delta < 0; Decrement or Remove
        if let Some(idx) = cart.iter().position(|item| item.product_id == product_id) {
            let item = &mut cart[idx];
            let new_qty = item.quantity as i32 + delta; // delta negative
            if new_qty < 1 {
                cart.remove(idx);
            } else {
                item.quantity = new_qty as u32;
            }
        }
    }
    session.insert("cart", &cart).unwrap();
    Ok(())
}


// UNIT TESTING 

#[cfg(test)]
mod tests {
    use super::*;
    use actix_session::SessionExt;
    use actix_web::test;

    // need database connection
    /*
    #[actix_rt::test]
    async fn test_add_to_cart() {
        let session = test::TestRequest::default().to_http_request();
        let session = session.get_session();
        let cart = get_cart(&session);
        assert_eq!(cart.len(), 0);
        update_cart_quantity(&session, 1, 1);
        let cart: Vec<CartItem> = get_cart(&session);
        assert_eq!(cart.len(), 1);
        assert_eq!(cart[0].product_id, 1);
        assert_eq!(cart[0].quantity, 1);
    }

    #[actix_rt::test]
    async fn test_remove_from_cart() {
        let session = test::TestRequest::default().to_http_request();
        let session = session.get_session();
        update_cart_quantity(&session, 1, 1);
        update_cart_quantity(&session, 2, 1);
        let cart = get_cart(&session);
        assert_eq!(cart.len(), 2);
        update_cart_quantity(&session, 1, -1);
        let cart = get_cart(&session);
        assert_eq!(cart.len(), 1);
        assert_eq!(cart[0].product_id, 2);
    }

    // Cart must be empty in a new session 
    #[actix_rt::test]
    async fn test_empty_cart_state() {
        let session = test::TestRequest::default().to_http_request();
        let session = session.get_session();
        let cart = get_cart(&session);
        assert!(cart.is_empty());
    }
    */
}