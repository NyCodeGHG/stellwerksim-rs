use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename = "zugdetails")]
pub struct TrainDetails {
    #[serde(rename = "zid")]
    pub id: String,
    pub name: String,
    #[serde(rename = "verspaetung")]
    pub delay: i32,
    #[serde(rename = "gleis")]
    pub platform: String,
    #[serde(rename = "plangleis")]
    pub scheduled_platform: String,
    #[serde(rename = "von")]
    pub origin: String,
    #[serde(rename = "nach")]
    pub destination: String,
    #[serde(rename = "sichtbar")]
    pub visible: bool,
    #[serde(rename = "amgleis")]
    pub at_platform: bool,
    #[serde(rename = "usertext")]
    pub user_text: Option<String>,
    #[serde(rename = "usertextsender")]
    pub user_text_sender: Option<String>,
    #[serde(rename = "hinweistext")]
    pub notice_text: Option<String>,
}
