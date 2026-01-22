use std::time::Duration;

pub struct Config {
    pub container_name : String ,
    pub name : String,
    pub grpc : Grpc,
}

pub struct Grpc {
    pub address : String,
    pub port : String ,
    pub grace_fully_shutdown_timeout : Duration,
}

impl Config {
    pub fn initialize(&mut self) {
         if self.grpc.grace_fully_shutdown_timeout == Duration::from_secs(0) {
             self.grpc.grace_fully_shutdown_timeout = Duration::from_secs(10)
         }
    }
}