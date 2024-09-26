use {
    crate::{
        domain::interfaces::{OrderService, self},
    },
    std::{ops::Deref, error::Error}
};

type Repository = dyn interfaces::Repository<Error = Box<dyn Error>>;

pub struct AppState {
    repository: Box<Repository>,
    order_service: Box<dyn OrderService>,
}
impl AppState {
    pub fn new(
        repository: Box<Repository>,
        order_service: Box<dyn OrderService>,
    ) -> Self {
        Self {
            repository,
            order_service,
        }
    }

    pub fn repository(&self) -> &Repository {
        self.repository.deref()
    }

    pub fn order_service(&self) -> &(dyn OrderService) { 
        self.order_service.deref()
    }
}
