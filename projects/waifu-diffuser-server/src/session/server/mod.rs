use std::{path::Path, sync::LazyLock};

use waifu_diffuser_types::DiffuserResult;

use super::*;

mod text2image;

pub static GLOBAL_RUNNER: LazyLock<WaifuDiffuserServer> = LazyLock::new(|| {
    let environment = OrtEnvironment::default().into_arc();
    WaifuDiffuserServer { environment, diffuser: Mutex::new(None) }
});

impl WaifuDiffuserServer {
    pub async fn drop_diffuser(&self) {
        let mut diffuser = self.diffuser.lock().await;
        *diffuser = None;
    }
    pub async fn load_diffuser(&self, config: &Path) -> DiffuserResult<&StableDiffusionPipeline> {
        let guard = self.diffuser.try_lock().unwrap();
        if let Some(diffuser) = guard.as_ref() {
            return Ok(diffuser);
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
            config,
            StableDiffusionOptions {
                devices: DiffusionDeviceControl { unet: cuda, ..Default::default() },
                ..Default::default()
            },
        )?;

        *diffuser = Some(pipeline);
        Ok(diffuser.as_ref().unwrap())
    }
}
