use diesel::{Queryable, Insertable};
use crate::schema::product;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub price: bigdecimal::BigDecimal,
    pub inventory: i32,
    pub category: String,
    pub tags: Option<String>,
    pub keywords: Option<String>,
    pub thumbnail_url: Option<String>,
    pub gallery_urls: Option<String>,
    pub tagline: Option<String>,
    pub description: Option<String>,
    pub discount_percent: Option<f32>,
}

#[derive(Insertable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = product)]
pub struct NewProduct {
    pub name: String,
    pub price: bigdecimal::BigDecimal,
    pub inventory: i32,
    pub category: String,
    pub tags: Option<String>,
    pub keywords: Option<String>,
    pub thumbnail_url: Option<String>,
    pub gallery_urls: Option<String>,
    pub tagline: Option<String>,
    pub description: Option<String>,
    pub discount_percent: Option<f32>,
}

#[derive(Serialize, Deserialize)]
pub struct CartItem {
    pub product_id: i32,
    pub quantity: u32,
}
