use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Id {
    #[serde(rename = "TypeId")]
    pub type_id: String,
    #[serde(rename = "SubtypeId")]
    pub subtype_id: String,
}
