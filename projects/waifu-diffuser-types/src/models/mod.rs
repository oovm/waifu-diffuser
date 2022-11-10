use crate::{ClipModel, DanBooruModel};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

pub mod clip;
pub mod deep_dan_booru;
mod reader;
pub mod unet;
pub mod vae;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiffuserModel {
    kind: ModelKind,
    path: PathBuf,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ModelKind {
    /// A ResNet model used for find prompts.
    DeepDanBooru(Box<DanBooruModel>),
    /// A VAE model used for stable diffusion 1.
    Vae,
    /// A U-Net model used for stable diffusion 1.
    UNet,
    /// A U-Net model used for stable diffusion 2.
    Clip(Box<ClipModel>),
}
