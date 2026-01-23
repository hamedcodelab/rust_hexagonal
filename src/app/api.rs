use crate::app::app::App;

impl App {
    pub fn start(&self) {
        println!("Running {}", self.config.name);
        println!("Running {}", self.config.grpc.grace_fully_shutdown_timeout.as_secs());
    }

    pub fn stop(&self) {
        println!("Stopping {}", self.config.name);
    }
}