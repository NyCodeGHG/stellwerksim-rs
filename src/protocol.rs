use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Status {
    pub code: i32,
    #[serde(rename = "$value")]
    pub text: String,
}
