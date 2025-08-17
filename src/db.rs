use crate::db::cache::initialize_caches;
use crate::errors::BeedleError;
use crate::models::{NewProduct};
use r2d2::{Pool, PooledConnection};
use diesel::{
    {QueryDsl,RunQueryDsl},
    r2d2::{self, ConnectionManager},
    pg::PgConnection,
};
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type Conn = PooledConnection<ConnectionManager<PgConnection>>;

pub mod cache;
pub mod products;
pub mod session;

pub fn establish_connection() -> Result<DbPool, BeedleError> {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
    let manager = ConnectionManager::<PgConnection>::new(&db_url);
    Pool::builder().build(manager).map_err(|e| BeedleError::DatabaseError(e.to_string()))
}

pub fn init_db(conn: &mut Conn) -> Result<(), BeedleError> {
    //seed_example_products(conn)?; // Moved to a db migration instead
    initialize_caches(conn)?;
    Ok(())
}

/*
// UNIT TESTING
#[cfg(test)]
mod tests {
use super::*;
use once_cell::sync::Lazy;
use std::env;

static POOL: Lazy<DbPool> = Lazy::new(|| {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
});

fn get_test_conn() -> Conn {
    POOL.get().expect("Failed to get a connection from the pool")
}

#[test]
fn test_count_products() {
    let mut conn = get_test_conn();
    let result = count_products(&mut conn);
    assert!(result.is_ok(), "Counting products failed");
    // Further checks could be made depending on the initial state of the database
}

#[test]
fn test_load_product_by_id_existing() {
    let mut conn = get_test_conn();
    // Assuming that there is a product with ID 1 in the test database
    let result = load_product_by_id(&mut conn, 1);
    assert!(result.is_ok(), "Loading existing product by ID should work");
    let product = result.unwrap();
    assert!(product.is_some(), "Product with ID 1 should exist");
    // Further validation of the product fields could be performed here
}

#[test]
fn test_load_product_by_id_non_existent() {
    let mut conn = get_test_conn();
    // Assuming that there is no product with this ID in the test database
    let result = load_product_by_id(&mut conn, -1);
    assert!(result.is_ok(), "Loading non-existing product should not cause an error");
    let product = result.unwrap();
    assert!(product.is_none(), "Product with an invalid ID should not exist");
}
    
}
*/