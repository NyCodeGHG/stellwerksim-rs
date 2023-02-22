use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct SystemInfo {
    #[serde(rename = "simbuild")]
    pub build: i32,
    pub name: String,
    pub aid: i32,
    pub region: String,
    pub online: bool,
}
