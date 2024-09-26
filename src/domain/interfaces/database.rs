use axum::async_trait;
use crate::domain::models::Order;

#[async_trait]
pub trait Database: Sync + Send {
    type Error;
    
    async fn insert(&self, data: Order) -> Result<(), Self::Error>;
    
    async fn remove(&self, id: &str) -> Result<(), Self::Error>;
    
    async fn get(&self, id: &str) -> Result<Option<Order>, Self::Error>;
}