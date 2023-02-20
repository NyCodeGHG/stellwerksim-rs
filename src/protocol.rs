#[cfg(feature = "simulator-time")]
use chrono::{NaiveDateTime, NaiveTime};
use serde::Deserialize;
#[cfg(feature = "simulator-time")]
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
pub(crate) struct SimulatorTimeResponse {
    #[serde_as(as = "NaiveTimeMilliSeconds")]
    #[serde(rename = "zeit")]
    pub time: NaiveTime,
}

#[cfg(feature = "simulator-time")]
struct NaiveTimeMilliSeconds;

#[cfg(feature = "simulator-time")]
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
