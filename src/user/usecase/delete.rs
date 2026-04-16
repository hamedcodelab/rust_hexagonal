use crate::user::port::UserRepository;
use crate::user::port::RepoError;



pub async fn delete_by_id(user_repo: &dyn UserRepository, id: i64) -> Result<(), RepoError> {
    let res =  user_repo.delete_by_id(id).await?;
    Ok(res)
}
