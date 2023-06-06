use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Vector3F {
    #[serde(rename = "X")]
    pub x: f32,
    #[serde(rename = "Y")]
    pub y: f32,
    #[serde(rename = "Z")]
    pub z: f32,
}
#[derive(Debug, Deserialize, PartialEq)]
pub struct Height {
    #[serde(rename = "Min")]
    pub min: f32,
    #[serde(rename = "Max")]
    pub max: f32,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Latitude {
    #[serde(rename = "Min")]
    pub min: f32,
    #[serde(rename = "Max")]
    pub max: f32,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Slope {
    #[serde(rename = "Min")]
    pub min: f32,
    #[serde(rename = "Max")]
    pub max: f32,
}
#[derive(Deserialize, Debug, PartialEq)]
pub struct SunAngleFromZenith {
    #[serde(rename = "Min")]
    pub min: f64,
    #[serde(rename = "Max")]
    pub max: f64,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Range {
    #[serde(rename = "Min")]
    pub min: f32,
    #[serde(rename = "Max")]
    pub max: f32,
}
