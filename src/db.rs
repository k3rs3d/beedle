use crate::db::cache::initialize_caches;
use crate::errors::BeedleError;
use crate::models::{CartItem, Product, NewProduct};
use bigdecimal::FromPrimitive;
use r2d2::{Pool, PooledConnection};
use diesel::{
    {ExpressionMethods,TextExpressionMethods,QueryDsl,RunQueryDsl},
    r2d2::{self, ConnectionManager},
    pg::PgConnection,
    prelude::*
};
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type Conn = PooledConnection<ConnectionManager<PgConnection>>;

pub mod cache;
pub mod products;
pub mod session;

// DEBUG
// Add fake example products to db
pub fn seed_example_products(conn: &mut Conn) -> Result<(), BeedleError> {
    use crate::schema::product::dsl::*;
    let existing: i64 = product.count().get_result(conn)
        .map_err(|e| BeedleError::DatabaseError(e.to_string()))?;
    if existing > 0 {
        log::info!("Products already exist, skipping seed.");
        return Ok(());
    }
    //let now = chrono::Utc::now().naive_utc();

let example_products = vec![
    NewProduct {
        name: "Red Apple".to_owned(), 
        price: bigdecimal::BigDecimal::from_f32(1.2).unwrap(),
        inventory: 100,
        category: "Produce".to_owned(),
        tags: Some("Fruit,Healthy".to_owned()),
        keywords: Some("apple,malus".to_owned()),
        thumbnail_url: Some("https://en.wikipedia.org/static/images/icons/wikipedia.png".to_owned()),
        gallery_urls: Some("https://commons.wikimedia.org/wiki/File:Box_of_Marbles.jpg".to_owned()),
        tagline: Some("A crisp, tasty red apple!".to_owned()),
        description: Some("Only the freshest...".to_owned()),
        discount_percent: Some(10.0),
        added_date: None,
        restock_date: None
    },
    NewProduct {
        name: "Green Apple".to_owned(), 
        price: bigdecimal::BigDecimal::from_f32(1.1).unwrap(),
        inventory: 130,
        category: "Produce".to_owned(),
        tags: Some("Fruit,Healthy".to_owned()),
        keywords: Some("red,apple,malus".to_owned()),
        thumbnail_url: Some("https://en.wikipedia.org/static/images/icons/wikipedia.png".to_owned()),
        gallery_urls: Some("https://commons.wikimedia.org/wiki/File:Box_of_Marbles.jpg".to_owned()),
        tagline: Some("A crisp, tangy green apple!".to_owned()),
        description: Some("Only the luigiest...".to_owned()),
        discount_percent: None,
        added_date: None,
        restock_date: None
    },
    NewProduct {
        name: "Coffee".to_owned(), 
        price: bigdecimal::BigDecimal::from_f32(7.2).unwrap(),
        inventory: 34,
        category: "Beverage".to_owned(),
        tags: Some("Caffeine".to_owned()),
        keywords: Some("brewed,hot".to_owned()),
        thumbnail_url: Some("https://en.wikipedia.org/static/images/icons/wikipedia.png".to_owned()),
        gallery_urls: Some("https://commons.wikimedia.org/wiki/File:Box_of_Marbles.jpg".to_owned()),
        tagline: Some("Burnt roast from elsewhere!".to_owned()),
        description: Some("Only the coffeeiest...".to_owned()),
        discount_percent: None,
        added_date: None,
        restock_date: None
    },
    NewProduct {
        name: "Tea".to_owned(), 
        price: bigdecimal::BigDecimal::from_f32(4.0).unwrap(),
        inventory: 50,
        category: "Beverage".to_owned(),
        tags: Some("Caffeine".to_owned()),
        keywords: Some("brewed,cold".to_owned()),
        thumbnail_url: Some("https://en.wikipedia.org/static/images/icons/wikipedia.png".to_owned()),
        gallery_urls: Some("https://commons.wikimedia.org/wiki/File:Box_of_Marbles.jpg".to_owned()),
        tagline: Some("Bagged!".to_owned()),
        description: Some("Mostly unspilled!".to_owned()),
        discount_percent: Some(5.0),
        added_date: None,
        restock_date: None
    },
    NewProduct {
        name: "Malk".to_owned(), 
        price: bigdecimal::BigDecimal::from_f32(1.1).unwrap(),
        inventory: 7,
        category: "Beverage".to_owned(),
        tags: Some("Dairy".to_owned()),
        keywords: Some("cold".to_owned()),
        thumbnail_url: Some("https://en.wikipedia.org/static/images/icons/wikipedia.png".to_owned()),
        gallery_urls: Some("https://commons.wikimedia.org/wiki/File:Box_of_Marbles.jpg".to_owned()),
        tagline: Some("Now with Vitamin R".to_owned()),
        description: Some("From the pastures of...".to_owned()),
        discount_percent: None,
        added_date: None,
        restock_date: None
    },
    NewProduct {
        name: "Kernberry Pie".to_owned(), 
        price: bigdecimal::BigDecimal::from_f32(123.79).unwrap(),
        inventory: 8,
        category: "Bakery".to_owned(),
        tags: Some("Pie".to_owned()),
        keywords: Some("kern,berry".to_owned()),
        thumbnail_url: Some("https://en.wikipedia.org/static/images/icons/wikipedia.png".to_owned()),
        gallery_urls: Some("https://commons.wikimedia.org/wiki/File:Box_of_Marbles.jpg".to_owned()),
        tagline: Some("For eating!".to_owned()),
        description: Some("Loaded with the juiciest Kernberries...".to_owned()),
        discount_percent: None,
        added_date: None,
        restock_date: None
    },
    NewProduct {
        name: "Rust Cookie".to_owned(), 
        price: bigdecimal::BigDecimal::from_f32(3.99).unwrap(),
        inventory: 50,
        category: "Bakery".to_owned(),
        tags: Some("Rust,Cookie".to_owned()),
        keywords: Some("rusty".to_owned()),
        thumbnail_url: Some("https://en.wikipedia.org/static/images/icons/wikipedia.png".to_owned()),
        gallery_urls: Some("https://commons.wikimedia.org/wiki/File:Box_of_Marbles.jpg".to_owned()),
        tagline: Some("Disgusting!".to_owned()),
        description: Some("Some people like it.".to_owned()),
        discount_percent: Some(25.0),
        added_date: None,
        restock_date: None
    },
];

for p in example_products {
    products::insert_product(conn, &p)?;
}
    log::info!("Seeded example products.");
    Ok(())
}



pub fn establish_connection() -> Result<DbPool, BeedleError> {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
    let manager = ConnectionManager::<PgConnection>::new(&db_url);
    Pool::builder().build(manager).map_err(|e| BeedleError::DatabaseError(e.to_string()))
}

pub fn init_db(conn: &mut Conn) -> Result<(), BeedleError> {
    seed_example_products(conn)?;
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