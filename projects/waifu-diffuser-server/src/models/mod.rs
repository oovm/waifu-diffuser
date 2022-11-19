use std::{mem::MaybeUninit, sync::Once};

use image::DynamicImage;
use pyke_diffusers::{
    ArenaExtendStrategy, CUDADeviceOptions, CuDNNConvolutionAlgorithmSearch, DiffusionDevice, DiffusionDeviceControl,
    EulerDiscreteScheduler, OrtEnvironment, SchedulerOptimizedDefaults, StableDiffusionCallback, StableDiffusionOptions,
    StableDiffusionPipeline, StableDiffusionTxt2ImgOptions,
};
use tokio::sync::Mutex;

use crate::WaifuDiffuserServer;

fn main() -> anyhow::Result<()> {
    let environment = OrtEnvironment::default().into_arc();
    let mut scheduler = EulerDiscreteScheduler::stable_diffusion_v1_optimized_default()?;
    let cuda = DiffusionDevice::CUDA(
        0,
        Some(CUDADeviceOptions {
            memory_limit: None,
            arena_extend_strategy: Some(ArenaExtendStrategy::SameAsRequested),
            cudnn_conv_algorithm_search: Some(CuDNNConvolutionAlgorithmSearch::Exhaustive),
        }),
    );
    let pipeline = StableDiffusionPipeline::new(
        &environment,
        "./pyke-diffusers-sd15-fp16/",
        StableDiffusionOptions {
            devices: DiffusionDeviceControl { unet: cuda.clone(), ..Default::default() },
            ..Default::default()
        },
    )?;

    let imgs = pipeline.txt2img(
        "rust robot holding a torch",
        &mut scheduler,
        StableDiffusionTxt2ImgOptions {
            steps: 21,
            negative_prompt: None,
            callback: Some(StableDiffusionCallback::ApproximateDecoded { frequency: 3, cb: Box::new(save_image) }),
            ..Default::default()
        },
    )?;
    imgs[0].to_rgb8().save("target/result.png")?;
    Ok(())
}

fn save_image(index: usize, timestamp: f32, image: Vec<DynamicImage>) -> bool {
    let image = image.first().unwrap();
    println!("Generated {} images", index);
    image.clone().into_rgb8().save(format!("target/result-{:03}.png", index)).unwrap();
    true
}
