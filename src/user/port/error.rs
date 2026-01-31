use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepoError {
    #[error("database error")]
    Database(#[from] sqlx::Error),

    #[error("not found")]
    NotFound,

    #[error("unknown error")]
    Unknown,
}