use serde::Deserialize;
use std::time::Duration;


#[derive(Debug, Deserialize)]
pub struct Config {
    pub container_name : String ,
    pub name : String,
    pub grpc : Grpc,
    pub postgres : Postgres,
}

#[derive(Debug, Deserialize)]
pub struct Grpc {
    pub address : String,
    pub port : String ,
    #[serde(default, with = "humantime_serde")]
    pub grace_fully_shutdown_timeout : Duration,
}

#[derive(Debug, Deserialize)]
pub struct Postgres {
    pub debug : bool,
    pub dialect : String ,
    pub connection_string : String ,
    pub max_idle_connections : u16 ,
    pub max_open_connections : u16 ,
    pub max_lifetime_seconds : u16 ,
}

impl Config {
    pub fn initialize(&mut self) {
         if self.grpc.grace_fully_shutdown_timeout == Duration::ZERO {
             self.grpc.grace_fully_shutdown_timeout = Duration::from_secs(10)
         }
    }
}