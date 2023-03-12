use semver::Version;

use crate::{ResourcePath, Text2ImageTask};

use super::*;

mod der;

#[derive(Clone, Debug, Serialize)]
pub struct UNetModel {
    id: String,
    version: Version,
    net: ResourcePath,
    vae_encoder: Option<ResourcePath>,
    vae_decoder: Option<ResourcePath>,
    preview: Option<ResourcePath>,
    examples: Vec<Text2ImageTask>,
}

impl UNetModel {
    pub fn new<S: AsRef<str>>(id: S, path: ResourcePath) -> Self {
        let mut empty = Self {
            id: "".to_string(),
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
    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn set_id<S: AsRef<str>>(&mut self, id: S) {
        self.id = norm_id(id.as_ref());
    }
    pub fn with_id<S: AsRef<str>>(mut self, id: S) -> Self {
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

fn norm_id(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            ' ' | '_' | '-' => out.push('_'),
            _ => out.push(c.to_ascii_uppercase()),
        }
    }
    out
}
