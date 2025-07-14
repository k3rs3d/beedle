//! Product database helpers: loading, CRUD, inventory adjustment, etc.

use crate::errors::BeedleError;
use crate::models::{CartItem, NewProduct, Product};
use crate::schema::product::dsl::*;
use diesel::{
    prelude::*,
    {ExpressionMethods, QueryDsl, RunQueryDsl, TextExpressionMethods},
};

use super::Conn;

/// Load all products, ordered by ID ascending.
pub fn load_products(conn: &mut Conn) -> Result<Vec<Product>, BeedleError> {
    product.order(id.asc()).load::<Product>(conn).map_err(|e| {
        log::error!("Loading all products failed: {e}");
        BeedleError::DatabaseError(e.to_string())
    })
}

/// Filter and page products by category/tag/search/sort.
/// Accepts optional filters and paginates with limit/offset.
///
/// # Parameters
/// * `category_opt` - Optional category filter
/// * `tag_opt` - Optional tag filter
/// * `search_opt` - Optional substring/full-text search
/// * `sort_opt` - Optional sort order ("alpha", "price_low", etc)
/// * `limit_opt`, `offset_opt` - Pagination controls
pub fn filter_products(
    conn: &mut Conn,
    category_opt: Option<&str>,
    tag_opt: Option<&str>,
    search_opt: Option<&str>,
    sort_opt: Option<&str>,
    limit_opt: usize,
    offset_opt: usize,
) -> Result<Vec<Product>, BeedleError> {
    let mut query = product.into_boxed();

    if let Some(cat) = category_opt.filter(|c| !c.trim().is_empty()) {
        query = query.filter(category.eq(cat));
    }

    if let Some(tag_val) = tag_opt.filter(|t| !t.trim().is_empty()) {
        query = query.filter(tags.like(format!("%{}%", tag_val)));
    }

    // Text search 
    let like_expr = search_opt
        .filter(|s| !s.trim().is_empty())
        .map(|search_str| format!("%{}%", search_str));
    if let Some(ref like_expr) = like_expr {
        query = query.filter(
            name.ilike(like_expr.clone())
                .or(description.ilike(like_expr.clone()))
                .or(tagline.ilike(like_expr.clone()))
        );
    }

    // Sorting
    query = match sort_opt {
        Some("alpha") => query.order(name.asc()),
        Some("price_low") => query.order(price.asc()),
        Some("price_high") => query.order(price.desc()),
        _ => query.order(price.asc()), // default sort, TODO: make default sort configurable?
    };

    query
        .limit(limit_opt as i64)
        .offset(offset_opt as i64)
        .load::<Product>(conn)
        .map_err(|e| {
            log::error!("Filtering products failed: {e}");
            BeedleError::DatabaseError(e.to_string())
        })
}

/// Counts total number of products matching the given filters.
pub fn count_filtered_products(
    conn: &mut Conn,
    category_opt: Option<&str>,
    tag_opt: Option<&str>,
    search_opt: Option<&str>,
) -> Result<i64, BeedleError> {
    use crate::schema::product::dsl::*;
    let mut query = product.into_boxed();

    if let Some(cat) = category_opt.filter(|s| !s.trim().is_empty()) {
        query = query.filter(category.eq(cat));
    }
    if let Some(tag_val) = tag_opt.filter(|s| !s.trim().is_empty()) {
        query = query.filter(tags.like(format!("%{}%", tag_val)));
    }
    if let Some(search_str) = search_opt.filter(|s| !s.trim().is_empty()) {
        let like_expr = format!("%{}%", search_str);
        query = query.filter(
            name.ilike(like_expr.clone())
                .or(description.ilike(like_expr.clone()))
                .or(tagline.ilike(like_expr.clone())),
        );
    }

    query.count().get_result(conn).map_err(|e| {
        log::error!("Product count with filter failed: {}", e);
        BeedleError::DatabaseError(e.to_string())
    })
}

/// Find a product by its ID. Returns Ok(None) if not found.
pub fn load_product_by_id(conn: &mut Conn, product_id_val: i32) -> Result<Option<Product>, BeedleError> {
    product
        .filter(id.eq(product_id_val))
        .first::<Product>(conn)
        .optional()
        .map_err(|e| {
            log::error!("Loading product id {} failed: {e}", product_id_val);
            BeedleError::DatabaseError(e.to_string())
        })
}

/// Update/save an existing product. Returns error if product ID not found or update fails.
/// Used for admin/product-edit (not needed for cart/browse).
pub fn save_product(conn: &mut Conn, product_in: &Product) -> Result<(), BeedleError> {
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
        .map_err(|e| {
            log::error!("Failed to update product {}: {e}", product_in.id);
            BeedleError::DatabaseError(e.to_string())
        })?;
    if updated_rows == 0 {
        Err(BeedleError::DatabaseError("No product rows updated (id not found)".to_string()))
    } else {
        log::info!("Product {} updated", product_in.id);
        Ok(())
    }
}

/// Create a new product and returns it.
pub fn insert_product(conn: &mut Conn, new_product: &NewProduct) -> Result<Product, BeedleError> {
    diesel::insert_into(product)
        .values(new_product)
        .get_result(conn)
        .map_err(|e| {
            log::error!("Insert product failed: {e}");
            BeedleError::DatabaseError(e.to_string())
        })
}

/// Remove a product by ID.
/// Returns error if the product does not exist.
pub fn delete_product(conn: &mut Conn, product_id_val: i32) -> Result<(), BeedleError> {
    use crate::schema::product::dsl::*;
    let affected = diesel::delete(product.filter(id.eq(product_id_val)))
        .execute(conn)
        .map_err(|e| {
            log::error!("Delete failed for product {}: {e}", product_id_val);
            BeedleError::DatabaseError(e.to_string())
        })?;
    if affected == 0 {
        Err(BeedleError::DatabaseError(format!("No product with id {}", product_id_val)))
    } else {
        log::info!("Deleted product id {}", product_id_val);
        Ok(())
    }
}

/// Atomically decrement inventory for all cart items. Rolls back if any product would go negative inventory.
pub fn update_inventory(conn: &mut Conn, cart: &[CartItem]) -> Result<(), BeedleError> {
    use crate::schema::product::dsl::*;
    conn.transaction::<_, diesel::result::Error, _>(|conn| {
        for item in cart {
            let prod = product.filter(id.eq(item.product_id)).first::<Product>(conn)?;
            if prod.inventory >= item.quantity as i32 {
                let new_inv = prod.inventory - item.quantity as i32;
                diesel::update(product.filter(id.eq(item.product_id)))
                    .set(inventory.eq(new_inv))
                    .execute(conn)?;
            } else {
                log::warn!("Attempted to purchase more than inventory for product id {}: wanted {}, in stock {}", 
                    item.product_id, item.quantity, prod.inventory);
                // Abort transaction!
                return Err(diesel::result::Error::RollbackTransaction);
            }
        }
        Ok(())
    })
    .map_err(|e| {
        log::error!("Inventory update failed (rollback): {e}");
        BeedleError::DatabaseError(e.to_string())
    })
}