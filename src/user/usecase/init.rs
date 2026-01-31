use std::sync::Arc;
use async_trait::async_trait;
use crate::user::port::UserUsecase;
use crate::user::port::UserRepository;
use crate::user::domain::user::User;
use crate::user::port::RepoError;



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
        let rows =  self.user_repo.list().await?;
        Ok(rows)
    }

    async fn create(&self, deposit: &User) -> Result<(), RepoError> {
        Ok(())
    }

    async fn get_by_id(&self, id: u64) -> Result<Option<User>, RepoError> {
        let row =  self.user_repo.get_by_id(1).await?;
        Ok(row)
    }

    async fn delete_by_id(&self, id: u64) -> Result<(), RepoError> {
        Ok(())
    }

}

