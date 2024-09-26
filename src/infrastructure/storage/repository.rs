use crate::domain::interfaces;
use crate::domain::interfaces::{Cache, Database};
use crate::domain::models::Order;
use std::error::Error;
use axum::async_trait;
use log::{log, Level};

pub struct Repository<C, D> {
    cache: C,
    database: D,
}

impl<C, D> Repository<C, D>
where
    C: Cache,
    D: Database,
{
    pub fn new(cache: C, database: D) -> Self {
        Self {
            cache,
            database,
        }
    }
}

#[async_trait]
impl<C, D> interfaces::Repository for Repository<C, D>
where
    D: Database<Error = Box<dyn Error>>,
    C: Cache,
    
{
    
    type Error = Box<dyn Error>;
    async fn insert(&self, order: Order) -> Result<(), Self::Error> {
        self.database.insert(order.clone()).await
    }

    async fn remove(&self, id: &str) -> Result<(), Self::Error> {
        self.cache.remove(id).await;
        self.database.remove(id).await
    }

    async fn get(&self, id: &str) -> Result<Option<Order>, Self::Error> {
        if let Some(order) = self.cache.get(id).await {
            log!(target: "repository", Level::Info, "Order with uid: {id} found in cache");
            return Ok(Some(order.clone()));
        }
        self.database.get(id).await
    }

    async fn get_and_cache(&self, id: &str) -> Result<Option<Order>, Self::Error> {
        let found = self.get(id).await?;
        match found {
            Some(order) => {
                if self.cache.get(id).await.is_none() {
                    self.cache.add(id.to_string(), order.clone()).await;
                    return Ok(Some(order));
                }
                Ok(Some(order))
            }
            None => {
                Ok(None)
            }
        }
    }
}
