use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Biome {
    #[serde(rename = "Biome")]
    pub biome: Option<u32>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Material {
    #[serde(rename = "Material")]
    pub material: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ItemDetail {
    #[serde(rename = "TypeId")]
    pub type_id: Option<String>,
    #[serde(rename = "SubtypeId")]
    pub subtype_id: Option<String>,
    #[serde(rename = "Density")]
    pub density: Option<f32>,
    #[serde(rename = "GroupId")]
    pub group_id: Option<String>,
    #[serde(rename = "ModifierId")]
    pub modifier_id: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct DefaultSurfaceMaterial {
    #[serde(rename = "Material")]
    pub material: Option<String>,
    #[serde(rename = "MaxDepth")]
    pub max_depth: Option<u32>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct DefaultSubSurfaceMaterial {
    #[serde(rename = "Material")]
    pub material: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct EnvironmentItems {
    #[serde(rename = "Item")]
    pub item: Vec<Item>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Item {
    #[serde(rename = "Biomes")]
    pub biomes: Biomes,
    #[serde(rename = "Materials")]
    pub materials: Materials,
    #[serde(rename = "Items")]
    pub items: Items,
    #[serde(rename = "Rule")]
    pub rule: Rule,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Biomes {
    #[serde(rename = "Biome")]
    pub biome: Option<Vec<u32>>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Materials {
    #[serde(rename = "Material")]
    pub material: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Items {
    #[serde(rename = "Item")]
    pub item: Vec<ItemAttributes>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ItemAttributes {
    #[serde(rename = "TypeId")]
    pub type_id: Option<String>,
    #[serde(rename = "SubtypeId")]
    pub subtype_id: Option<String>,
    #[serde(rename = "GroupId")]
    pub group_id: Option<String>,
    #[serde(rename = "ModifierId")]
    pub modifier_id: Option<String>,
    #[serde(rename = "Density")]
    pub density: Option<f32>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Rule {
    #[serde(rename = "Height")]
    pub height: Option<MinMax>,
    #[serde(rename = "Latitude")]
    pub latitude: Option<MinMax>,
    #[serde(rename = "Slope")]
    pub slope: Option<MinMax>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct MinMax {
    #[serde(rename = "Min")]
    pub min: Option<f32>,
    #[serde(rename = "Max")]
    pub max: Option<f32>,
}
