use super::*;

static SINGLETON: LazyLock<Arc<Mutex<StableDiffusionWorker>>> = LazyLock::new(|| {
    let worker = StableDiffusionWorker { queue: Default::default(), model: None, worker: None };
    Arc::new(Mutex::new(worker))
});

pub struct StableDiffusionWorker {
    queue: VecDeque<DiffuserTask>,
    model: Option<UNetModel>,
    worker: Option<StableDiffusionPipeline>,
}

impl StableDiffusionWorker {
    /// Load model from path.
    pub async fn load_model(env: &Arc<OrtEnvironment>, path: &Path) -> DiffuserResult<()> {
        let mut this = SINGLETON.lock().await;
        if this.is_same_model(path) {
            return Ok(());
        }
        this.do_load_model(env, path)
    }
    fn is_same_model(&self, path: &Path) -> bool {
        let _ = path;
        false
    }
    fn do_load_model(&mut self, env: &Arc<OrtEnvironment>, path: &Path) -> DiffuserResult<()> {
        // release memory
        self.model = None;
        self.worker = None;
        let loading = StableDiffusionPipeline::new(
            &env,
            path,
            StableDiffusionOptions { devices: DiffusionDeviceControl::all(cuda_device(0)), lpw: true },
        );
        match loading {
            Ok(o) => {
                self.worker = Some(o);
            }
            Err(e) => {
                unimplemented!("{}", e)
            }
        }
        Ok(())
    }
    /// Drop current model and release memory.
    pub async fn drop_model() {
        let mut this = SINGLETON.lock().await;
        this.model = None;
        this.worker = None;
    }
}

impl StableDiffusionWorker {
    pub fn spawn() -> JoinHandle<()> {
        tokio::task::spawn_blocking(move || {
            loop {
                match SINGLETON.try_lock() {
                    Ok(mut o) => match o.run() {
                        Ok(_) => {}
                        Err(e) => {
                            println!("Error: {}", e);
                        }
                    },
                    Err(_) => continue,
                };
            }
        })
    }
    fn run(&mut self) -> DiffuserResult<()> {
        match self.queue.pop_front() {
            Some(s) => match &s.task {
                DiffuserTaskKind::Text2Image(task) => self.run_text2img(task, s.task_id),
                _ => unimplemented!(),
            },
            None => Ok(()),
        }
    }
    fn run_text2img(&self, task: &Text2ImageTask, id: InsensitiveKey) -> DiffuserResult<()> {
        let config = StableDiffusionTxt2ImgOptions::default()
            .with_prompts(task.positive.as_str(), Some(task.negative.as_str()))
            .with_steps(task.steps)
            .with_size(task.width, task.height)
            .callback_decoded(1, {
                let task = task.clone();
                move |step, _, images| {
                    for (index, image) in images.iter().enumerate() {
                        let reply = match encode_png(image) {
                            Ok(o) => task.as_reply(step, index, o),
                            Err(_) => continue,
                        };
                    }
                    true
                }
            });
        let mut scheduler = DDIMScheduler::stable_diffusion_v1_optimized_default().unwrap();

        match config.run(self.worker.as_ref().unwrap(), &mut scheduler) {
            Ok(images) => {
                for (index, image) in images.iter().enumerate() {
                    let reply = match encode_png(image) {
                        Ok(o) => task.as_reply(task.steps, index, o),
                        Err(_) => continue,
                    };
                }
            }
            Err(_) => {
                unimplemented!()
            }
        }
        Ok(())
    }
}

fn encode_png(image: &DynamicImage) -> DiffuserResult<Vec<u8>> {
    let mut png = vec![];
    let encoder = PngEncoder::new(&mut png);
    encoder.write_image(image.to_rgb8().as_bytes(), image.width(), image.height(), ColorType::Rgb8).unwrap();
    Ok(png)
}
