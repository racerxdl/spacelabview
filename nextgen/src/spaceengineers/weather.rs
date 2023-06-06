use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct WeatherGenerators {
    #[serde(rename = "WeatherGenerator")]
    pub weather_generator: Vec<WeatherGenerator>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct WeatherGenerator {
    #[serde(rename = "Voxel")]
    pub voxel: String,
    #[serde(rename = "Weathers")]
    pub weathers: Weathers,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Weathers {
    #[serde(rename = "Weather")]
    pub weather: Vec<Weather>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Weather {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Weight")]
    pub weight: Option<u32>,
    #[serde(rename = "MinLength")]
    pub min_length: Option<u32>,
    #[serde(rename = "MaxLength")]
    pub max_length: Option<u32>,
    #[serde(rename = "SpawnOffset")]
    pub spawn_offset: Option<u32>,
}
