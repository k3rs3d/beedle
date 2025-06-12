use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: Option<u32>,
    pub name: String,
    pub price: f64,
    pub inventory: u32,
    pub category: String, // Singular
    pub tags: Vec<String>, // up to 5?
    pub keywords: Vec<String>, // For internal use
    pub thumbnail_url: Option<String>, // Thumbnail image URL
    pub gallery_urls: Vec<String>, // 0–3 gallery image URLs
    pub tagline: String, // Short desc
    pub description: Option<String>, // Longer HTML/text
    pub discount_percent: Option<f64>, // [0.0, 100.0]
}

#[derive(Serialize, Deserialize)]
pub struct CartItem {
    pub product_id: u32,
    pub quantity: u32,
}
