use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct PostgresConfig {
    pub host: String,
    pub user: String,
    pub password: String,
    pub port: u16,
    pub database: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub backend: String,
    pub postgresql: PostgresConfig,
}

impl Config {
    pub fn load() -> Self {
        // Open config file at "~/.mldb_config.json"
        let home = std::env::var("HOME").unwrap();
        let mut path = PathBuf::from(home);
        path.push(".mldb_config.json");
        let mut config_f = File::open(path).unwrap();

        // Read config file contents to string
        let mut config_s = String::new();
        config_f.read_to_string(&mut config_s).unwrap();

        // Load config and return
        serde_json::from_str(config_s.as_str()).unwrap()
    }
}
