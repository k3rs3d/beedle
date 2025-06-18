use actix_session::Session;
use crate::models::CartItem;

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
pub fn update_cart_quantity(session: &Session, product_id: u32, quantity: i32) {
    let mut cart = get_cart(session);
    // TODO: Don't allow more than Inventory amount
    // TODO: also set a limit per customer, like ceil(inventory, limit)
    if quantity > 0 {
        // Add to cart
        if let Some(item) = cart.iter_mut().find(|item| item.product_id == product_id) {
            item.quantity += quantity as u32; // TODO: fix quantity can exceed u32 max value here
        } else {
            cart.push(CartItem { product_id, quantity: quantity as u32 });
        }
    } else if quantity < 0 {
        // If the quantity specified is equal to -1, remove the item completely
        let quantity_abs = quantity.abs() as u32;
        if quantity_abs == 1 {
            cart.retain(|item| item.product_id != product_id);
        } else {
            // remove if it drops to zero or below
            if let Some(item) = cart.iter_mut().find(|item| item.product_id == product_id) {
                if item.quantity > quantity_abs {
                    item.quantity -= quantity_abs;
                } else {
                    // Equivalent to removing the item since its count would become non-positive
                    cart.retain(|item| item.product_id != product_id);
                }
            }
        }
    }

    session.insert("cart", &cart).unwrap();
}


// UNIT TESTING 

#[cfg(test)]
mod tests {
    use super::*;
    use actix_session::SessionExt;
    use actix_web::test;
    
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
}