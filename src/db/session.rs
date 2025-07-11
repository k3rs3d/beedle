//! Database access for web sessions via the `session` table.
//! CRUD for session rows and shopping cart storage.

use crate::errors::*;
use crate::db::Conn;
use crate::models::{SessionRow, CartItem};
use uuid::Uuid;
use chrono::{Utc, Duration};
use diesel::prelude::*;

/// Look up a session row by session_id (UUID).
/// Returns Ok(None) if not found.
/// TODO: expiry filtering
pub fn find_session_by_id(conn: &mut Conn, sid: Uuid) -> Result<Option<SessionRow>, BeedleError> {
    use crate::schema::session::dsl::*;

    let result = session
        .filter(session_id.eq(sid))
        // .filter(expires_at.ge(Utc::now().naive_utc()))
        .first::<SessionRow>(conn)
        .optional();

    match result {
        Ok(opt_row) => {
            if let Some(ref row) = opt_row {
                log::debug!("Found session row: {}", row.session_id);
            } else {
                log::debug!("No session found for session_id={}", sid);
            }
            Ok(opt_row)
        }
        Err(e) => {
            log::error!("DB error while finding session {}: {e}", sid);
            Err(BeedleError::DatabaseError(format!("Find session error: {e}")))
        }
    }
}

/// Insert a new session row with given IP and user-agent. Returns the full SessionRow.
/// New sessions are created as anonymous (user_id=None), empty cart, 1-week expiry.
pub fn create_new_session(
    conn: &mut Conn,
    ip: &str,
    user_agent_str: &str,
) -> Result<SessionRow, BeedleError> {
    use crate::schema::session::dsl::*;
    let sid = Uuid::new_v4();
    let now = Utc::now().naive_utc();
    let exp = now + Duration::days(7); 
    let new_session = SessionRow {
        session_id: sid,
        user_id: None,
        created_at: now,
        updated_at: now,
        expires_at: exp,
        ip_address: Some(ip.to_owned()),
        user_agent: Some(user_agent_str.to_owned()),
        cart_data: Some(serde_json::json!([])), // Empty cart as default
    };
    let inserted_count = diesel::insert_into(session)
        .values(&new_session)
        .execute(conn)
        .map_err(|e| {
            log::error!("Failed to insert new session: {e}");
            BeedleError::DatabaseError(format!("Session insert error: {e}"))
        })?;

    log::info!("Created new session {} for ip {}, inserted {} row(s)",
        sid, ip, inserted_count);

    Ok(new_session)
}

/// Update the cart JSON for a session by ID. 
/// Also updates the updated_at timestamp.
pub fn update_session_cart(
    conn: &mut Conn,
    session_id_val: Uuid,
    cart: &[CartItem],
) -> Result<(), BeedleError> {
    use crate::schema::session::dsl::*;

    let cart_json = serde_json::to_value(cart).map_err(|e| {
        log::error!("Cart serialization failed for session {}: {e}", session_id_val);
        BeedleError::SessionError(format!("Cart serialization error: {e}"))
    })?;

    let now = Utc::now().naive_utc();

    log::info!("Updating session {} with new cart ({} items)", session_id_val, cart.len());

    let rows_updated = diesel::update(session.filter(session_id.eq(session_id_val)))
        .set((cart_data.eq(cart_json), updated_at.eq(now)))
        .execute(conn)
        .map_err(|e| {
            log::error!("DB error on cart update for session {}: {e}", session_id_val);
            BeedleError::DatabaseError(format!("Session DB error: {e}"))
        })?;

    if rows_updated == 0 {
        log::warn!("Failed to update cart for session_id {}", session_id_val);
        return Err(BeedleError::DatabaseError(format!(
            "Possibly missing session row for session_id: {}", session_id_val
        )));
    }

    Ok(())
}


// TODO: associate an existing session with a user ID 
// pub fn set_session_user_id(...)