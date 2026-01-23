use config::{Config as Cfg, Environment, File};
use crate::app::config::Config;

pub fn load() -> Result<Config, config::ConfigError> {
    let env = std::env::var("APP_ENV").unwrap_or("local".into());
    let cfg = Cfg::builder()
        .add_source(File::with_name(&format!("src/cmd/config/config-{env}.yaml")).required(false))
        .add_source(Environment::with_prefix("APP").separator("__"))
        .build()?;

    cfg.try_deserialize()
}