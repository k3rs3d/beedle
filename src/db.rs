use crate::errors::BeedleError;
use crate::models::{CartItem, Product};
use chrono::{NaiveDateTime};
use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{Row,params,OptionalExtension};

pub type DbPool = Pool<SqliteConnectionManager>;
type Conn = PooledConnection<SqliteConnectionManager>;

const CREATE_PRODUCT_TABLE: &str = "CREATE TABLE IF NOT EXISTS product (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    price REAL NOT NULL,
    inventory INTEGER NOT NULL,
    category TEXT NOT NULL,
    tags TEXT,
    keywords TEXT,
    thumbnail_url TEXT,
    gallery_urls TEXT,
    tagline TEXT,
    description TEXT,
    discount_percent REAL
)";

const SELECT_PRODUCTS_BASE: &str = "SELECT 
    id, name, price, inventory, category, tags, keywords,
    thumbnail_url, gallery_urls, tagline, description, discount_percent
FROM product";

fn parse_product(row: &Row<'_>) -> Result<Product, rusqlite::Error> {
    Ok(Product {
        id: row.get(0)?,
        name: row.get(1)?,
        price: row.get(2)?,
        inventory: row.get(3)?,
        category: row.get(4)?,
        tags: row.get::<_, String>(5)?
            .split(',')
            .map(|s| s.trim().to_owned())
            .filter(|s| !s.is_empty())
            .collect(),
        keywords: row.get::<_, String>(6)?
            .split(',')
            .map(|s| s.trim().to_owned())
            .filter(|s| !s.is_empty())
            .collect(),
        thumbnail_url: row.get(7)?,
        gallery_urls: row.get::<_, String>(8)?
            .split(',')
            .map(|s| s.trim().to_owned())
            .filter(|s| !s.is_empty())
            .collect(),
        tagline: row.get(9)?,
        description: row.get(10)?,
        discount_percent: row.get(11)?,
    })
}

