use super::*;

static SINGLETON: LazyLock<StableDiffusionWorker> = LazyLock::new(|| StableDiffusionWorker {
    //
    queue: Default::default(),
    model: Arc::new(Mutex::default()),
});

impl StableDiffusionWorker {
    pub fn instance() -> &'static StableDiffusionWorker {
        SINGLETON.deref()
    }
    /// Load model from path.
    pub async fn load_model(&self, env: &Arc<OrtEnvironment>, path: &Path) -> DiffuserResult<()> {
        let model = self.load_model_config(path).await?;
        // skip reload
        if self.already_loaded(&model).await {
            return Ok(());
        }
        // release memory
        self.drop_model();
        let loading = StableDiffusionPipeline::new(
            &env,
            path,
            StableDiffusionOptions { devices: DiffusionDeviceControl::all(cuda_device(0)), lpw: true },
        );
        match loading {
            Ok(o) => {
                *self.model.lock().await = Some(StableDiffusionInstance { model, worker: o });
            }
            Err(e) => {
                unimplemented!("{}", e)
            }
        }
        Ok(())
    }
    async fn already_loaded(&self, net: &UNetModel) -> bool {
        let model = self.model.lock().await;
        match model.as_ref() {
            Some(s) => s.model.get_id().eq(net.get_id()),
            None => false,
        }
    }
    async fn load_model_config(&self, path: &Path) -> DiffuserResult<UNetModel> {
        let _ = path;
        Ok(UNetModel::new("TEST", ResourcePath::new("DFF", "./sd").unwrap()))
    }
    /// Drop current model and release memory.
    pub async fn drop_model(&self) {
        *self.model.lock().await = None;
    }
}

impl StableDiffusionWorker {
    pub fn spawn(&self) -> JoinHandle<()> {
        tokio::task::spawn(async {
            loop {
                self.run()
            }
        })
    }
    async fn run(&self) -> Option<()> {
        // ensure model is loaded
        let task = self.queue.lock().await.pop_front()?;
        let model = self.model.lock().await;
        match &task.task {
            DiffuserTaskKind::Text2Image(kind) => {
                if let Err(e) = model.as_ref()?.run_text2img(kind, task.task_id, task.task_id) {
                    unimplemented!("{}", e)
                }
            }
            _ => unimplemented!(),
        }
        Some(())
    }
}

impl StableDiffusionInstance {
    pub fn run_text2img(&self, task: &Text2ImageTask, user_id: Uuid, task_id: Uuid) -> DiffuserResult<()> {
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
        match config.run(&self.worker, &mut scheduler) {
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
