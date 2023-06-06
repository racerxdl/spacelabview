use serde::Deserialize;

use super::atmosphere::*;
use super::environment::*;
use super::id::*;
use super::material::*;
use super::mesh_post_processing::MesherPostprocessing;
use super::oremapping::*;
use super::sound::SoundRules;
use super::vector::*;
use super::weather::*;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(deny_unknown_fields)]
pub struct Definitions {
    pub planet_generator_definitions: PlanetGeneratorDefinitions,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlanetGeneratorDefinitions {
    pub planet_generator_definition: PlanetGeneratorDefinition,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlanetGeneratorDefinition {
    pub id: Id,
    pub planet_maps: PlanetMaps,
    #[serde(rename = "MesherPostprocessing")]
    pub mesher_postprocessing: MesherPostprocessing,
    pub surface_detail: SurfaceDetail,
    pub ore_mappings: OreMappings,
    pub sound_rules: SoundRules,
    pub complex_materials: ComplexMaterials,
    pub environment_items: EnvironmentItems,
    pub default_surface_material: DefaultSurfaceMaterial,
    pub default_sub_surface_material: DefaultSubSurfaceMaterial,
    pub has_atmosphere: bool,
    pub atmosphere: Atmosphere,
    pub atmosphere_settings: AtmosphereSettings,
    pub cloud_layers: CloudLayers,
    pub weather_frequency_min: u32,
    pub weather_frequency_max: u32,
    pub weather_generators: WeatherGenerators,
    pub minimum_surface_layer_depth: u32,
    pub surface_gravity: u32,
    pub materials_max_depth: MaterialsMaxDepth,
    pub materials_min_depth: MaterialsMinDepth,
    pub hill_params: HillParams,
}
#[derive(Deserialize, Debug, PartialEq)]
pub struct PlanetMaps {
    #[serde(rename = "Material")]
    pub material: bool,
    #[serde(rename = "Ores")]
    pub ores: bool,
    #[serde(rename = "Biome")]
    pub biome: bool,
    #[serde(rename = "Occlusion")]
    pub occlusion: bool,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct MaterialsMaxDepth {
    #[serde(rename = "Min")]
    pub min: u32,
    #[serde(rename = "Max")]
    pub max: u32,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct MaterialsMinDepth {
    #[serde(rename = "Min")]
    pub min: u32,
    #[serde(rename = "Max")]
    pub max: u32,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct HillParams {
    #[serde(rename = "Min")]
    pub min: f32,
    #[serde(rename = "Max")]
    pub max: f32,
}
#[derive(Deserialize, Debug, PartialEq)]
pub struct SurfaceDetail {
    #[serde(rename = "Texture")]
    pub texture: String,
    #[serde(rename = "Size")]
    pub size: i32,
    #[serde(rename = "Scale")]
    pub scale: i32,
    #[serde(rename = "Slope")]
    pub slope: Range,
    #[serde(rename = "Transition")]
    pub transition: f64,
}
