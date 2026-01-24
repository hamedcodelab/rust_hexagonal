use crate::app::config::Config;


pub struct App {
   pub config : Config,
   pub pool : Option<sqlx::PgPool>
}

impl App {
    pub fn new(conf:Config) -> App {
        App {
            config: conf,
            pool: None,
        }
    }
    pub async fn init_dep(&mut self) {
           self.init_config();
           self.init_postgres().await.expect("Error initializing postgres connection");
    }
}