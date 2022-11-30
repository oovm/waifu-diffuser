use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use crate::{ClipModel, DanBooruModel, UNetModel, VaeModel};

pub mod clip;
pub mod deep_dan_booru;
mod reader;
pub mod unet;
pub mod vae;

#[derive(Clone, Debug)]
pub struct DiffuserRunner {
    // models: DiffuserModel,
    // vae: Option<StableDiffusionConfig>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiffuserModel {
    kind: ModelKind,
    path: PathBuf,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EncodingMode {
    Bincode,
    Json,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ModelKind {
    /// A ResNet model used for find prompts.
    #[serde(rename = "deep_danbooru")]
    DeepDanBooru(Box<DanBooruModel>),
    /// A VAE model used for stable diffusion 1.
    #[serde(rename = "vae")]
    Vae(Box<VaeModel>),
    /// A U-Net model used for stable diffusion 1.
    #[serde(rename = "unet")]
    UNet(Box<UNetModel>),
    /// A Clip model used for stable diffusion 1.
    #[serde(rename = "clip")]
    Clip(Box<ClipModel>),
}
