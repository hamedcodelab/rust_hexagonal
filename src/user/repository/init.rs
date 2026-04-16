use sqlx::{PgPool, Row};
use std::sync::Arc;
use async_trait::async_trait;
use chrono::Utc;
use crate::user::port::UserRepository;
use crate::user::domain::user::User;
use crate::user::domain::user::BaseModel;
use crate::user::port::RepoError;

pub struct UserR {
    pool: PgPool,
}

impl UserR {
    pub fn new(pool: PgPool) -> Arc<dyn UserRepository> {
        Arc::new(Self { pool })
    }
}

#[async_trait]
impl UserRepository for UserR {
    async fn list(&self) -> Result<Vec<User>, RepoError> {
        let rows = sqlx::query(
            r#"
        SELECT id, email, created_at
        FROM users
        "#
        )
            .fetch_all(&self.pool)
            .await
            .map_err(RepoError::from)?;

        let users = rows
            .into_iter()
            .map(|rec| User {
                base: BaseModel {
                    id: rec.get("id"),
                    created_at: rec.get("created_at"),
                    updated_at: None,
                    deleted_at: None,
                },
                email: rec.get("email"),
                password: String::new(), // 🔐 never expose password
            })
            .collect();

        Ok(users)
    }

    async fn create(&self, u: &User) -> Result<User, RepoError> {
        let rec = sqlx::query(
                    r#"
            INSERT INTO users (created_at,email, password)
            VALUES ($1, $2, $3)
            RETURNING id, email, created_at
            "#
                )
            .bind(&u.base.created_at)
            .bind(&u.email)
            .bind(&u.password)
            .fetch_one(&self.pool)
            .await
            .map_err(RepoError::from)?;




       Ok(User{
           base: BaseModel {
               id: rec.get::<i64, _>("id"),
               created_at: rec.get::<Option<chrono::DateTime<Utc>>, _>("created_at"),
               updated_at: None,
               deleted_at: None,
           },
           email: rec.get::<String, _>("email"),
           // password: rec.get::<String, _>("password"),
           password: String::new(),
       })
    }

    async fn get_by_id(&self, id: i64) -> Result<Option<User>, RepoError> {
        let rec = sqlx::query(
            r#"
            SELECT id, email, created_at FROM users WHERE id = $1
            "#
        ).bind(&id).fetch_optional(&self.pool).await.map_err(RepoError::from)?;
        match rec {
            Some(rec) => Ok(Some(User {
                base: BaseModel {
                    id: rec.get("id"),
                    created_at: rec.get("created_at"),
                    updated_at: None,
                    deleted_at: None,
                },
                email: rec.get("email"),
                password: String::new(),
            })),
            None => Ok(None),
        }
    }

    async fn delete_by_id(&self, id: i64) -> Result<(), RepoError> {
        let result = sqlx::query(
            r#"
        UPDATE users
        SET deleted_at = NOW()
        WHERE id = $1 AND deleted_at IS NULL
        "#
        ).bind(id)
            .execute(&self.pool)
            .await
            .map_err(RepoError::from)?;

        if result.rows_affected() == 0 {
            return Err(RepoError::NotFound);
        }

        Ok(())
    }

}

