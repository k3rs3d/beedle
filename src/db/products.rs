use crate::errors::BeedleError;
use crate::models::{CartItem, Product, NewProduct};
use r2d2::PooledConnection;
use diesel::{
    {ExpressionMethods,TextExpressionMethods,QueryDsl,RunQueryDsl},
    r2d2::{self, ConnectionManager},
    pg::PgConnection,
    prelude::*
};
use super::Conn;

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