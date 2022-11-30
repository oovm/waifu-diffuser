use package_key::InsensitiveKey;
use semver::Version;

use crate::{ResourcePath, Text2ImageTask};

use super::*;

mod der;

#[derive(Clone, Debug, Serialize)]
pub struct UNetModel {
    model_id: InsensitiveKey,
    version: Version,
    net: ResourcePath,
    vae_encoder: Option<ResourcePath>,
    vae_decoder: Option<ResourcePath>,
    preview: Option<ResourcePath>,
    examples: Vec<Text2ImageTask>,
}

impl UNetModel {
    pub fn new(id: &str, path: ResourcePath) -> Self {
        let mut empty = Self {
            model_id: InsensitiveKey::new(""),
            version: Version::new(1, 5, 0),
            net: path,
            vae_encoder: None,
            vae_decoder: None,
            preview: None,
            examples: vec![],
        };
        empty.set_id(id);
        empty
    }
    pub fn get_id(&self) -> &InsensitiveKey {
        &self.model_id
    }
    pub fn set_id(&mut self, id: &str) {
        self.model_id = InsensitiveKey::new(id);
    }
    pub fn with_id(mut self, id: &str) -> Self {
        self.set_id(id);
        self
    }
    pub fn with_vae(mut self, encoder: Option<ResourcePath>, decoder: Option<ResourcePath>) -> Self {
        self.vae_encoder = encoder;
        self.vae_decoder = decoder;
        self
    }
    pub fn add_example(&mut self, example: Text2ImageTask) {
        self.examples.push(example);
    }
}
