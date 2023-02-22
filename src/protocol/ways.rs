use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename = "wege")]
pub struct Ways {
    #[serde(rename = "shape")]
    pub shapes: Vec<Shape>,
    #[serde(rename = "connector")]
    pub connectors: Vec<Connector>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
pub struct Shape {
    #[serde(rename = "type")]
    pub shape_type: String,
    pub name: Option<String>,
    pub enr: Option<String>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum Connector {
    Enrs {
        enr1: String,
        enr2: String,
    },
    EnrName {
        #[serde(alias = "enr1", alias = "enr2")]
        enr: String,
        #[serde(alias = "name1", alias = "name2")]
        name: String,
    },
    Names {
        name1: String,
        name2: String,
    },
}
