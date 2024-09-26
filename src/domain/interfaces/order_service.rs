use crate::domain::models::Order;
use axum::async_trait;
use std::error::Error;
use crate::domain::interfaces;


type Repository = dyn interfaces::Repository<Error=Box<dyn Error>>;

#[async_trait]
pub trait OrderService: Sync + Send {
    async fn add_order(
        &self,
        repository: &Repository,
        order: Order,
    ) -> Result<(), Box<dyn Error>>;

    async fn get_order(
        &self,
        order_uid: &str,
        repository: &Repository,
    ) -> Result<Option<Order>, Box<dyn Error>>;
}
