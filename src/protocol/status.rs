use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Status {
    pub code: i32,
    #[serde(rename = "$value")]
    pub text: String,
}
