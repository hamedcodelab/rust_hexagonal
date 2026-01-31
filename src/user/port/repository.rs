use async_trait::async_trait;
use crate::user::domain::user::User;
use crate::user::port::RepoError;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn list(&self) -> Result<Vec<User>, RepoError>;
    async fn create(&self, user: &User) -> Result<(), RepoError>;
    async fn get_by_id(&self, id: u64) -> Result<Option<User>, RepoError>;
    async fn delete_by_id(&self, id: u64) -> Result<(), RepoError>;
}