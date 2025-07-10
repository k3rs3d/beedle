use actix_session::Session;
use actix_web::{cookie::Cookie, HttpRequest, HttpResponse, web};
use futures_util::future::{BoxFuture, FutureExt};
use uuid::Uuid;
use crate::models::CartItem;
use crate::db::{DbPool};

#[derive(Clone)]
pub struct SessionInfo {
    pub session_id: Uuid,
    pub was_created: bool,
    pub user_id: Option<i32>, // TODO: user accounts
    pub cart: Vec<CartItem>,
    pub ip_address: String, 
    pub user_agent: String,
}

impl actix_web::FromRequest for SessionInfo {
    type Error = actix_web::Error;
    type Future = BoxFuture<'static, Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let pool = match req.app_data::<web::Data<DbPool>>() {
            Some(p) => p.clone(),
            None => req.app_data::<web::Data<DbPool>>().expect("DB pool missing in app_data!").clone(), // should never happen 
        };
        // Gather cookie/session_id etc
        let cookie_session_id = req.cookie("session_id").and_then(|c| Uuid::parse_str(c.value()).ok());
        let ip = req.peer_addr().map(|addr| addr.ip().to_string()).unwrap_or_default();
        let user_agent = req.headers().get("User-Agent")
            .and_then(|v| v.to_str().ok()).unwrap_or_default().to_owned();
        
        log::info!("connection_info host: {:?}", req.connection_info().host());
        log::info!("req.url: {:?}", req.uri());
        for cookie in req.cookies().iter() {
            log::info!("cookie: {:?}", cookie);
}
        
        async move {
            let mut conn = pool.get().map_err(|_| actix_web::error::ErrorInternalServerError("No DB connection"))?;
            let mut was_created = false;
            log::info!("Looking for session_id in cookie: {:?}", cookie_session_id);
            let (session_id, row) = if let Some(sid) = cookie_session_id {
                if let Some(row) = crate::db::session::find_session_by_id(&mut conn, sid)? {
                    log::info!("Cart of find_session_by_id: {:?}", row.cart_data);
                    (sid, row) // loaded from db; was_created remains false
                } else {
                    // Invalid or expired session: create new
                    log::info!("Session invalid/expired. Creating new session.");
                    let new_row = crate::db::session::create_new_session(&mut conn, &ip, &user_agent)?;
                    was_created = true;
                    (new_row.session_id, new_row)
                }
            } else {
                // No cookie: create new session
                log::info!("No cookie found. Creating new session.");
                let new_row = crate::db::session::create_new_session(&mut conn, &ip, &user_agent)?;
                was_created = true;
                (new_row.session_id, new_row)
            };
            // Parse cart_data
            let cart: Vec<crate::models::CartItem> = row.cart_data
                .as_ref()
                .and_then(|j| serde_json::from_value(j.clone()).ok())
                .unwrap_or_default();
            Ok(SessionInfo {
                session_id,
                was_created: was_created,
                user_id: row.user_id,
                cart,
                ip_address: ip,
                user_agent,
            })
        }.boxed()
    }
}

pub fn ensure_session_cookie(mut res: HttpResponse, sid: Uuid) -> HttpResponse {
    // .secure(true) for prod
    let cookie = Cookie::build("session_id", sid.to_string())
        .path("/")
        .http_only(true)
        .max_age(actix_web::cookie::time::Duration::days(7)) // If you want expiry
        .finish();
    res.add_cookie(&cookie).unwrap();
    res
}

// A starter tera context with generic elements added 
pub fn create_base_context(session:&SessionInfo, config: &crate::config::Config) -> tera::Context {
    let mut ctx = tera::Context::new();
    ctx.insert("site_name", &config.site_name);
    ctx.insert("root_domain", &config.root_domain);
    ctx.insert("cart_item_count", &get_cart_item_count(session));
    ctx
}
 
// Num items in cart (kind of obsolete)
pub fn get_cart_item_count(session: &SessionInfo) -> u32 {
    session.cart.iter().map(|item| item.quantity).sum()
}

// Retrieve current cart from session
// DEPRECATED 
pub fn get_cart(session: &Session) -> Vec<CartItem> {
    match session.get::<Vec<CartItem>>("cart") {
        Ok(Some(cart)) => cart,
        _ => Vec::new(),
    }
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