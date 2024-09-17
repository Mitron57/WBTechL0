use std::error::Error;
use crate::domain::models::Order;

pub trait Database {
    async fn insert(&mut self, data: Order) -> Result<(), Box<dyn Error>>;
    async fn remove(&mut self, id: &str) -> Result<(), Box<dyn Error>>;
    async fn get(&self, id: &str) -> Option<Order>;
}