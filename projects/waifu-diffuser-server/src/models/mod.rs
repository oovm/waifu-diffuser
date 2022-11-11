use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ModelKind {
    /// A VAE model used for stable diffusion 1.
    Vae,
    /// A VAE model used for stable diffusion 2.
    Vae2,
    /// A U-Net model used for stable diffusion 1.
    UNet,
    /// A U-Net model used for stable diffusion 2.
    Clip,
    /// A CLIP model used for stable diffusion 2.
    Clip2,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DiffuserModel {
    /// A VAE model used for stable diffusion 1.
    Vae,
    /// A VAE model used for stable diffusion 2.
    Vae2,
    /// A U-Net model used for stable diffusion 1.
    UNet,
    /// A U-Net model used for stable diffusion 2.
    Clip,
    /// A CLIP model used for stable diffusion 2.
    Clip2,
}

impl DiffuserModel {
    pub fn new(path: PathBuf) -> Self {
        Self { kind: DiffuserModel::Clip, path }
    }
}

pub struct ClipModel {
    name: String,
    path: PathBuf
}

impl ClipModel {
    pub fn new(path: PathBuf) -> Self {
        Self { name: "".to_string(), path }
    }
}


