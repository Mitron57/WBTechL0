use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Payment {
    pub transaction: String,
    pub request_id: String,
    pub currency: String,
    pub provider: String,
    pub amount: i32,
    pub payment_dt: i32,
    pub bank: String,
    pub delivery_cost: i32,
    pub goods_total: i32,
    pub custom_fee: i32,
}
