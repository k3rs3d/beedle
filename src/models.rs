use diesel::{AsChangeset, Queryable, Insertable};
use crate::schema::*;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Product {
    pub id: i32,
    pub name: String,
    pub inventory: i32,
    pub category: String,
    pub tags: Option<String>,
    pub keywords: Option<String>,
    pub thumbnail_url: Option<String>,
    pub gallery_urls: Option<String>,
    pub tagline: Option<String>,
    pub description: Option<String>,
    pub discount_percent: Option<f32>,
    pub added_date: chrono::NaiveDateTime,
    pub restock_date: Option<chrono::NaiveDateTime>,
    pub price: i64,
}

#[derive(Insertable, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = product)]
pub(crate) struct NewProduct {
    pub name: String,
    pub price: i64,
    pub inventory: i32,
    pub category: String,
    pub tags: Option<String>,
    pub keywords: Option<String>,
    pub thumbnail_url: Option<String>,
    pub gallery_urls: Option<String>,
    pub tagline: Option<String>,
    pub description: Option<String>,
    pub discount_percent: Option<f32>,
    pub added_date: Option<chrono::NaiveDateTime>, 
    pub restock_date: Option<chrono::NaiveDateTime>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct CartItem {
    pub product_id: i32,
    pub quantity: u32,
}

#[derive(Queryable, Insertable, AsChangeset)]
#[diesel(table_name = session)]
pub(crate) struct SessionRow {
    pub session_id: uuid::Uuid,
    pub user_id: Option<i32>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub expires_at: chrono::NaiveDateTime,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub cart_data: Option<serde_json::Value>,
}