use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

mod reader;

pub struct DiffuserModel {
    kind: ModelKind,
    path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ModelKind {
    /// A VAE model used for stable diffusion 1.
    Vae,
    /// A VAE model used for stable diffusion 2.
    Vae2,
    /// A U-Net model used for stable diffusion 1.
    UNet,
    /// A U-Net model used for stable diffusion 2.
    Clip(Box<ClipModel>),
    /// A CLIP model used for stable diffusion 2.
    Clip2,
}

impl DiffuserModel {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let path = path.as_ref().canonicalize().unwrap();
        todo!()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClipModel {
    name: String,
}
