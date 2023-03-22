use deadpool_postgres::{Manager, Pool};

use super::config::Config as ConfigInfo;

pub fn get_dbpool() -> Pool {
    let config_info = ConfigInfo::load();
    let mut config = tokio_postgres::Config::new();
    config
        .user(config_info.postgresql.user.as_str())
        .password(config_info.postgresql.password.as_str())
        .dbname(config_info.postgresql.database.as_str())
        .host(config_info.postgresql.host.as_str())
        .port(config_info.postgresql.port);
    let mgr = Manager::new(config, tokio_postgres::NoTls);
    Pool::builder(mgr).max_size(16).build().unwrap()
}
