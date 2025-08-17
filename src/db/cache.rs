use crate::errors::BeedleError;
use diesel::{QueryDsl,RunQueryDsl};
use once_cell::sync::Lazy;
use std::sync::RwLock;

pub static DATA: Lazy<RwLock<Vec<String>>> = Lazy::new(|| RwLock::new(Vec::new()));
pub struct CategoriesCache;

pub fn initialize_caches(conn: &mut crate::db::Conn) -> Result<(), BeedleError> {
    match load_categories_from_db(conn) {
        Ok(categories) => {
            CategoriesCache::initialize(categories);
            Ok(())
        }
        Err(e) => Err(e),
    }
}

/// Load all unique categories (strings) from DB via Diesel
fn load_categories_from_db(conn: &mut crate::db::Conn) -> Result<Vec<String>, BeedleError> {
    use crate::schema::product::dsl::*;
    // select distinct category from product;
    product
        .select(category)
        .distinct()
        .load::<String>(conn)
        .map_err(|e| BeedleError::DatabaseError(e.to_string()))
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
    use crate::db::{Conn, DbPool, establish_connection, init_db};
    use diesel::{PgConnection, r2d2::ConnectionManager};
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
    fn test_initialization_of_cache() {
        let mut conn = get_test_conn();

        init_db(&mut conn).expect("Failed to initialize DB");
        //seed_example_products(&mut conn).expect("Failed to seed DB with test data");
        // removed this function; now depends on db to be pre seeded with test data
        initialize_caches(&mut conn).expect("Failed to cache from DB");

        // Ensure cache is initialized with correct data
        let cached_categories = CategoriesCache::get_categories();
        assert!(!cached_categories.is_empty());
        assert!(cached_categories.contains(&"Produce".to_string()));
    }

    #[test]
    fn test_cache_invalidation() {
        let mut conn = get_test_conn();

        init_db(&mut conn).expect("Failed to initialize DB");
        //seed_example_products(&mut conn).expect("Failed to seed DB with test data");
        // removed this function; now depends on db to be pre seeded with test data
        initialize_caches(&mut conn).expect("Failed to cache from DB");

        CategoriesCache::invalidate();
        let cached_categories = CategoriesCache::get_categories();
        assert!(cached_categories.is_empty());
    }
}