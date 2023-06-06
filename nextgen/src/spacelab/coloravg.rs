use std::collections::HashMap;

use serde::{Serialize, Deserialize};



#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MatColorAverage(pub HashMap<String, PlanetMaterialAverage>);

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PlanetMaterialAverage(pub HashMap<String, AverageColor>);

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct AverageColor(pub Vec<u8>);