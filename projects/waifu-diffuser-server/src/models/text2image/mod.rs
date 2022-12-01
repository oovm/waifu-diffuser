use std::{path::Path, sync::LazyLock};

use pyke_diffusers::{
    DiffusionDeviceControl, EulerAncestralDiscreteScheduler, OrtEnvironment, SchedulerOptimizedDefaults,
    StableDiffusionOptions, StableDiffusionTxt2ImgOptions,
};

use waifu_diffuser_types::DiffuserResult;

use crate::utils::cuda_device;

use super::*;

static SINGLETON: LazyLock<StableDiffusionWorker> = LazyLock::new(|| StableDiffusionWorker::default());

#[derive(Default)]
pub struct StableDiffusionWorker {
    worker: Arc<Mutex<Option<StableDiffusionPipeline>>>,
}

impl StableDiffusionWorker {
    pub fn instance() -> &'static Self {
        &SINGLETON
    }
    pub fn load_model(&self, env: &Arc<OrtEnvironment>, path: &Path) -> DiffuserResult<()> {
        let mut this = match self.worker.try_lock() {
            Ok(mut s) => {
                if self.same_model(path) {
                    return Ok(());
                }
                s
            }
            Err(e) => unimplemented!("{}", e),
        };
        let loading = StableDiffusionPipeline::new(
            &env,
            path,
            StableDiffusionOptions { devices: DiffusionDeviceControl::all(cuda_device(0)), lpw: true },
        );
        match loading {
            Ok(o) => {
                *this = Some(o);
                Ok(())
            }
            Err(e) => {
                unimplemented!("{}", e)
            }
        }
    }
    fn same_model(&self, path: &Path) -> bool {
        unimplemented!()
    }

    pub fn drop_model(&self) -> bool {
        match self.worker.try_lock() {
            Ok(s) => {
                *s = None;
                true
            }
            Err(_) => false,
        }
    }
    pub fn spawn(&self) -> bool {
        let worker = self.worker.clone();
        tokio::task::spawn_blocking(move || {
            loop {
                let runner = match worker.try_lock() {
                    Ok(o) => match o.as_ref() {
                        Some(o) => o,
                        None => continue,
                    },
                    Err(_) => continue,
                };
                let task = StableDiffusionTxt2ImgOptions::default();
                let mut scheduler = match EulerAncestralDiscreteScheduler::stable_diffusion_v1_optimized_default() {
                    Ok(o) => o,
                    Err(e) => {
                        unimplemented!("{}", e)
                    }
                };
                let result = task.run(runner, &mut scheduler);
            }
        });
        todo!()
    }
}

fn text2image(task: Text2ImageTask) {}
