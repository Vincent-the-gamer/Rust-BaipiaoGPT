use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct Message {
    pub role: String,
    pub content: String
}

impl Message {
    pub fn new(role: &str, content: &str) -> Self {
        Self { 
            role: String::from(role), 
            content: String::from(content) 
        }
    }
} 

impl From<Value> for Message {
    fn from(value: Value) -> Self {
        Message { 
            role: format!("{}",value["role"].as_str().unwrap()),
            content: format!("{}",value["content"].as_str().unwrap())
        }
    }
}

