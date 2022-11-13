use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClipModel {
    key: String,
    name: String,
}

impl ClipModel {
    pub fn new(key: String, name: String) -> Self {
        Self { key, name }
    }
}