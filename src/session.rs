//! HTTP session extraction and session cookie logic using a
//! backend database session via `db::sessions`.  
//! Provides `SessionInfo` type. 

use actix_session::Session;
use actix_web::{cookie::Cookie, HttpRequest, HttpResponse, web};
use futures_util::future::{BoxFuture, FutureExt};
use uuid::Uuid;
use crate::models::CartItem;
use crate::db::{DbPool, session::*};

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

    /// Try to extract session info from the HTTP request.
    /// - Looks for a "session_id" cookie.
    /// - Loads the session from DB if found/valid.
    /// - Otherwise, creates a new session row in DB and sets a flag.
    fn from_request(req: &HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        // Get DB pool (should always be present)
        let pool = req
            .app_data::<web::Data<DbPool>>()
            .expect("DB pool missing in app_data!")
            .clone();

        // Extract relevant info for session association
        let cookie_session_id = req.cookie("session_id").and_then(|c| Uuid::parse_str(c.value()).ok());
        let ip = req.peer_addr().map(|addr| addr.ip().to_string()).unwrap_or_default();
        let user_agent = req.headers()
            .get("User-Agent")
            .and_then(|v| v.to_str().ok())
            .unwrap_or_default()
            .to_owned();

        // Log basic info
        let cookie_names: Vec<_> = match req.cookies().as_ref() {
            Ok(cookies) => cookies.iter().map(|c| c.name().to_string()).collect(),
            Err(_) => vec![],
        };
        log::debug!("Request at {:?} from {:?}, cookies: {:?}", req.uri(), ip, cookie_names);

        async move {
            let mut conn = pool.get()
                .map_err(|_| actix_web::error::ErrorInternalServerError("No DB connection"))?;
            let mut was_created = false;

            // try existing session, or create if missing/expired.
            let (session_id, row) = match cookie_session_id {
                Some(sid) => match find_session_by_id(&mut conn, sid) {
                    Ok(Some(row)) => {
                        log::debug!("Found active session in DB for {:?}", sid);
                        (sid, row)
                    }
                    Ok(None) | Err(_) => {
                        log::info!("Session {:?} not found/expired/bad. Making new session.", sid);
                        let row = create_new_session(&mut conn, &ip, &user_agent)
                            .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Session create failed: {e}")))?;
                        was_created = true;
                        (row.session_id, row)
                    }
                },
                None => {
                    log::info!("No session_id cookie. Creating new session for ip={}", ip);
                    let row = create_new_session(&mut conn, &ip, &user_agent)
                        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("Session create failed: {e}")))?;
                    was_created = true;
                    (row.session_id, row)
                }
            };
            // parse cart from JSON (can never panic)
            let cart: Vec<CartItem> = row.cart_data
                .as_ref()
                .and_then(|j| serde_json::from_value(j.clone()).ok())
                .unwrap_or_default();

            Ok(SessionInfo {
                session_id,
                was_created,
                user_id: row.user_id,
                cart,
                ip_address: ip,
                user_agent,
            })
        }
        .boxed()
    }
}


/// Sets session_id cookie for client on outgoing response
pub fn ensure_session_cookie(mut res: HttpResponse, sid: Uuid) -> HttpResponse {
    // .secure(true) should be enabled in production (HTTPS).
    let cookie = Cookie::build("session_id", sid.to_string())
        .path("/")
        .http_only(true)
        .max_age(actix_web::cookie::time::Duration::days(7))
        // .secure(true) // prod only
        .finish();

    if let Err(e) = res.add_cookie(&cookie) {
        log::error!("Adding session_id cookie failed: {e}");
    }
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