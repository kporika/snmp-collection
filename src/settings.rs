use std::env;
use config::{ConfigError, Config, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Database {
    pub dbname: String,
    pub host: String,
    pub user: String,
    pub password: String,
   
}

#[derive(Debug, Deserialize)]
pub struct DataCollection {
    pub dc_actor_pairs: usize,
    pub retries: u8,
    pub wait_time_millis: u64,
    pub interval_seconds: u64,
    pub udp_bind_port: u32
}

#[derive(Debug, Deserialize)]
pub struct Logger {
    pub file_path: String,
    pub log_level: String 
}


#[derive(Debug, Deserialize)]
pub struct Settings {
    pub debug: bool,
    pub database: Database,
    pub datacollection: DataCollection, 
    pub logger: Logger

}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        // Start off by merging in the "default" configuration file
        s.merge(File::with_name("config/default"))?;

        // Add in the current environment file
        // Default to 'dev' env
        // Note that this file is _optional_
        let env = env::var("RUN_MODE").unwrap_or_else(|_| "dev".into());
        s.merge(File::with_name(&format!("config/{}", env)).required(false))?;
        
        s.try_into()
    }
}