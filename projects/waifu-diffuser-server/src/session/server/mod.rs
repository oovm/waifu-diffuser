use super::*;

static mut SINGLETON: MaybeUninit<WaifuDiffuserServer> = MaybeUninit::uninit();
static ONCE: Once = Once::new();

impl WaifuDiffuserServer {
    pub fn singleton() -> &'static WaifuDiffuserServer {
        let environment = OrtEnvironment::default().into_arc();
        unsafe {
            ONCE.call_once(|| {
                let singleton = WaifuDiffuserServer { diffuser: Mutex::new(None) };
                SINGLETON.write(singleton);
            });
            SINGLETON.assume_init_ref()
        }
    }
    pub async fn load_diffuser(&self) {
        let mut diffuser = self.diffuser.lock().await;

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
        *diffuser = Some(pipeline);
    }
}
