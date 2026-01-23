use crate::app::config::Config;

pub struct App {
   pub config : Config ,
}

impl App {
    pub fn new(conf:Config) -> App {
        App {
            config: conf,
        }
    }
    pub fn init_dep(&mut self) {
           self.init_config();
    }
}