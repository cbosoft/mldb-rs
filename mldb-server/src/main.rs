mod config;
mod dbpool;
mod experiment;
mod overview;

#[macro_use]
extern crate rocket;

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
    rocket::build()
        .manage(dbpool::get_dbpool())
        .mount("/", FileServer::from("site"))
        .mount("/site", routes![command_processor])
}
