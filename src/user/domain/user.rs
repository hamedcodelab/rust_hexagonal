use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BaseModel {
    pub id: u64,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub base: BaseModel,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn new(email: String) -> Self {
        let new_password : String;
        new_password = "password".to_string();
        Self {
            base: BaseModel::new(),
            email,
            password : new_password,
        }
    }
}

impl BaseModel {
    pub fn new() -> Self {
        Self {
            id: 0,
            created_at: None,
            updated_at: None,
            deleted_at: None,
        }
    }
}