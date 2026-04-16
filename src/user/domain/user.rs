use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BaseModel {
    pub id: i64,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone,sqlx::FromRow)]
pub struct User {
    pub base: BaseModel,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn new(email: String,password: String) -> Self {
        Self {
            base: BaseModel::new(),
            email,
            password,
        }
    }
}

impl BaseModel {
    pub fn new() -> Self {
        let created_at = Utc::now();
        Self {
            id: 0,
            created_at: Some(created_at),
            updated_at: None,
            deleted_at: None,
        }
    }
}
