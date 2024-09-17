use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Delivery {
    pub name: String,
    pub phone: String,
    pub zip: String,
    pub address: String,
    pub region: String,
    pub email: String,
}