use crate::app::app::App;
use sqlx::postgres::PgPoolOptions;

impl App {
    pub fn init_config(&mut self) {
        self.config.initialize();
    }

    pub async fn init_postgres(&mut self) -> Result<(), sqlx::Error> {
        let pool = PgPoolOptions::new()
            .connect(&self.config.postgres.connection_string)
            .await?;

        self.pool = Some(pool);
        Ok(())
    }
}