use chrono::NaiveTime;
use serde::Deserialize;
use serde_with::serde_as;

use super::serialize::NaiveTimeMilliSeconds;

#[serde_as]
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct SimulatorTimeResponse {
    #[serde_as(as = "NaiveTimeMilliSeconds")]
    #[serde(rename = "zeit")]
    pub time: NaiveTime,
}
