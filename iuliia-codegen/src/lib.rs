use serde::{Serialize, Deserialize};
use serde_json::{Map, Value};

#[derive(Serialize, Deserialize)]
pub struct Schema {
    pub name: String,
    pub description: String,
    pub url: String,
    pub comments: Vec<String>,
    pub mapping: Option<Map<String, Value>>,
    pub prev_mapping: Option<Map<String, Value>>,
    pub next_mapping: Option<Map<String, Value>>,
    pub ending_mapping: Option<Map<String, Value>>,
    pub samples: Vec<(String, String)>
}