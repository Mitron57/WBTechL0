use crate::domain::interfaces::{Cache, Database};
use tokio::sync::RwLock;

pub struct AppState<C, D> {
    cache: RwLock<C>,
    database: RwLock<D>,
}
impl<C, D> AppState<C, D>
where
    C: Cache,
    D: Database,
{
    pub fn new(cache: C, database: D) -> Self {
        Self {
            cache: RwLock::new(cache),
            database: RwLock::new(database),
        }
    }

    pub fn cache(&self) -> &RwLock<C> {
        &self.cache
    }

    pub fn database(&self) -> &RwLock<D> {
        &self.database
    }
}
