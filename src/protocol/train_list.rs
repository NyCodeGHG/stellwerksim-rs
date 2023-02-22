use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename = "zugliste")]
pub(crate) struct TrainListResponse {
    #[serde(rename = "zug")]
    pub trains: Vec<Train>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Train {
    #[serde(rename = "zid")]
    pub id: String,
    pub name: String,
}
