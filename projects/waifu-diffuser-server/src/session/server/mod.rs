use std::{future::Future, path::Path};

use waifu_diffuser_types::DiffuserResult;

use super::*;

mod text2image;

impl WaifuDiffuserServer {
    pub async fn drop_diffuser(&self) {
        let mut diffuser = self.diffuser.lock().await;
        *diffuser = None;
    }
    pub async fn load_diffuser<P, T, F>(&self, load: P) -> DiffuserResult<&StableDiffusionPipeline>
    where
        P: AsRef<Path>,
        T: Future<Output = F>,
        F: Fn(&StableDiffusionPipeline) -> DiffuserResult<()>,
    {
        let mut guard = self.diffuser.try_lock().unwrap();
        if let Some(diffuser) = guard.as_ref() {
            return f(diffuser);
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
            load.as_ref(),
            StableDiffusionOptions {
                devices: DiffusionDeviceControl { unet: cuda, ..Default::default() },
                ..Default::default()
            },
        )?;
        *guard = Some(pipeline);
        f(guard.as_ref().unwrap())
    }
}
