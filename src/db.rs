use crate::errors::BeedleError;
use crate::models::{CartItem, Product};
use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;

pub type DbPool = Pool<SqliteConnectionManager>;
type Conn = PooledConnection<SqliteConnectionManager>;

// DEBUG
// Add fake example products to db
pub fn seed_example_products(conn: &mut Conn) -> Result<(), BeedleError> {
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM product", [], |row| row.get(0))
    .unwrap_or(0);
    if count > 0 {
        log::info!("Products already exist, skipping seed.");
        return Ok(());
    }

    let example_products = vec![
        Product { id: None, name: "Red Apple".to_string(), price: 1.50, inventory: 100 },
        Product { id: None, name: "Blueberry Muffin".to_string(), price: 2.50, inventory: 40 },
        Product { id: None, name: "Coffee".to_string(), price: 3.00, inventory: 30 },
        Product { id: None, name: "Orange Juice".to_string(), price: 2.80, inventory: 25 },
    ];

    for p in example_products {
        save_product(conn, &p)?;
    }
    log::info!("Seeded example products.");
    Ok(())
}



pub fn establish_connection() -> Result<DbPool, BeedleError> {
    let manager = SqliteConnectionManager::file("inventory.db");
    Pool::builder().build(manager).map_err(|e| {
        log::error!("Database connection pool initialization error: {:?}", e);
        BeedleError::DatabaseError(rusqlite::Error::QueryReturnedNoRows) // Should be a "no connection" error
    })
}

pub fn init_db(pool: &DbPool) -> Result<(), BeedleError> {
    let conn = pool.get()?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS product (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            price REAL NOT NULL,
            inventory INTEGER NOT NULL
        )",
        [],
    )?;
    Ok(())
}

// Total product count
pub fn count_products(conn: &Conn) -> Result<usize, BeedleError> {
    let count: usize = conn.query_row(
    "SELECT COUNT(*) FROM product", [], |row| row.get(0)
    )?;
    Ok(count)
}

pub fn load_products(conn: &Conn) -> Result<Vec<Product>, BeedleError> {
    let mut stmt = conn.prepare("SELECT id, name, price, inventory FROM product")?;
    let product_iter = stmt.query_map([], |row| {
        Ok(Product {
            id: row.get(0)?,
            name: row.get(1)?,
            price: row.get(2)?,
            inventory: row.get(3)?,
        })
    })?;
    let mut products = Vec::new();
    for product in product_iter {
        products.push(product?);
    }
    Ok(products)
}

// only load a limited subset,
// to be used later for filtering products
pub fn load_products_limited(conn: &Conn, limit: usize, offset: usize) -> Result<Vec<Product>, BeedleError> {
    let mut stmt = conn.prepare("SELECT id, name, price, inventory FROM product LIMIT ? OFFSET ?")?;
    let product_iter = stmt.query_map(params![limit as i64, offset as i64], |row| {
        Ok(Product {
            id: row.get(0)?,
           name: row.get(1)?,
           price: row.get(2)?,
           inventory: row.get(3)?,
        })
    })?;
    let mut products = Vec::new();
    for product in product_iter {
        products.push(product?);
    }
    Ok(products)
}

pub fn save_product(conn: &mut Conn, product: &Product) -> Result<(), BeedleError> {
    let result = match product.id {
        Some(id) => {
            // If id is present, we are updating an existing product
            conn.execute(
                "UPDATE product SET name = ?1, price = ?2, inventory = ?3 WHERE id = ?4",
                params![product.name, product.price, product.inventory, id],
            )
        }
        None => {
            // If id is None, we are inserting a new product
            conn.execute(
                "INSERT INTO product (name, price, inventory) VALUES (?1, ?2, ?3)",
                params![product.name, product.price, product.inventory],
            )
        }
    };

    match result {
        Ok(_) => Ok(()),
        Err(e) => {
            log::error!("Failed to execute save product SQL: {:?}", e);
            Err(BeedleError::DatabaseError(e))
        }
    }
}

pub fn update_inventory(conn: &mut Conn, cart: &Vec<CartItem>) -> Result<(), BeedleError> {
    let transaction = conn.transaction()?;

    for cart_item in cart {
        let product_result = transaction.query_row(
            "SELECT id, name, price, inventory FROM product WHERE id = ?1",
            params![cart_item.product_id],
            |row| {
                Ok(Product {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    price: row.get(2)?,
                    inventory: row.get(3)?,
                })
            },
        );
        if let Ok(mut product) = product_result {
            if product.inventory >= cart_item.quantity {
                product.inventory -= cart_item.quantity;
                transaction.execute(
                    "UPDATE product SET inventory = ?1 WHERE id = ?2",
                    params![product.inventory, product.id],
                )?;
            } else {
                return Err(BeedleError::InventoryError(
                    "Not enough inventory".to_string(),
                ));
            }
        } else {
            return Err(BeedleError::InventoryError("Product not found".to_string()));
        }
    }

    transaction.commit()?;
    Ok(())
}

pub fn delete_product(conn: &mut Conn, product_id: i32) -> Result<(), BeedleError> {
    let result = conn.execute("DELETE FROM product WHERE id = ?1", params![product_id]);

    match result {
        Ok(rows_affected) => {
            if rows_affected == 0 {
                log::error!(
                    "No rows affected when trying to delete product with ID: {:?}",
                    product_id
                );
                return Err(BeedleError::DatabaseError(
                    rusqlite::Error::QueryReturnedNoRows,
                ));
            } else {
                Ok(())
            }
        }
        Err(e) => {
            log::error!("Failed to execute delete product SQL: {:?}", e);
            Err(BeedleError::DatabaseError(e))
        }
    }
}
