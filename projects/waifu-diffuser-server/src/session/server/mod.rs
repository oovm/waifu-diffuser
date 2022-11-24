use std::{future::Future, path::Path};
use tokio::sync::MutexGuard;

use waifu_diffuser_types::DiffuserResult;

use super::*;

mod text2image;

impl WaifuDiffuserServer {
    pub async fn drop_diffuser(&self) {
        let mut diffuser = self.diffuser.lock().await;
        *diffuser = None;
    }
    pub async fn load_diffuser(&self) -> DiffuserResult<MutexGuard<Option<StableDiffusionPipeline>>> {
        let mut guard = self.diffuser.try_lock().unwrap();
        if let Some(diffuser) = guard.as_ref() {
            return Ok(guard);
        }
        let cuda = DiffusionDevice::CUDA(
            0,
            Some(CUDADeviceOptions {
                memory_limit: None,
                arena_extend_strategy: Some(ArenaExtendStrategy::SameAsRequested),
                cudnn_conv_algorithm_search: Some(CuDNNConvolutionAlgorithmSearch::Exhaustive),
            }),
        );
        let pipeline = StableDiffusionPipeline::new(
            &self.environment,
            "./stable-diffusion-v1-5/",
            StableDiffusionOptions {
                devices: DiffusionDeviceControl { unet: cuda, ..Default::default() },
                ..Default::default()
            },
        )?;
        *guard = Some(pipeline);
        Ok(guard)
    }
}
