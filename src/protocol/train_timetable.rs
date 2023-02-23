use chrono::NaiveTime;
use serde::Deserialize;
use serde_with::serde_as;

use super::serialization::NaiveTimeHoursMinutes;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename = "zugfahrplan")]
pub struct TrainTimetable {
    #[serde(rename = "zid")]
    pub id: String,
    #[serde(rename = "gleis")]
    pub platforms: Vec<TimetablePlatform>,
}

#[serde_as]
#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct TimetablePlatform {
    #[serde(rename = "plan")]
    pub platform: String,
    pub name: String,
    #[serde_as(as = "NaiveTimeHoursMinutes")]
    #[serde(rename = "an")]
    pub arrival: NaiveTime,
    #[serde_as(as = "NaiveTimeHoursMinutes")]
    #[serde(rename = "ab")]
    pub departure: NaiveTime,
    pub flags: String,
}
