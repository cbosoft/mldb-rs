mod config;
mod dbpool;
mod experiment;
mod overview;

#[macro_use]
extern crate rocket;

use std::env;
use std::net::Ipv4Addr;

use deadpool_postgres::Pool;
use rocket::fs::FileServer;
use rocket::response::content::RawJson;
use rocket::serde::{json::Json, Deserialize};
use rocket::State;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Command {
    pub command: String,
    // pub groups: Option<Vec<String>>,
    pub expids: Option<Vec<String>>,
}

#[post("/", data = "<data>")]
async fn command_processor(data: Json<Command>, pool: &State<Pool>) -> RawJson<String> {
    if data.command == "get_overview".to_string() {
        overview::get_experiment_details(pool).await
    } else if data.command == "get_groups".to_string() {
        todo!();
    } else if data.command == "get_experiments".to_string() {
        experiment::get_experiments(data.expids.as_ref().unwrap(), pool).await
    } else {
        todo!();
    }
}

#[launch]
fn rocket() -> _ {
    let ip: Ipv4Addr = match env::var("MLDB_SITE_IP") {
        Ok(ip) => ip.as_str().parse().unwrap(),
        Err(_) => "127.0.0.1".parse().unwrap(),
    };
    let port: u16 = match env::var("MLDB_SITE_PORT") {
        Ok(port) => port.as_str().parse().unwrap(),
        Err(_) => "8008".parse().unwrap(),
    };
    let config = rocket::Config {
        port,
        address: ip.into(),
        ..rocket::Config::debug_default()
    };

    rocket::custom(&config)
        .manage(dbpool::get_dbpool())
        .mount("/", FileServer::from("site"))
        .mount("/site", routes![command_processor])
}
