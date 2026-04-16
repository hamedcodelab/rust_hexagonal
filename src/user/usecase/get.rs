use crate::user::port::UserRepository;
use crate::user::domain::user::User;
use crate::user::port::RepoError;



pub async fn get_by_id(user_repo: &dyn UserRepository, id: i64) -> Result<Option<User>, RepoError> {
    let row =  user_repo.get_by_id(id).await?;
    Ok(row)
}
