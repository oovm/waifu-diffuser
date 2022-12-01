use pyke_diffusers::StableDiffusionPipeline;
use std::sync::Arc;
use task_system::TaskSystem;
use tokio::sync::Mutex;
use waifu_diffuser_types::Text2ImageTask;

mod text2image;
