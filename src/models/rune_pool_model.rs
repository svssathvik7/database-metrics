use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RunePool {
    pub count: f64,
    pub end_time: i64,
    pub start_time: i64,
    pub units: f64,
}
