use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: Option<i32>,
    pub name: String,
    pub price: f64,
    pub inventory: i32,
}

pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS product (
            id              INTEGER PRIMARY KEY,
            name            TEXT NOT NULL,
            price           REAL NOT NULL,
            inventory       INTEGER NOT NULL
        )",
        [],
    )?;
    Ok(())
}