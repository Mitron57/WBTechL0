use wb_tech_l0::models::Order;
use wb_tech_l0::interfaces::{OrderService, Repository};
use std::error::Error;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use axum::async_trait;
use wb_tech_l0::infrastructure;

#[derive(Default)]
pub struct MockRepository {
    orders: Arc<RwLock<HashMap<String, Order>>>,
}

#[async_trait]
impl Repository for MockRepository {
    async fn insert(&mut self, order: Order) -> Result<(), Box<dyn Error>> {
        let mut wlock = self.orders.write().await;
        if wlock.contains_key(&order.order_uid) {
            return Err("already exists".into());
        }
        wlock.insert(order.order_uid.to_string(), order);
        Ok(())
    }

    async fn remove(&mut self, id: &str) -> Result<(), Box<dyn Error>> {
        self.orders.write().await.remove(id);
        Ok(())
    }

    async fn get(&self, id: &str) -> Result<Option<Order>, Box<dyn Error>> {
        Ok(self.orders.read().await.get(id).cloned())
    }

    async fn get_and_cache(&mut self, id: &str) -> Result<Option<Order>, Box<dyn Error>> {
        self.get(id).await
    }
}

#[tokio::test]
async fn add_order() {
    let mut mock_repo: Box<dyn Repository + Send + Sync> = Box::new(MockRepository::default());
    let order_service = infrastructure::OrderService;

    let order = Order {
        order_uid: "order1".to_string(),
        track_number: "TRACK123".to_string(),
        entry: "WBIL".to_string(),
        delivery: Default::default(),
        payment: Default::default(),
        items: vec![],
        locale: "en".to_string(),
        internal_signature: "signature".to_string(),
        customer_id: "customer1".to_string(),
        delivery_service: "service".to_string(),
        shardkey: "1".to_string(),
        sm_id: 1,
        date_created: "2023-10-01T12:00:00Z".to_string(),
        oof_shard: "1".to_string(),
    };
    let result = order_service.add_order(&mut mock_repo, order.clone()).await;
    assert!(result.is_ok());
    let result = order_service.add_order(&mut mock_repo, order).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_order() {
    let mut mock_repo: Box<dyn Repository + Send + Sync> = Box::new(MockRepository::default());
    let order_service = infrastructure::OrderService;

    let order = Order {
        order_uid: "order1".to_string(),
        track_number: "TRACK123".to_string(),
        entry: "WBIL".to_string(),
        delivery: Default::default(),
        payment: Default::default(),
        items: vec![],
        locale: "en".to_string(),
        internal_signature: "signature".to_string(),
        customer_id: "customer1".to_string(),
        delivery_service: "service".to_string(),
        shardkey: "1".to_string(),
        sm_id: 1,
        date_created: "2023-10-01T12:00:00Z".to_string(),
        oof_shard: "1".to_string(),
    };
    mock_repo.insert(order.clone()).await.unwrap();
    let result = order_service.get_order("order1", &mut mock_repo).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().unwrap().order_uid, "order1");
    let result = order_service.get_order("non_existent_order", &mut mock_repo).await;
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}