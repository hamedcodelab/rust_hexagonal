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

    pub fn run(&self) {
        println!("Running {}", self.config.name);
    }
}