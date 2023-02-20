#[cfg(feature = "simulator-time")]
use chrono::NaiveDateTime;
use chrono::NaiveTime;
use serde::Deserialize;
use serde_with::{formats::Strict, serde_as, DeserializeAs, TimestampMilliSeconds};

#[derive(Debug, Deserialize)]
pub struct Status {
    pub code: i32,
    #[serde(rename = "$value")]
    pub text: String,
}

#[cfg(feature = "simulator-time")]
#[serde_as]
#[derive(Debug, Deserialize)]
pub struct SimulatorTime {
    #[serde_as(as = "NaiveTimeMilliSeconds")]
    #[serde(rename = "zeit")]
    pub time: NaiveTime,
}

struct NaiveTimeMilliSeconds;

impl<'de> DeserializeAs<'de, NaiveTime> for NaiveTimeMilliSeconds {
    fn deserialize_as<D>(deserializer: D) -> Result<NaiveTime, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let date: NaiveDateTime =
            TimestampMilliSeconds::<i64, Strict>::deserialize_as(deserializer)?;
        Ok(date.time())
    }
}

#[derive(Debug, Deserialize)]
pub struct SystemInfo {
    #[serde(rename = "simbuild")]
    pub build: i32,
    pub name: String,
    pub aid: i32,
    pub region: String,
    pub online: bool,
}
