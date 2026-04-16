use crate::user::port::UserRepository;
use crate::user::domain::user::User;
use crate::user::port::RepoError;
use crate::helper::helper::create_hashed_password;


pub async fn create(user_repo: &dyn UserRepository, user: &mut User) -> Result<User, RepoError> {
    // hash password by add handler
    let hash  = create_hashed_password(user.password.as_str())?;
    user.password  = hash;
    let res =  user_repo.create(user).await?;
    Ok(res)
}
