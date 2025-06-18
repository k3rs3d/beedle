use crate::errors::BeedleError;
use once_cell::sync::Lazy;
use std::sync::RwLock;

pub static DATA: Lazy<RwLock<Vec<String>>> = Lazy::new(|| RwLock::new(Vec::new()));
pub struct CategoriesCache;
// TODO: cache more future stuff (featured products, sales events, trending items...)

pub fn initialize_caches(conn: &crate::db::Conn) -> Result<(), crate::BeedleError> {
    match load_categories_from_db(conn) {
        Ok(categories) => {
            CategoriesCache::initialize(categories);
            Ok(())
        }
        Err(e) => Err(e),
    }
}

fn load_categories_from_db(conn: &crate::db::Conn) -> Result<Vec<String>, BeedleError> {
    let mut stmt = conn.prepare("SELECT DISTINCT category FROM product")?;
    let categories_iter = stmt.query_map([], |row| row.get(0))?;

    let categories = categories_iter
        .collect::<Result<_, rusqlite::Error>>()
        .map_err(BeedleError::DatabaseError)?;

    Ok(categories)
}

impl CategoriesCache {
    pub fn initialize(data: Vec<String>) {
        let mut cache = DATA.write().unwrap();
        *cache = data;
    }

    pub fn get_categories() -> std::sync::RwLockReadGuard<'static, Vec<String>> {
        DATA.read().unwrap()
    }

    pub fn invalidate() {
        let mut write_guard = DATA.write().unwrap();
        write_guard.clear();
    }
}

#[cfg(test)]
mod cache_tests {
    use super::*;
    use crate::db::{establish_connection, init_db, seed_example_products};

    #[test]
    fn test_initialization_of_cache() {
        let pool = establish_connection().expect("Failed to establish connection");
        let mut conn = pool.get().expect("Failed to get connection from pool");
        
        init_db(&pool).expect("Failed to initialize DB");
        seed_example_products(&mut conn).expect("Failed to seed DB with test data");
        initialize_caches(&conn).expect("Failed to cache from DB");

        // Ensure cache is initialized with correct data
        let cached_categories = CategoriesCache::get_categories();
        assert_eq!(cached_categories.len(), 3);
        assert!(cached_categories.contains(&"Produce".to_string()));
    }

    #[test]
    fn test_cache_invalidation() {
        let pool = establish_connection().expect("Failed to establish connection");
        let mut conn = pool.get().expect("Failed to get connection from pool");
        
        init_db(&pool).expect("Failed to initialize DB");
        seed_example_products(&mut conn).expect("Failed to seed DB with test data");
        initialize_caches(&conn).expect("Failed to cache from DB");

        CategoriesCache::invalidate();
        let cached_categories = CategoriesCache::get_categories();
        assert!(cached_categories.is_empty());
    }
}