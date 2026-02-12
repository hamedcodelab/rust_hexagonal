use sqlx::PgPool;
use std::sync::Arc;
use async_trait::async_trait;

use crate::user::port::UserRepository;
use crate::user::domain::user::User;
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
        // let rows = sqlx::query_as!(
        //     User,
        //     "SELECT * FROM deposits"
        // )
        //     .fetch_all(&self.pool)
        //     .await?;
        //
        // Ok(rows)
        Ok(vec![])
    }

    async fn create(&self, u: &User) -> Result<Option<User>, RepoError> {
        // sqlx::query!(
        //     "INSERT INTO deposits (id, amount) VALUES ($1, $2)",
        //     deposit.id,
        //     deposit.amount
        // )
        //     .execute(&self.pool)
        //     .await?;
        //
        println!("Repository create Call , Done!");
        let user = User::new(u.email.clone());

        Ok(Some(user))
    }

    async fn get_by_id(&self, id: u64) -> Result<Option<User>, RepoError> {
        // let row = sqlx::query_as!(
        //     User,
        //     "SELECT * FROM deposits WHERE id = $1",
        //     id as i64
        // )
        //     .fetch_optional(&self.pool)
        //     .await?;
        //
        // Ok(row)
        Ok(None)
    }

    async fn delete_by_id(&self, id: u64) -> Result<(), RepoError> {
        // sqlx::query!(
        //     "DELETE FROM deposits WHERE id = $1",
        //     id as i64
        // )
        //     .execute(&self.pool)
        //     .await?;

        Ok(())
    }

}

