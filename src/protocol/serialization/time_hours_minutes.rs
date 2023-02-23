use chrono::NaiveTime;
use serde::{de, Deserialize};
use serde_with::DeserializeAs;

pub struct NaiveTimeHoursMinutes;

impl<'de> DeserializeAs<'de, NaiveTime> for NaiveTimeHoursMinutes {
    fn deserialize_as<D>(deserializer: D) -> Result<NaiveTime, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let string: String = Deserialize::deserialize(deserializer)?;
        NaiveTime::parse_from_str(&string, "%H:%M").map_err(de::Error::custom)
    }
}
