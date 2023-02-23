use chrono::{NaiveDateTime, NaiveTime};
use serde_with::{formats::Strict, DeserializeAs, TimestampMilliSeconds};

pub struct NaiveTimeMilliSeconds;

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
