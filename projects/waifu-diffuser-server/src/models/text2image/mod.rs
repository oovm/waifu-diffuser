use std::{path::Path, sync::LazyLock};

use pyke_diffusers::{
    DiffusionDeviceControl, EulerAncestralDiscreteScheduler, OrtEnvironment, SchedulerOptimizedDefaults,
    StableDiffusionOptions, StableDiffusionTxt2ImgOptions,
};
use tokio::{sync::MutexGuard, task::JoinHandle};

use waifu_diffuser_types::{DiffuserResult, UNetModel};

use crate::utils::cuda_device;

use super::*;

static SINGLETON: LazyLock<Arc<Mutex<StableDiffusionWorker>>> = LazyLock::new(|| {
    let worker = StableDiffusionWorker { model: None, worker: None };
    Arc::new(Mutex::new(worker))
});

pub struct StableDiffusionWorker {
    model: Option<UNetModel>,
    worker: Option<StableDiffusionPipeline>,
}

impl StableDiffusionWorker {
    // pub async fn instance() -> MutexGuard<'static, StableDiffusionWorker> {
    //     SINGLETON.lock().await
    // }
    pub async fn load_model(env: &Arc<OrtEnvironment>, path: &Path) -> DiffuserResult<()> {
        let mut this = SINGLETON.lock().await;
        if this.is_same_model(path) {
            return Ok(());
        }
        let loading = StableDiffusionPipeline::new(
            &env,
            path,
            StableDiffusionOptions { devices: DiffusionDeviceControl::all(cuda_device(0)), lpw: true },
        );
        match loading {
            Ok(o) => {
                this.worker = Some(o);
            }
            Err(e) => {
                unimplemented!("{}", e)
            }
        }
        Ok(())
    }
    fn is_same_model(&self, path: &Path) -> bool {
        let _ = path;
        false
    }
    pub async fn drop_model() {
        let mut this = SINGLETON.lock().await;
        this.model = None;
        this.worker = None;
    }
    pub fn spawn(&self) -> JoinHandle<()> {
        tokio::task::spawn_blocking(move || {
            loop {
                match SINGLETON.try_lock() {
                    Ok(o) => match o.worker.as_ref() {
                        Some(runner) => {
                            let task = StableDiffusionTxt2ImgOptions::default();
                            let mut scheduler = match EulerAncestralDiscreteScheduler::stable_diffusion_v1_optimized_default() {
                                Ok(o) => o,
                                Err(e) => {
                                    unimplemented!("{}", e)
                                }
                            };
                            let result = task.run(runner, &mut scheduler);
                        }
                        None => continue,
                    },
                    Err(_) => continue,
                };
            }
        })
    }
}

fn text2image(task: Text2ImageTask) {}
