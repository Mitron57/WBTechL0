use crate::domain::models::Order;
use axum::async_trait;

#[async_trait]
pub trait Repository: Send + Sync {
    type Error;
    
    async fn insert(&self, order: Order) -> Result<(), Self::Error>;

    async fn remove(&self, id: &str) -> Result<(), Self::Error>;
    
    async fn get(&self, id: &str) -> Result<Option<Order>, Self::Error>;
    
    async fn get_and_cache(&self, id: &str) -> Result<Option<Order>, Self::Error>;
}
