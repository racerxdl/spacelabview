use serde::Deserialize;

use super::vector::{Height, Latitude, SunAngleFromZenith};

#[derive(Deserialize, Debug, PartialEq)]
pub struct SoundRule {
    #[serde(rename = "Height")]
    pub height: Option<Height>,
    #[serde(rename = "Latitude")]
    pub latitude: Option<Latitude>,
    #[serde(rename = "SunAngleFromZenith")]
    pub sun_angle_from_zenith: Option<SunAngleFromZenith>,
    #[serde(rename = "EnvironmentSound")]
    pub environment_sound: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct SoundRules {
    #[serde(rename = "SoundRule")]
    pub sound_rule: Vec<SoundRule>,
}
