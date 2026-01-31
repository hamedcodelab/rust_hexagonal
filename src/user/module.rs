use std::sync::Arc;
use crate::user::port::{UserRepository, UserUsecase};
use crate::user::usecase::init::UserU;

pub struct UserModule {
    pub uc: Arc<dyn UserUsecase>,
}

impl UserModule {
    pub fn new(repo: Arc<dyn UserRepository>) -> Self {
        Self {
            uc: UserU::new(repo),
        }
    }
}