use serde::Serialize;
use crate::models::Product;
use crate::price::Price;

#[derive(Serialize)]
pub struct ProductView {
    pub product_id: i32,
    pub name: String,
    pub price_original: Price,
    pub price_discounted: Price,
    pub price_original_formatted: String,
    pub price_discounted_formatted: String,
    pub is_on_sale: bool,
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
        let price_original = Price::from_cents(product.price);

        let price_discounted = if let Some(percent) = product.discount_percent {
            if percent > 0.0 {
                // Calculate discounted price
                let discount = (price_original.as_cents() as f64 * ((100.0 as f64 - percent as f64) / 100.0)).round() as i64;
                Price::from_cents(discount)
            } else {
                price_original
            }
        } else {
            price_original
        };

        ProductView {
            product_id: product.id,
            name: product.name.clone(),
            price_original,
            price_discounted,
            price_original_formatted: price_original.to_decimal_string(),
            price_discounted_formatted: price_discounted.to_decimal_string(),
            is_on_sale: (price_discounted < price_original),
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