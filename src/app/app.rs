use std::time::Duration;
use crate::app::config::{Config,Grpc};

pub struct App {
   pub config : Config ,
}

impl App {
    pub fn new(cn:String,pn:String) -> App {
        let conf = Config {
            container_name : cn ,
            name : pn ,
            grpc: Grpc {
                address: "127.0.0.1".to_string(),
                port: "50051".to_string(),
                grace_fully_shutdown_timeout: Duration::from_secs(0),
            },
        };

        App {
         config : conf,
        }
    }
}