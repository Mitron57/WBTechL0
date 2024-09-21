use std::error::Error;
use crate::domain::models::Order;
use axum::async_trait;

#[async_trait]
pub trait Repository: Send + Sync {
    async fn insert(&mut self, order: Order) -> Result<(), Box<dyn Error>>;

    async fn remove(&mut self, id: &str) -> Result<(), Box<dyn Error>>;
    
    async fn get(&self, id: &str) -> Result<Option<Order>, Box<dyn Error>>;
    
    async fn get_and_cache(&mut self, id: &str) -> Result<Option<Order>, Box<dyn Error>>;
}