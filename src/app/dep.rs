use sqlx::migrate;
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
        self.sql_migrate().await?;
        Ok(())
    }
    async fn sql_migrate(&self) -> Result<(), sqlx::Error> {
        let pool = self.pool.as_ref().expect("pool not initialized");
        let migrator = migrate!("data/postgres/migrations/");

        match migrator.run(pool).await {
            Ok(applied) => println!("[POSTGRES] migrations applied: {:?} steps", applied),
            Err(e) => eprintln!("Migration failed: {:?}", e),
        }
        Ok(())
    }

    pub async fn init_redis(&mut self) -> redis::RedisResult<()> {
        let url = format!(
            "redis://{}:{}@{}:{}/{}",
            if self.config.cache.password.is_empty() { "" } else { &self.config.cache.password },
            "",
            self.config.cache.host,
            self.config.cache.port,
            self.config.cache.db
        );

        let client = redis::Client::open(url)?;
        let conn = client.get_multiplexed_async_connection().await?;
        println!("[REDIS] connected");

        self.cache = Some(conn);
        Ok(())
    }
}