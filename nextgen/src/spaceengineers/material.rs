use serde::Deserialize;

use super::vector::{Height, Latitude, Slope};
#[derive(Debug, Deserialize, PartialEq)]
pub struct ComplexMaterials {
    #[serde(rename = "MaterialGroup")]
    pub material_groups: Option<Vec<MaterialGroup>>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct MaterialGroup {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Value")]
    pub value: Option<u32>,
    #[serde(rename = "Rule")]
    pub rules: Option<Vec<Rule>>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Rule {
    #[serde(rename = "Layers")]
    pub layers: Option<Vec<Layer>>,
    #[serde(rename = "Height")]
    pub height: Option<Height>,
    #[serde(rename = "Latitude")]
    pub latitude: Option<Latitude>,
    #[serde(rename = "Slope")]
    pub slope: Option<Slope>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Layer {
    #[serde(rename = "Material")]
    pub material: Option<String>,
    #[serde(rename = "Depth")]
    pub depth: Option<u32>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Textures {
    #[serde(rename = "Texture")]
    pub texture: Option<Vec<String>>,
}