// DEBUG
// Add fake example products to db
pub fn seed_example_products(conn: &mut Conn) -> Result<(), BeedleError> {
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM product", [], |row| row.get(0))
    .unwrap_or(0);
    if count > 0 {
        log::info!("Products already exist, skipping seed.");
        return Ok(());
    }

    //let now = chrono::Utc::now().naive_utc();

    let example_products = vec![
        Product {
            id: None,
            name: "Red Apple".to_string(),
            price: 1.50,
            inventory: 100,
            category: "Produce".to_string(),
            tags: vec!["Fruit".to_string(), "Healthy".to_string()],
            keywords: vec!["apple".to_string(), "malus".to_string()],
            thumbnail_url: Some("https://en.wikipedia.org/static/images/icons/wikipedia.png".to_string()),
            gallery_urls: vec![
                "https://commons.wikimedia.org/wiki/File:Box_of_Marbles.jpg".to_string(),
                "https://commons.wikimedia.org/wiki/File:Box_of_Marbles.jpg".to_string()
            ],
            tagline: "A crisp, tasty red apple!".to_string(),
            description: Some("Only the freshest apples, direct from the farm. Perfect for snacks or baking.".to_string()),
            discount_percent: Some(10.0),
        },
        Product {
            id: None,
            name: "Granny Smith Apple".to_string(),
            price: 1.25,
            inventory: 130,
            category: "Produce".to_string(),
            tags: vec!["Fruit".to_string(), "Healthy".to_string()],
            keywords: vec!["apple".to_string(), "malus".to_string()],
            thumbnail_url: Some("https://en.wikipedia.org/static/images/icons/wikipedia.png".to_string()),
            gallery_urls: vec![
                "https://commons.wikimedia.org/wiki/File:Box_of_Marbles.jpg".to_string(),
                "https://commons.wikimedia.org/wiki/File:Box_of_Marbles.jpg".to_string()
            ],
            tagline: "A crisp, sour green apple!".to_string(),
            description: Some("Only the freshest apples, direct from the farm. Perfect for snacks.".to_string()),
            discount_percent: None,
        },
        Product {
            id: None,
            name: "Blueberry Muffin".to_string(),
            price: 2.50,
            inventory: 40,
            category: "Bakery".to_string(),
            tags: vec!["Dessert".to_string(), "Baked".to_string()],
            keywords: vec!["blueberry".to_string(), "muffin".to_string(), "cake".to_string()],
            thumbnail_url: Some("https://en.wikipedia.org/static/images/icons/wikipedia.png".to_string()),
            gallery_urls: vec![],
            tagline: "A delicious blueberry muffin.".to_string(),
            description: Some("Moist, fluffy muffin loaded with juicy blueberries.".to_string()),
            discount_percent: None,
        },
        Product {
            id: None,
            name: "Kernberry Pie".to_string(),
            price: 123.00,
            inventory: 2,
            category: "Bakery".to_string(),
            tags: vec!["Dessert".to_string(), "Baked".to_string()],
            keywords: vec!["pie".to_string(), "berry".to_string(), "cake".to_string()],
            thumbnail_url: Some("https://en.wikipedia.org/static/images/icons/wikipedia.png".to_string()),
            gallery_urls: vec![],
            tagline: "A delicious Kernberry pie.".to_string(),
            description: Some("Flaky crust, loaded with juicy kernberries.".to_string()),
            discount_percent: None,
        },
        Product {
            id: None,
            name: "Coffee".to_string(),
            price: 3.00,
            inventory: 30,
            category: "Beverage".to_string(),
            tags: vec!["Drink".to_string(), "Caffeinated".to_string()],
            keywords: vec!["coffee".to_string(), "arabica".to_string()],
            thumbnail_url: Some("https://en.wikipedia.org/static/images/icons/wikipedia.png".to_string()),
            gallery_urls: vec![
                "https://commons.wikimedia.org/wiki/File:Box_of_Marbles.jpg".to_string(),
            ],
            tagline: "Freshly brewed hot coffee".to_string(),
            description: Some("Sourced from premium beans.".to_string()),
            discount_percent: Some(20.0),
        },
        Product {
            id: None,
            name: "Orange Juice".to_string(),
            price: 2.80,
            inventory: 25,
            category: "Beverage".to_string(),
            tags: vec!["Drink".to_string(), "Citrus".to_string()],
            keywords: vec!["orange".to_string(), "juice".to_string(), "citrus".to_string()],
            thumbnail_url: Some("https://en.wikipedia.org/static/images/icons/wikipedia.png".to_string()),
            gallery_urls: vec![],
            tagline: "Freshly squeezed orange juice".to_string(),
            description: Some("Packed with vitamin C.".to_string()),
            discount_percent: None,
        },
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
    conn.execute(CREATE_PRODUCT_TABLE, [])?;
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
    let mut stmt = conn.prepare(SELECT_PRODUCTS_BASE)?;
    let products = stmt.query_map([], parse_product)?;
    products.collect::<Result<_, rusqlite::Error>>().map_err(BeedleError::DatabaseError)
}

pub fn filter_products(
    conn: &Conn,
    category: Option<&str>,
    tag: Option<&str>,
    search: Option<&str>,
    sort: Option<&str>,
    limit: usize,
    offset: usize,
) -> Result<Vec<Product>, BeedleError> {
    let mut sql = format!("{} WHERE 1=1", SELECT_PRODUCTS_BASE);
    let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();
    let mut backing_strings: Vec<String> = Vec::new(); // To hold any values we format on the fly
    // Backing for numbers
    let limit_i64 = limit as i64;
    let offset_i64 = offset as i64;

    if let Some(cat) = category {
        sql += " AND category = ?";
        backing_strings.push(cat.to_string());
    }
    if let Some(tag) = tag {
        sql += " AND tags LIKE ?";
        backing_strings.push(format!("%{}%", tag));
    }
    if let Some(search) = search {
        sql += " AND (name LIKE ? OR tagline LIKE ?)";
        backing_strings.push(format!("%{}%", search));
        backing_strings.push(format!("%{}%", search));
    }

    if let Some(sort_col) = sort {
        match sort_col {
            //"newest" => sql += " ORDER BY date_added DESC",
            //"oldest" => sql += " ORDER BY date_added ASC",
            "price_low" => sql += " ORDER BY price ASC",
            "price_high" => sql += " ORDER BY price DESC",
            _ => {},
        }
    } else {
        //sql += " ORDER BY date_added DESC";
        sql += " ORDER BY price DESC";
    }
    sql += " LIMIT ? OFFSET ?";

    // Now push refs in same order as strings above
    let mut idx = 0;
    if category.is_some() {
        params.push(&backing_strings[idx]);
        idx += 1;
    }
    if tag.is_some() {
        params.push(&backing_strings[idx]);
        idx += 1;
    }
    if search.is_some() {
        params.push(&backing_strings[idx]);
        params.push(&backing_strings[idx+1]);
        idx += 2;
    }
    params.push(&limit_i64);
    params.push(&offset_i64);

    let mut stmt = conn.prepare(&sql)?;
    let product_iter = stmt.query_map(&params[..], parse_product)?;
    
    product_iter.collect::<Result<_, rusqlite::Error>>().map_err(BeedleError::DatabaseError)
}

pub fn load_product_by_id(conn: &Conn, product_id: i32) -> Result<Option<Product>, BeedleError> {
    let mut stmt = conn.prepare(&format!("{} WHERE id = ?", SELECT_PRODUCTS_BASE))?;
    stmt.query_row(params![product_id], parse_product).optional().map_err(BeedleError::DatabaseError)
}

pub fn save_product(conn: &mut Conn, product: &Product) -> Result<(), BeedleError> {
    let result = match product.id {
        Some(id) => {
            conn.execute(
                "UPDATE product SET
                name = ?1,
                price = ?2,
                inventory = ?3,
                category = ?4,
                tags = ?5,
                keywords = ?6,
                thumbnail_url = ?7,
                gallery_urls = ?8,
                tagline = ?9,
                description = ?10,
                discount_percent = ?11
                WHERE id = ?12",
                params![
                    &product.name,
                    &product.price,
                    &product.inventory,
                    &product.category,
                    &product.tags.join(","),
                         &product.keywords.join(","),
                         product.thumbnail_url.as_deref(),
                         &product.gallery_urls.join(","),
                         &product.tagline,
                         product.description.as_deref(),
                         product.discount_percent,
                         id
                ],
            )
        }
        None => {
            conn.execute(
                "INSERT INTO product
                (name, price, inventory, category, tags, keywords, thumbnail_url, gallery_urls, tagline, description, discount_percent)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
                         params![
                             &product.name,
                         &product.price,
                         &product.inventory,
                         &product.category,
                         &product.tags.join(","),
                         &product.keywords.join(","),
                         product.thumbnail_url.as_deref(),
                         &product.gallery_urls.join(","),
                         &product.tagline,
                         product.description.as_deref(),
                         product.discount_percent
                         ],
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

pub fn update_inventory(conn: &mut Conn, cart: &[CartItem]) -> Result<(), BeedleError> {
    let transaction = conn.transaction()?;

    for cart_item in cart {
        let product_result = transaction
            .query_row(&format!("{} WHERE id = ?", SELECT_PRODUCTS_BASE), params![cart_item.product_id], parse_product);

        if let Ok(mut product) = product_result {
            if product.inventory >= cart_item.quantity {
                product.inventory -= cart_item.quantity;
                transaction.execute(
                    "UPDATE product SET inventory = ? WHERE id = ?",
                    params![product.inventory, product.id],
                )?;
            } else {
                return Err(BeedleError::InventoryError("Not enough inventory".to_string()));
            }
        } else {
            return Err(BeedleError::InventoryError("Product not found".to_string()));
        }
    }

    transaction.commit().map_err(BeedleError::DatabaseError)
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



#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Product;
    use r2d2_sqlite::SqliteConnectionManager;

    fn setup_test_db() -> DbPool {
        let manager = SqliteConnectionManager::memory();
        let pool: DbPool = Pool::builder().max_size(1).build(manager).expect("Failed to create pool.");
        
        let conn = pool.get().expect("Failed to get connection");
        conn.execute(CREATE_PRODUCT_TABLE, []).unwrap();

        pool
    }

    #[test]
    fn test_parse_product() {
        let pool = setup_test_db();
        let conn = pool.get().expect("Failed to get connection");
        let product_row = conn.prepare(
            "INSERT INTO product 
            (name, price, inventory, category, tags, keywords, thumbnail_url, gallery_urls, tagline, description, discount_percent)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        ).and_then(|mut stmt| {
            stmt.insert(params![
                "Test Product",
                9.99,
                10,
                "Something",
                "test,example",
                "anything,nothing",
                Some("http://example.com/thumbnail.jpg"),
                "http://example.com/gallery1.jpg,http://example.com/gallery2.jpg",
                "An amazing Test Product!",
                Some("The best Test Product you will ever see."),
                0.0
            ])
        }).expect("Inserting test product");

        let result = conn.query_row(
            "SELECT id, name, price, inventory, category, tags, keywords, thumbnail_url, gallery_urls, tagline, description, discount_percent FROM product WHERE rowid = ?1", 
            params![product_row], 
            parse_product
        );
        assert!(result.is_ok());
        let product = result.unwrap();
        assert_eq!(product.name, "Test Product");
        assert_eq!(product.price, 9.99);
        assert_eq!(product.inventory, 10);
    }

    #[test]
    fn test_seed_example_products() {
        let pool = setup_test_db();
        let mut conn = pool.get().expect("Failed to get connection");
        let result = seed_example_products(&mut conn);
        assert!(result.is_ok());

        let count = count_products(&conn).expect("Counting products");
        assert!(count > 0);
    }

    #[test]
    fn test_count_products() {
        let pool = setup_test_db();
        let conn = pool.get().expect("Failed to get connection");
        // Assuming the database is empty before we count
        let count = count_products(&conn).expect("Counting products");
        assert_eq!(count, 0);
    }

    #[test]
    fn test_load_products() {
        let pool = setup_test_db();
        let mut conn = pool.get().expect("Failed to get connection");
        seed_example_products(&mut conn).expect("Seeding products");
        let products = load_products(&conn).expect("Loading products");
        assert!(!products.is_empty());
    }

    #[test]
    fn test_save_product_insert() {
        let pool = setup_test_db();
        let mut conn = pool.get().expect("Failed to get connection");

        let new_product = Product {
            id: None,
            name: "New Product".to_string(),
            price: 29.99,
            inventory: 15,
            category: "Misc".to_string(),
            tags: Vec::new(),
            keywords: Vec::new(),
            thumbnail_url: None,
            gallery_urls: Vec::new(),
            tagline: "A brand new product".to_string(),
            description: None,
            discount_percent: None,
        };

        let result = save_product(&mut conn, &new_product);
        assert!(result.is_ok());

        let count = count_products(&conn).unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_delete_product() {
        let pool = setup_test_db();
        let mut conn = pool.get().expect("Failed to get connection");
        seed_example_products(&mut conn).expect("Seeding products");
        // Assuming we know the ID of product to delete is 1 for testing purpose
        let result = delete_product(&mut conn, 1);
        assert!(result.is_ok());

        let count = count_products(&conn).unwrap();
        // One less than the seeded number (if 6 are seeded)
        assert_eq!(count, 5); 
    }
}