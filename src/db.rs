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
    },
];

for p in example_products {
    insert_product(conn, &p)?;
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


// PRODUCTS
// Total product count
/*
pub fn count_products(conn: &mut Conn) -> Result<usize, BeedleError> {
    use crate::schema::product::dsl::*;
    product.count()
        .get_result::<i64>(conn)
        .map(|n| n as usize)
        .map_err(|e| BeedleError::DatabaseError(e.to_string()))
}
*/

pub fn load_products(conn: &mut Conn) -> Result<Vec<Product>, BeedleError> {
    use crate::schema::product::dsl::*;
    product
        .order(id.asc())
        .load::<Product>(conn)
        .map_err(|e| BeedleError::DatabaseError(e.to_string()))
}

// Complex filter using Diesel query builder
pub fn filter_products(
    conn: &mut Conn,
    category_opt: Option<&str>,
    tag_opt: Option<&str>,
    search_opt: Option<&str>,
    sort_opt: Option<&str>,
    limit_opt: usize,
    offset_opt: usize,
) -> Result<Vec<Product>, BeedleError> {
    use crate::schema::product::dsl::*;
    let mut query = product.into_boxed();

    if let Some(cat) = category_opt {
        let cat = cat.trim();
        if !cat.is_empty() {
            query = query.filter(category.eq(cat));
        }
    }
    if let Some(tag_val) = tag_opt {
        // we search tags as a comma string right now, so like is the best without parsing
        query = query.filter(tags.like(format!("%{}%", tag_val)));
    }

    // Text search
    if let Some(search_str) = search_opt {
        let s = search_str.trim();
        if !s.is_empty() {
            let like_expr = format!("%{}%", search_str);
            query = query.filter(
                name.ilike(like_expr.clone()).or(description.ilike(like_expr.clone())).or(tagline.ilike(like_expr))
            );
        }
    }

    // Sorting
    match sort_opt {
        Some("alpha")       => query = query.order(name.asc()),
        Some("price_low") =>  query = query.order(price.asc()),
        Some("price_high") => query = query.order(price.desc()),
        _ =>                  query = query.order(price.desc()), // default
        //Some("newest")      => query = query.order(id.desc()),
        //Some("oldest")      => query = query.order(id.asc()),
        //Some("rating")      => query = query.order(rating.desc()),
    }
    query
        .limit(limit_opt as i64)
        .offset(offset_opt as i64)
        .load::<Product>(conn)
        .map_err(|e| BeedleError::DatabaseError(e.to_string()))
}

pub fn load_product_by_id(conn: &mut Conn, product_id_val: i32) -> Result<Option<Product>, BeedleError> {
    use crate::schema::product::dsl::*;
    product
        .filter(id.eq(product_id_val))
        .first::<Product>(conn)
        .optional()
        .map_err(|e| BeedleError::DatabaseError(e.to_string()))
}

pub fn save_product(conn: &mut Conn, product_in: &Product) -> Result<(), BeedleError> {
    use crate::schema::product::dsl::*;
    let updated_rows = diesel::update(product.filter(id.eq(product_in.id)))
        .set((
            name.eq(&product_in.name),
            price.eq(product_in.price.clone()),
            inventory.eq(product_in.inventory),
            category.eq(&product_in.category),
            tags.eq(&product_in.tags),
            keywords.eq(&product_in.keywords),
            thumbnail_url.eq(&product_in.thumbnail_url),
            gallery_urls.eq(&product_in.gallery_urls),
            tagline.eq(&product_in.tagline),
            description.eq(&product_in.description),
            discount_percent.eq(&product_in.discount_percent),
        ))
        .execute(conn)
        .map_err(|e| BeedleError::DatabaseError(e.to_string()))?;
    if updated_rows == 0 {
        Err(BeedleError::DatabaseError("No rows updated".into()))
    } else {
        Ok(())
    }
}

pub fn insert_product(conn: &mut Conn, new_product: &NewProduct) -> Result<Product, BeedleError> {
    use crate::schema::product::dsl::*;
    diesel::insert_into(product)
        .values(new_product)
        .get_result(conn)
        .map_err(|e| BeedleError::DatabaseError(e.to_string()))
}

pub fn delete_product(conn: &mut Conn, product_id_val: i32) -> Result<(), BeedleError> {
    use crate::schema::product::dsl::*;
    let affected = diesel::delete(product.filter(id.eq(product_id_val)))
        .execute(conn)
        .map_err(|e| BeedleError::DatabaseError(e.to_string()))?;
    if affected == 0 {
        Err(BeedleError::DatabaseError(format!("No product with id {}", product_id_val)))
    } else {
        Ok(())
    }
}

// Inventory update sample: (update stock after order)
// This function now uses Diesel transactions.
pub fn update_inventory(conn: &mut Conn, cart: &[CartItem]) -> Result<(), BeedleError> {
    use crate::schema::product::dsl::*;
    conn.transaction::<_, diesel::result::Error, _>(|conn| {
        for item in cart {
            // Get product for this ID
            let prod = product.filter(id.eq(item.product_id as i32)).first::<Product>(conn)?;
            if prod.inventory >= item.quantity as i32 {
                let new_inv = prod.inventory - item.quantity as i32;
                diesel::update(product.filter(id.eq(item.product_id as i32)))
                    .set(inventory.eq(new_inv))
                    .execute(conn)?;
            } else {
                return Err(diesel::result::Error::RollbackTransaction) // abort
            }
        }
        Ok(())
    }).map_err(|e| BeedleError::DatabaseError(e.to_string()))
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