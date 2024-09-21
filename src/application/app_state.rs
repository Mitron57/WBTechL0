use {
    crate::{
        domain::interfaces::{OrderService, Repository},
        utils::Interior,
    },
    std::ops::Deref,
};

pub struct AppState {
    repository: Interior<Box<dyn Repository + Send + Sync>>,
    order_service: Box<dyn OrderService + Send + Sync>,
}
impl AppState {
    pub fn new(
        repository: Box<dyn Repository + Send + Sync>,
        order_service: Box<dyn OrderService + Send + Sync>,
    ) -> Self {
        Self {
            repository: Interior::new(repository),
            order_service,
        }
    }

    pub fn repository_mut(&self) -> &mut Box<dyn Repository + Send + Sync> {
        self.repository.get_mut()
    }

    pub fn order_service(&self) -> &(dyn OrderService + Send + Sync) {
        self.order_service.deref()
    }
}
