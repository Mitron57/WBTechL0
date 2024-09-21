use std::error::Error;
use axum::async_trait;
use crate::domain::models::Order;

#[async_trait]
pub trait Database {
    async fn insert(&mut self, data: Order) -> Result<(), Box<dyn Error>>;
    async fn remove(&mut self, id: &str) -> Result<(), Box<dyn Error>>;
    async fn get(&self, id: &str) -> Result<Option<Order>, Box<dyn Error>>;
}