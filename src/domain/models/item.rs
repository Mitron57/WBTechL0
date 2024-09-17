use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Item {
    pub chrt_id: i32,
    pub track_number: String,
    pub price: i32,
    pub rid: String,
    pub name: String,
    pub sale: i32,
    pub size: String,
    pub total_price: i32,
    pub nm_id: i32,
    pub brand: String,
    pub status: i32,
}