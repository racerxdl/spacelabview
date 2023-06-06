use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MatFile(pub HashMap<String, MatEntry>);

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MatEntry {
    pub path: String,
    pub file: String,
}
