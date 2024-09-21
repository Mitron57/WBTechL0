use crate::domain::models::Order;
use axum::async_trait;
use crate::domain::interfaces::Repository;

#[async_trait]
pub trait OrderService {
    async fn add_order(
        &self,
        repository: &mut Box<dyn Repository + Send + Sync>,
        order: Order,
    ) -> Result<(), Box<dyn std::error::Error>>;

    async fn get_order(
        &self,
        order_uid: &str,
        repository: &mut Box<dyn Repository + Send + Sync>,
    ) -> Result<Option<Order>, Box<dyn std::error::Error>>;
}
