use std::{
    collections::VecDeque,
    ops::Deref,
    path::Path,
    sync::{Arc, LazyLock},
};

use image::{codecs::png::PngEncoder, ColorType, DynamicImage, EncodableLayout, ImageEncoder};
use pyke_diffusers::{
    DDIMScheduler, DiffusionDeviceControl, OrtEnvironment, SchedulerOptimizedDefaults, StableDiffusionOptions,
    StableDiffusionPipeline, StableDiffusionTxt2ImgOptions,
};
use tokio::{sync::Mutex, task::JoinHandle};
use uuid::Uuid;

use waifu_diffuser_types::{DiffuserResult, DiffuserTask, DiffuserTaskKind, ResourcePath, Text2ImageTask, UNetModel};

use crate::{utils::cuda_device, WaifuDiffuserServer};

pub mod text2image;

pub struct StableDiffusionWorker {
    queue: Arc<Mutex<VecDeque<DiffuserTask>>>,
    model: Arc<Mutex<Option<StableDiffusionInstance>>>,
}

struct StableDiffusionInstance {
    model: UNetModel,
    worker: StableDiffusionPipeline,
}
