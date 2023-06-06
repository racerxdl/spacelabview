use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct LodSettings {
    #[serde(rename = "Settings")]
    pub settings: Option<Vec<Settings>>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Settings {
    #[serde(rename = "FromLod")]
    pub from_lod: Option<String>,
    #[serde(rename = "FeatureAngle")]
    pub feature_angle: Option<f32>,
    #[serde(rename = "EdgeThreshold")]
    pub edge_threshold: Option<f32>,
    #[serde(rename = "PlaneThreshold")]
    pub plane_threshold: Option<f32>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Step {
    #[serde(rename = "ForPhysics")]
    pub for_physics: Option<bool>,
    #[serde(rename = "LodSettings")]
    pub lod_settings: Option<LodSettings>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct PostprocessingSteps {
    #[serde(rename = "Step")]
    pub step: Option<Step>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct MesherPostprocessing {
    #[serde(rename = "PostprocessingSteps")]
    pub postprocessing_steps: Option<PostprocessingSteps>,
}
