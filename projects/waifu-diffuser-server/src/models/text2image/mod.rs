use super::*;
use pyke_diffusers::EulerDiscreteScheduler;

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
    pub async fn load_model(&self, config_path: &Path) -> DiffuserResult<()> {
        tracing::info!("Loading model from {}", config_path.canonicalize()?.display());
        let model = self.load_model_config(config_path).await?;
        // skip reload
        if self.already_loaded(&model).await {
            return Ok(());
        }
        // release memory
        self.drop_model().await;
        let loading = StableDiffusionPipeline::new(
            &WaifuDiffuserServer::environment(),
            config_path,
            StableDiffusionOptions { devices: DiffusionDeviceControl::all(cuda_device(0)), lpw: true },
        );
        match loading {
            Ok(o) => {
                tracing::info!("Model loaded");
                *self.model.lock().await = Some(StableDiffusionInstance { model, worker: Arc::new(o) });
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
        // let json = File::open(path)?;
        Ok(UNetModel::new("test", ResourcePath::new("https://example.json", "./a").unwrap()))
    }
    /// Drop current model and release memory.
    pub async fn drop_model(&self) {
        *self.model.lock().await = None;
    }
}

impl StableDiffusionWorker {
    pub fn spawn() -> JoinHandle<()> {
        tracing::info!("Stable diffusion worker awake");
        tokio::spawn(async {
            loop {
                StableDiffusionWorker::instance().run().await;
            }
        })
    }
    async fn run(&self) -> Option<()> {
        // ensure model is loaded
        let task = self.queue.lock().await.pop_front()?;
        let model = match self.model.lock().await.as_ref() {
            Some(s) => s.worker.clone(),
            None => return Some(()),
        };
        match &task.body {
            DiffuserTaskKind::Text2Image(kind) => match run_text2img(model, kind, task.user_id, task.task_id).await {
                Ok(_) => {}
                Err(e) => {
                    unimplemented!("{}", e)
                }
            },
            _ => unimplemented!(),
        }
        Some(())
    }
    pub async fn accept_task(&self, task: DiffuserTask) -> DiffuserResult<()> {
        self.queue.lock().await.push_back(task);
        Ok(())
    }
}

pub async fn run_text2img(
    worker: Arc<StableDiffusionPipeline>,
    task: &Text2ImageTask,
    user_id: Uuid,
    task_id: Uuid,
) -> DiffuserResult<()> {
    let task = task.clone();
    let (tx, rx) = std::sync::mpsc::channel();
    let tx = Arc::new(std::sync::Mutex::new(tx));
    tokio::task::spawn_blocking(move || {
        let config = StableDiffusionTxt2ImgOptions::default()
            .with_prompts(task.positive.as_str(), Some(task.negative.as_str()))
            .with_steps(task.steps)
            .with_size(task.width, task.height)
            .callback_decoded(1, {
                let tx = tx.clone();
                let task = task.clone();
                move |step, _, images| {
                    for (index, image) in images.iter().enumerate() {
                        let reply = match encode_png(image) {
                            Ok(o) => task.as_reply(task_id, step, index, o),
                            Err(_) => continue,
                        };
                        tx.lock().unwrap().send(reply).ok();
                    }
                    true
                }
            });
        let mut scheduler = EulerDiscreteScheduler::stable_diffusion_v1_optimized_default().unwrap();
        match config.run(&worker, &mut scheduler) {
            Ok(images) => {
                for (index, image) in images.iter().enumerate() {
                    let reply = match encode_png(image) {
                        Ok(o) => task.as_reply(task_id, task.steps, index, o),
                        Err(_) => continue,
                    };
                    tx.lock().unwrap().send(reply).ok();
                }
            }
            Err(_) => {
                unimplemented!()
            }
        }
    });
    for image in rx {
        WaifuDiffuserServer::instance().send_response(&user_id, image).await.ok();
    }
    Ok(())
}

fn encode_png(image: &DynamicImage) -> DiffuserResult<Vec<u8>> {
    let mut png = vec![];
    let encoder = PngEncoder::new(&mut png);
    encoder.write_image(image.to_rgb8().as_bytes(), image.width(), image.height(), ColorType::Rgb8).unwrap();
    Ok(png)
}
