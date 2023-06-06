use super::{material::Textures, vector::Vector3F};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Atmosphere {
    #[serde(rename = "Breathable")]
    pub breathable: Option<bool>,
    #[serde(rename = "OxygenDensity")]
    pub oxygen_density: Option<u32>,
    #[serde(rename = "Density")]
    pub density: Option<f32>,
    #[serde(rename = "LimitAltitude")]
    pub limit_altitude: Option<f32>,
    #[serde(rename = "MaxWindSpeed")]
    pub max_wind_speed: Option<u32>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct AtmosphereSettings {
    #[serde(rename = "RayleighScattering")]
    pub rayleigh_scattering: Option<Vector3F>,
    #[serde(rename = "MieScattering")]
    pub mie_scattering: Option<f32>,
    #[serde(rename = "MieColorScattering")]
    pub mie_color_scattering: Option<Vector3F>,
    #[serde(rename = "RayleighHeight")]
    pub rayleigh_height: Option<f32>,
    #[serde(rename = "RayleighHeightSpace")]
    pub rayleigh_height_space: Option<f32>,
    #[serde(rename = "RayleighTransitionModifier")]
    pub rayleigh_transition_modifier: Option<f32>,
    #[serde(rename = "MieHeight")]
    pub mie_height: Option<f32>,
    #[serde(rename = "MieG")]
    pub mie_g: Option<f32>,
    #[serde(rename = "Intensity")]
    pub intensity: Option<f32>,
    #[serde(rename = "SeaLevelModifier")]
    pub sea_level_modifier: Option<f32>,
    #[serde(rename = "AtmosphereTopModifier")]
    pub atmosphere_top_modifier: Option<f32>,
    #[serde(rename = "FogIntensity")]
    pub fog_intensity: Option<f32>,
    #[serde(rename = "Scale")]
    pub scale: Option<u32>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct CloudLayers {
    #[serde(rename = "CloudLayer")]
    pub cloud_layer: Option<Vec<CloudLayer>>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct CloudLayer {
    #[serde(rename = "Model")]
    pub model: Option<String>,
    #[serde(rename = "Textures")]
    pub textures: Option<Textures>,
    #[serde(rename = "RelativeAltitude")]
    pub relative_altitude: Option<f32>,
    #[serde(rename = "RotationAxis")]
    pub rotation_axis: Option<Vector3F>,
    #[serde(rename = "AngularVelocity")]
    pub angular_velocity: Option<f32>,
    #[serde(rename = "InitialRotation")]
    pub initial_rotation: Option<f32>,
    #[serde(rename = "ScalingEnabled")]
    pub scaling_enabled: Option<bool>,
    #[serde(rename = "FadeOutRelativeAltitudeStart")]
    pub fade_out_relative_altitude_start: Option<f32>,
    #[serde(rename = "FadeOutRelativeAltitudeEnd")]
    pub fade_out_relative_altitude_end: Option<f32>,
    #[serde(rename = "ApplyFogRelativeDistance")]
    pub apply_fog_relative_distance: Option<f32>,
}
