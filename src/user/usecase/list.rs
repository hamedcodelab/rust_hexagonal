use crate::user::port::UserRepository;
use crate::user::domain::user::User;
use crate::user::port::RepoError;

pub async fn list(user_repo: &dyn UserRepository) -> Result<Vec<User>, RepoError> {
    let rows =  user_repo.list().await?;
    Ok(rows)
}
