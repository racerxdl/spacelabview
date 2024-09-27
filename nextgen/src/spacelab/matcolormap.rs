use std::collections::HashMap;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use super::{coloravg::MatColorAverage, matfile::MatFile};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PlanetMaterials(pub HashMap<String, PlanetMaterial>);

lazy_static! {
    pub static ref ORE_COLORS: HashMap<String, (u8, u8, u8)> = {
        let mut ore_colors = HashMap::new();
        ore_colors.insert("Iron_01".to_string(), (255, 216, 0));
        ore_colors.insert("Iron_02".to_string(), (255, 216, 0));
        ore_colors.insert("Nickel_01".to_string(), (239, 166, 117));
        ore_colors.insert("Silicon_01".to_string(), (216, 107, 128));
        ore_colors.insert("Magnesium_01".to_string(), (0, 255, 255));
        ore_colors.insert("Cobalt_01".to_string(), (181, 254, 0));
        ore_colors.insert("Silver_01".to_string(), (145, 145, 145));
        ore_colors.insert("Gold_01".to_string(), (255, 0, 220));
        ore_colors.insert("Platinum_01".to_string(), (219, 249, 255));
        ore_colors.insert("Uraninite_01".to_string(), (155, 114, 241));
        ore_colors.insert("Copper".to_string(), (184, 115, 51));
        ore_colors.insert("Bauxite".to_string(), (249, 166, 64));
        ore_colors.insert("Coal".to_string(), (145, 145, 145));
        ore_colors.insert("Titanium".to_string(), (81, 127, 84));
        ore_colors.insert("OilSand".to_string(), (183, 0, 3));
        ore_colors.insert("Sulfur".to_string(), (190, 167, 151));
        ore_colors.insert("Lithium".to_string(), (172, 80, 141));
        ore_colors.insert("Tantalum".to_string(), (255, 0, 0));
        ore_colors.insert("Cronyx".to_string(), (184, 115, 51));
        ore_colors.insert("Dorium".to_string(), (181, 254, 0));
        ore_colors
    };
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OreMap {
    #[serde(rename = "Value")]
    pub value: Option<u32>,
    #[serde(rename = "Type")]
    pub ore_type: Option<String>,
    #[serde(rename = "Start")]
    pub start: Option<u32>,
    #[serde(rename = "Depth")]
    pub depth: Option<u32>,
    #[serde(rename = "TargetColor")]
    pub target_color: Option<[u32; 3]>,
    #[serde(rename = "ColorInfluence")]
    pub color_influence: Option<u32>,
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PlanetMaterial {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "DefaultMaterial")]
    pub default_material: MaterialLayer,
    #[serde(rename = "SimpleMaterials")]
    pub simple_materials: HashMap<String, MaterialLayer>,
    #[serde(rename = "ComplexMaterials")]
    pub complex_materials: HashMap<String, VoxelMaterial>,
    #[serde(rename = "Ores")]
    pub ores: HashMap<String, OreMap>,
    #[serde(rename = "BaseFolder")]
    pub base_path: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MaterialLayer {
    #[serde(rename = "R")]
    pub r: u8,
    #[serde(rename = "G")]
    pub g: u8,
    #[serde(rename = "B")]
    pub b: u8,
    #[serde(rename = "Material")]
    pub material: String,
    #[serde(rename = "Depth")]
    pub depth: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct VoxelMaterial {
    pub id: i32,
    pub name: String,
    pub rules: Vec<MaterialRule>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MaterialRule {
    #[serde(rename = "Layers")]
    pub layers: Vec<MaterialLayer>,
    #[serde(rename = "MinHeight")]
    pub min_height: f32,
    #[serde(rename = "MaxHeight")]
    pub max_height: f32,
    #[serde(rename = "LatitudeMin")]
    pub latitude_min: f32,
    #[serde(rename = "LatitudeMax")]
    pub latitude_max: f32,
    #[serde(rename = "SlopeMin")]
    pub slope_min: f32,
    #[serde(rename = "SlopeMax")]
    pub slope_max: f32,
}

impl MaterialRule {
    pub fn matches(&self, height: f32, lat: f32, slope: f32) -> bool {
        if height != -999.0 && (height < self.min_height || height > self.max_height) {
            return false;
        }
        if lat != -999.0 && (lat < self.latitude_min || lat > self.latitude_max) {
            return false;
        }
        if slope != -999.0 && (slope < self.slope_min || slope > self.slope_max) {
            return false;
        }
        true
    }
    pub fn get_first_layer_color(&self) -> Option<(u8, u8, u8)> {
        if let Some(layer) = self.layers.get(0) {
            return Some((layer.r, layer.g, layer.b));
        }
        None // No layers exist
    }

    pub fn get_lowest_depth_color(&self) -> Option<(u8, u8, u8)> {
        let mut lowest_layer: Option<&MaterialLayer> = None;
        for layer in &self.layers {
            if lowest_layer.is_none() || layer.depth < lowest_layer.unwrap().depth {
                lowest_layer = Some(layer);
            }
        }
        lowest_layer.map(|layer| (layer.r, layer.g, layer.b))
    }
}
impl VoxelMaterial {
    pub fn get_layer(
        &self,
        height: f32,
        lat: f32,
        slope: f32,
    ) -> (Option<&MaterialRule>, Option<(u8, u8, u8)>) {
        for rule in &self.rules {
            if rule.matches(height, lat, slope) {
                return (Some(rule), rule.get_first_layer_color());
            }
        }
        (None, None)
    }
}
impl PlanetMaterial {
    pub fn cache(&mut self, matfiles: &MatFile, matcoloravg: &MatColorAverage) {
        // Cache Complex
        for voxel_material in self.complex_materials.values_mut() {
            for rule in &mut voxel_material.rules {
                for layer in &mut rule.layers {
                    if let Some(entry) = matfiles.0.get(&layer.material) {
                        if let Some(color) = matcoloravg.0.get(&entry.path).and_then(|pm_avg| {
                            pm_avg.0.get(&entry.file).map(|avg_color| &avg_color.0)
                        }) {
                            if let [r, g, b] = color.as_slice() {
                                layer.r = *r;
                                layer.g = *g;
                                layer.b = *b;
                            } else {
                                println!("Invalid color format: {} | {}", entry.path, entry.file);
                            }
                        } else if let Some(color) =
                            matcoloravg.0.get("default").and_then(|pm_avg| {
                                pm_avg.0.get(&entry.file).map(|avg_color| &avg_color.0)
                            })
                        {
                            if let [r, g, b] = color.as_slice() {
                                layer.r = *r;
                                layer.g = *g;
                                layer.b = *b;
                            } else {
                                println!("Invalid color format: {} | {}", entry.path, entry.file);
                            }
                        } else {
                            println!("404 color: {} | {}", entry.path, entry.file);
                        }
                    } else {
                        println!("404: {}", layer.material);
                    }
                }
            }
        }

        // Cache Simple
        for layer in self.simple_materials.values_mut() {
            if let Some(entry) = matfiles.0.get(&layer.material) {
                if let Some(color) = matcoloravg
                    .0
                    .get(&entry.path)
                    .and_then(|pm_avg| pm_avg.0.get(&entry.file).map(|avg_color| &avg_color.0))
                {
                    if let [r, g, b] = color.as_slice() {
                        layer.r = *r;
                        layer.g = *g;
                        layer.b = *b;
                    } else {
                        println!("Invalid color format: {} | {}", entry.path, entry.file);
                    }
                } else if let Some(color) = matcoloravg
                    .0
                    .get("default")
                    .and_then(|pm_avg| pm_avg.0.get(&entry.file).map(|avg_color| &avg_color.0))
                {
                    if let [r, g, b] = color.as_slice() {
                        layer.r = *r;
                        layer.g = *g;
                        layer.b = *b;
                    } else {
                        println!("Invalid color format: {} | {}", entry.path, entry.file);
                    }
                } else {
                    println!("404 color: {} | {}", entry.path, entry.file);
                }
            } else {
                println!("404: {}", layer.material);
            }
        }

        // Cache default
        if let Some(entry) = matfiles.0.get(&self.default_material.material) {
            if let Some(color) = matcoloravg
                .0
                .get(&entry.path)
                .and_then(|pm_avg| pm_avg.0.get(&entry.file).map(|avg_color| &avg_color.0))
            {
                if let [r, g, b] = color.as_slice() {
                    self.default_material.r = *r;
                    self.default_material.g = *g;
                    self.default_material.b = *b;
                } else {
                    println!("Invalid color format: {} | {}", entry.path, entry.file);
                }
            } else if let Some(color) = matcoloravg
                .0
                .get("default")
                .and_then(|pm_avg| pm_avg.0.get(&entry.file).map(|avg_color| &avg_color.0))
            {
                if let [r, g, b] = color.as_slice() {
                    self.default_material.r = *r;
                    self.default_material.g = *g;
                    self.default_material.b = *b;
                } else {
                    println!("Invalid color format: {} | {}", entry.path, entry.file);
                }
            } else {
                println!("404 color: {} | {}", entry.path, entry.file);
            }
        }
    }
}
