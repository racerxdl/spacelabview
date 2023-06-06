use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OreMappings {
    #[serde(rename = "Ore")]
    pub ores: Option<Vec<Ore>>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Ore {
    #[serde(rename = "Value")]
    pub value: Option<u32>,
    #[serde(rename = "Type")]
    pub ore_type: Option<String>,
    #[serde(rename = "Start")]
    pub start: Option<u32>,
    #[serde(rename = "Depth")]
    pub depth: Option<u32>,
    #[serde(rename = "TargetColor")]
    pub target_color: Option<String>,
    #[serde(rename = "ColorInfluence")]
    pub color_influence: Option<u32>,
}
