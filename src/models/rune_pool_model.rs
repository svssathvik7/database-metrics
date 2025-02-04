use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RunePool {
    pub count: String,
    pub end_time: String,
    pub start_time: String,
    pub units: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RunePoolMeta {
    end_count: String,
    end_time: String,
    end_units: String,
    start_count: String,
    start_time: String,
    start_units: String,
}

#[derive(Deserialize)]
pub struct RunePoolResponse {
    pub intervals: Vec<RunePool>,
    pub meta: RunePoolMeta,
}
