use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename = "bahnsteigliste")]
pub(crate) struct PlatformListResponse {
    #[serde(rename = "bahnsteig")]
    pub platforms: Vec<Platform>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Platform {
    pub name: String,
    #[serde(rename = "haltepunkt")]
    pub stop: bool,
    #[serde(rename = "n", default)]
    pub neighbors: Vec<PlatformNeighbor>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct PlatformNeighbor {
    pub name: String,
}
