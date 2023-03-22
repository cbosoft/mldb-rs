use deadpool_postgres::Pool;
use rocket::response::content::RawJson;
use rocket::State;

use super::experiment::Experiment;

pub async fn get_experiment_details(pool: &State<Pool>) -> RawJson<String> {
    let client = pool.get().await.unwrap();

    let rows = client
        .query("SELECT * FROM STATUS ORDER BY EXPID DESC LIMIT 100;", &[])
        .await
        .unwrap();

    let mut exps: Vec<Experiment> = Vec::new();
    for row in rows {
        let expid: String = row.get("EXPID");
        let status: String = row.get("STATUS");

        exps.push(Experiment::new(expid, status));
    }

    let mut rv = Vec::new();
    for exp in exps {
        let groups = exp.groups.join(", ");
        let expid = exp.expid;
        let status = exp.status;
        rv.push((expid, status, groups));
    }

    RawJson(serde_json::to_string(&rv).unwrap())
}
