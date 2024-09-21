use crate::domain::interfaces;
use crate::domain::interfaces::Repository;
use crate::domain::models::Order;
use axum::async_trait;
use log::{log, Level};
use std::error::Error;

pub struct OrderService;

#[async_trait]
impl interfaces::OrderService for OrderService {
    async fn add_order(
        &self,
        repository: &mut Box<dyn Repository + Send + Sync>,
        order: Order,
    ) -> Result<(), Box<dyn Error>> {
        let order_uid = order.order_uid.clone();
        let result = repository.insert(order).await;
        if let Err(err) = result {
            log!(target: "add_order_service", Level::Error, "Insertion failed: err: {err}");
            Err(err)
        } else {
            log!(target: "add_order_service", Level::Info, "Order with order_uid: {order_uid} successfully added");
            Ok(())
        }
    }

    async fn get_order(
        &self,
        order_uid: &str,
        repository: &mut Box<dyn Repository + Send + Sync>,
    ) -> Result<Option<Order>, Box<dyn Error>> {
        let result = repository.get_and_cache(order_uid).await;
        match result {
            Ok(Some(order)) => {
                log!(target: "get_order_service", Level::Info, "Found order with order_uid: {order_uid}");
                Ok(Some(order))
            }
            Ok(None) => {
                log!(target: "get_order_service", Level::Info, "No order with order_uid: {order_uid}");
                Ok(None)
            }
            Err(err) => {
                log!(target: "get_order_service", Level::Error, "Failed to get order with order_uid: {order_uid}, error: {err}");
                Err(err)
            }
        }
    }
}
