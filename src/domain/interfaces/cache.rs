use crate::domain::models::Order;
pub trait Cache {
    fn add(&mut self, order_id: String, order: Order);
    
    fn get(&self, key: &str) -> Option<&Order>;

    fn remove(&mut self, order_id: &str) -> Option<Order>;
}