use bcrypt::{hash, verify, DEFAULT_COST};
use crate::user::port::RepoError;

pub fn validate_password(password_hash: &str, password: &str) -> Result<(), RepoError> {
    match verify(password, password_hash) {
        Ok(true) => Ok(()),

        Ok(false) => Err(RepoError::NotFound),

        Err(_) => Err(RepoError::Unknown),
    }
}

pub fn create_hashed_password(password: &str) -> Result<String, RepoError> {
    match hash(password, DEFAULT_COST) {
        Ok(hashed) => Ok(hashed),

        Err(_) => Err(RepoError::Unknown),
    }
}