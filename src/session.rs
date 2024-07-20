use actix_session::Session;
use crate::models::CartItem;

// Retrieve current cart from session
pub fn get_cart(session: &Session) -> Vec<CartItem> {
    match session.get::<Vec<CartItem>>("cart") {
        Ok(Some(cart)) => cart,
        _ => Vec::new(),
    }
}

pub fn add_to_cart(session: &Session, product_id: u32) {
    let mut cart = get_cart(session);
    if let Some(item) = cart.iter_mut().find(|item| item.product_id == product_id) {
        item.quantity += 1;
    } else {
        cart.push(CartItem { product_id, quantity: 1 });
    }
    session.insert("cart", &cart).unwrap();
}

pub fn remove_from_cart(session: &Session, product_id: u32) {
    let mut cart = get_cart(session);
    cart.retain(|item| item.product_id != product_id);
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
        add_to_cart(&session, 1);
        let cart = get_cart(&session);
        assert_eq!(cart.len(), 1);
        assert_eq!(cart[0].product_id, 1);
        assert_eq!(cart[0].quantity, 1);
    }

    #[actix_rt::test]
    async fn test_remove_from_cart() {
        let session = test::TestRequest::default().to_http_request();
        let session = session.get_session();
        add_to_cart(&session, 1);
        add_to_cart(&session, 2);
        let cart = get_cart(&session);
        assert_eq!(cart.len(), 2);
        remove_from_cart(&session, 1);
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

    // Customer shouldn't be able to remove nonexistent item 
    #[actix_rt::test]
    async fn test_not_updating_nonexistent_cart_item() {
        let session = test::TestRequest::default().to_http_request();
        let session = session.get_session();

        remove_from_cart(&session, 1);
        let cart = get_cart(&session);
        assert!(cart.is_empty());
    }
}