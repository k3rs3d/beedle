use crate::errors::*;
use crate::db::{Conn, DbPool};
use crate::models::{SessionRow, CartItem};
use uuid::Uuid;
use chrono::{Utc, Duration, NaiveDateTime};
use diesel::prelude::*;

pub fn find_session_by_id(conn: &mut Conn, sid: Uuid) -> Result<Option<SessionRow>, BeedleError> {
    use crate::schema::session::dsl::*;
    //let now = Utc::now().naive_utc();
    session
        .filter(session_id.eq(sid)) // <- THIS was the bug
        //.filter(expires_at.ge(now)) // HACK: Fix expiration later
        .first::<SessionRow>(conn)
        .optional()
        .map_err(Into::into)
}

pub fn create_new_session(
    conn: &mut Conn,
    ip: &str,
    user_agent_str: &str,
) -> Result<SessionRow, BeedleError> {
    use crate::schema::session::dsl::*;
    let sid = Uuid::new_v4();
    let now = Utc::now().naive_utc();
    let exp = now + Duration::days(7); // expire 7 days for anon users
    let new_session = SessionRow {
        session_id: sid,
        user_id: None,
        created_at: now,
        updated_at: now,
        expires_at: exp,
        ip_address: Some(ip.to_owned()),
        user_agent: Some(user_agent_str.to_owned()),
        cart_data: Some(serde_json::json!([])), // empty cart
    };
    let inserted = diesel::insert_into(session).values(&new_session).execute(conn)?;
    log::info!("Inserted session row: {:?}, count={}", sid, inserted);
    Ok(new_session)
}

pub fn update_session_cart(
    conn: &mut Conn,
    session_id_val: Uuid,
    cart: &[CartItem],
) -> Result<(), BeedleError> {
    use crate::schema::session::dsl::*;
    let cart_json = match serde_json::to_value(cart) {
        Ok(v) => v,
        Err(e) => {
            log::error!("Failed to serialize cart for session {}: {:?}", session_id_val, e);
            return Err(BeedleError::DatabaseError(format!("Cart serialization error: {}", e)));
        }
    };
    let now = Utc::now().naive_utc();

    log::info!("Updating session cart for session_id: {session_id_val}, cart: {cart_json}");

    let n = diesel::update(session.filter(session_id.eq(session_id_val)))
        .set((cart_data.eq(cart_json), updated_at.eq(now)))
        .execute(conn)?;

    log::info!("Session update: {} rows affected for session_id: {session_id_val}", n);

    if n == 0 {
        return Err(BeedleError::DatabaseError(format!(
            "No session row found for session_id: {session_id_val}"
        )));
    }
    Ok(())
}

// TODO: associate an existing session with a user ID 