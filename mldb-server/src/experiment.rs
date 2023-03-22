use std::collections::HashMap;

use deadpool_postgres::Pool;
use rocket::response::content::RawJson;
use rocket::serde::Serialize;
use rocket::State;

#[derive(Serialize)]
pub struct Experiment {
    pub expid: String,
    pub status: String,
    pub groups: Vec<String>,
    pub losses: HashMap<String, Vec<(i64, f64)>>,
    pub metrics: HashMap<String, f64>,
}

impl Experiment {
    pub fn new(expid: String, status: String) -> Self {
        Experiment {
            expid,
            status,
            groups: Vec::new(),
            losses: HashMap::new(),
            metrics: HashMap::new(),
        }
    }

    async fn expdetails(expid: String, pool: &State<Pool>) -> Experiment {
        let client = pool.get().await.unwrap();
        let rows = client
            .query("SELECT * FROM STATUS WHERE EXPID=$1::TEXT", &[&expid])
            .await
            .unwrap();
        let status: String = rows[0].get("STATUS");

        let mut exp = Experiment::new(expid.clone(), status);

        let rows = client
            .query(
                "SELECT GROUPNAME FROM EXPGROUPS WHERE EXPID=$1::TEXT",
                &[&expid],
            )
            .await
            .unwrap();
        for row in rows {
            let groupname = row.get("GROUPNAME");
            exp.groups.push(groupname);
        }

        let rows = client
            .query("SELECT * FROM LOSS WHERE EXPID=$1::TEXT;", &[&expid])
            .await
            .unwrap();

        for row in rows {
            let loss_value: f32 = row.get("VALUE");
            let loss_kind: String = row.get("KIND");
            let epoch: i32 = row.get("EPOCH");
            if !exp.losses.contains_key(&loss_kind) {
                exp.losses.insert(loss_kind.clone(), Vec::new());
            }
            exp.losses
                .get_mut(&loss_kind)
                .unwrap()
                .push((epoch.into(), loss_value.into()));
        }

        let rows = client
            .query(
                "SELECT * FROM METRICS
            WHERE (EXPID, EPOCH) IN
            (SELECT EXPID, max(EPOCH) FROM METRICS WHERE EXPID=$1::TEXT GROUP BY EXPID);",
                &[&expid],
            )
            .await
            .unwrap();
        for row in rows {
            let metric: String = row.get("KIND");
            let value: f32 = row.get("VALUE");
            exp.metrics.insert(metric, value.into());
        }

        exp
    }
}

pub async fn get_experiments(expids: &Vec<String>, pool: &State<Pool>) -> RawJson<String> {
    let mut exps: Vec<Experiment> = Vec::new();
    for expid in expids {
        let exp = Experiment::expdetails(expid.clone(), pool).await;
        exps.push(exp);
    }
    RawJson(serde_json::to_string(&exps).unwrap())
}
