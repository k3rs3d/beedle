use serde::Serialize;
use crate::models::Product;

#[derive(Serialize)]
pub struct ProductView {
    pub product_id: i32,
    pub name: String,
    pub price: f64,
    pub category: String,
    pub tags: Vec<String>,
    pub gallery_urls: Vec<String>,
    pub tagline: Option<String>,
    pub discount_percent: Option<f32>,
    pub thumbnail_url: Option<String>,
    pub description: Option<String>,
    pub date_added: Option<String>,
    pub date_restock_expected: Option<String>,
}

impl From<&Product> for ProductView {
    fn from(product: &Product) -> Self {
        ProductView {
            product_id: product.id,
            name: product.name.clone(),
            price: bigdecimal::ToPrimitive::to_f64(&product.price).unwrap_or_default(),
            category: product.category.clone(),
            tags: product.tags.as_ref()
                .map(|s| s.split(',')
                     .map(|s| s.trim().to_owned())
                     .filter(|s| !s.is_empty())
                     .collect())
                .unwrap_or_default(),
            gallery_urls: product.gallery_urls.as_ref()
                .map(|s| s.split(',')
                     .map(|s| s.trim().to_owned())
                     .filter(|s| !s.is_empty())
                     .collect())
                .unwrap_or_default(),
            tagline: product.tagline.clone(),
            discount_percent: product.discount_percent,
            thumbnail_url: product.thumbnail_url.clone(),
            description: product.description.clone(),
            // Format to RFC3339....could also just pass as raw chrono::NaiveDateTime
            date_added: Some(product.added_date.format("%Y-%m-%d %H:%M:%S").to_string()),
            date_restock_expected: product.restock_date.map(|d| d.format("%Y-%m-%d %H:%M:%S").to_string()),
        }
    }
}