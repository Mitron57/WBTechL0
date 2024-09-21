//Consider use Redis, but there I guess we can use HashMap shamelessly

use std::collections::HashMap;
use tokio::sync::RwLock;
use axum::{async_trait, Json};
use crate::domain::{models::Order, interfaces};

pub struct Cache {
    memory: RwLock<HashMap<String, Json<Order>>>,
}

impl Cache {
    pub fn new() -> Cache {
        Cache {
            memory: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl interfaces::Cache for Cache {
    async fn add(&mut self, order_id: String, order: Order) {
        self.memory.write().await.insert(order_id, Json::from(order));
    }
    
    async fn get(&self, order_id: &str) -> Option<Order> {
        Some(self.memory.read().await.get(order_id)?.0.clone())
    }
    
    async fn remove(&mut self, order_id: &str) -> Option<Order> {
        Some(self.memory.write().await.remove(order_id)?.0)
    }
}