use std::{
    collections::VecDeque,
    path::Path,
    sync::{Arc, LazyLock},
};

use image::{codecs::png::PngEncoder, ColorType, DynamicImage, EncodableLayout, ImageEncoder};
use pyke_diffusers::{
    DDIMScheduler, DiffusionDeviceControl, OrtEnvironment, SchedulerOptimizedDefaults, StableDiffusionOptions,
    StableDiffusionPipeline, StableDiffusionTxt2ImgOptions,
};
use tokio::{sync::Mutex, task::JoinHandle};

use waifu_diffuser_types::{DiffuserResult, DiffuserTask, DiffuserTaskKind, InsensitiveKey, Text2ImageTask, UNetModel};

use crate::utils::cuda_device;

pub mod text2image;
