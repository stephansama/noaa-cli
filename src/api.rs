use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, fmt::Debug};

#[derive(Debug, Deserialize, Serialize)]
pub struct DynamicJSON {
    #[serde(flatten)]
    pub dynamic_properties: HashMap<String, Value>,
}
