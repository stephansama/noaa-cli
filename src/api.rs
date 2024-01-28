use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, fmt::Debug};

#[derive(Debug, Deserialize, Serialize)]
pub struct DynamicJSON {
    #[serde(flatten)]
    pub dynamic_properties: HashMap<String, Value>,
}

// pub fn extract_values<T>(
//     json: &DynamicJSON,
//     property: String,
// ) -> Result<T, Box<dyn std::error::Error>>
// {
//     let result: T = serde_json::from_value(json.dynamic_properties[&property].clone())?;
//     Ok(T)
// }
