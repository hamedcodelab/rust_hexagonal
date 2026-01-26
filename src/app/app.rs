use crate::app::config::Config;
use redis::{aio::MultiplexedConnection};


pub struct App {
   pub config : Config,
   pub pool : Option<sqlx::PgPool>,
   pub cache: Option<MultiplexedConnection>,
}

impl App {
    pub fn new(conf:Config) -> App {
        App {
            config: conf,
            pool: None,
            cache: None
        }
    }
    pub async fn init_dep(&mut self) {
           self.init_config();
           self.init_postgres().await.expect("Error initializing postgres connection");
        self.init_redis().await.expect("Error initializing redis connection");
    }
}