use std::sync::Arc;
use async_trait::async_trait;
use crate::user::port::UserUsecase;
use crate::user::port::UserRepository;
use crate::user::domain::user::User;
use crate::user::port::RepoError;


use crate::user::usecase::{list,create,delete,get};



pub struct UserU {
    pub user_repo: Arc<dyn UserRepository>
}

impl UserU {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Arc<dyn UserUsecase> {
        Arc::new(Self { user_repo })
    }
}

#[async_trait]
impl UserUsecase for UserU {
    async fn list(&self) -> Result<Vec<User>, RepoError> {
        list::list(self.user_repo.as_ref()).await
    }
    async fn create(&self, user: &mut User) -> Result<User, RepoError> {
        create::create(self.user_repo.as_ref(),user).await
    }
    async fn get_by_id(&self, id: i64) -> Result<Option<User>, RepoError> {
        get::get_by_id(self.user_repo.as_ref(),id).await
    }
    async fn delete_by_id(&self, id: i64) -> Result<(), RepoError> {
        delete::delete_by_id(self.user_repo.as_ref(),id).await
    }

}