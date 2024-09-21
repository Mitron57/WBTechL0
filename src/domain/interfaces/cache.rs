use crate::domain::models::Order;
use axum::async_trait;

#[async_trait]
pub trait Cache {
    async fn add(&mut self, order_id: String, order: Order);
    
    async fn get(&self, key: &str) -> Option<Order>;

    async fn remove(&mut self, order_id: &str) -> Option<Order>;
}