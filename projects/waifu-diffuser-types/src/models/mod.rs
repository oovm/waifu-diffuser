use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

mod reader;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiffuserModel {
    kind: ModelKind,
    path: PathBuf,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClipModel {
    name: String,
}
