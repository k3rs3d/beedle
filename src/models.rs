use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: Option<u32>,
    pub name: String,
    pub price: f64,
    pub inventory: u32,
}

#[derive(Serialize, Deserialize)]
pub struct CartItem {
    pub product_id: u32,
    pub quantity: u32,
}