use crate::app::app::App;

impl App {
    pub fn init_config(&mut self) {
        self.config.initialize();
    }
}