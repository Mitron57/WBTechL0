//Consider use Redis, but there I guess we can use HashMap shamelessly

use std::collections::HashMap;
use axum::Json;
use crate::domain::{models::Order, interfaces};

pub struct Cache {
    memory: HashMap<String, Json<Order>>,
}

impl Cache {
    pub fn new() -> Cache {
        Cache {
            memory: HashMap::new(),
        }
    }
}

impl interfaces::Cache for Cache {
    fn add(&mut self, order_id: String, order: Order) {
        self.memory.insert(order_id, Json::from(order));
    }
    
    fn get(&self, order_id: &str) -> Option<&Order> {
        Some(&self.memory.get(order_id)?.0)
    }
    
    fn remove(&mut self, order_id: &str) -> Option<Order> {
        Some(self.memory.remove(order_id)?.0)
    }
}