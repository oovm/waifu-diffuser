use image::{codecs::png::PngEncoder, ColorType, DynamicImage, EncodableLayout, ImageEncoder};
use pyke_diffusers::{
    EulerDiscreteScheduler, Prompt, SchedulerOptimizedDefaults, StableDiffusionCallback, StableDiffusionPipeline,
    StableDiffusionTxt2ImgOptions,
};

use waifu_diffuser_types::{Text2ImageReply, Text2ImageTask};

use super::*;

impl WaifuDiffuserServer {
    pub fn text2image(&self, task: Text2ImageTask) {}
}

impl WaifuDiffuserSession {
    pub(super) async fn emit_text2image(&mut self, task: Text2ImageTask, readable: bool) {
        let run =
            GLOBAL_RUNNER.load_diffuser("source", async move |pipeline| self.run_text2image(task, pipeline, readable)).await;
        match run {
            Ok(_) => {}
            Err(_) => {}
        }
    }

    async fn run_text2image(
        &mut self,
        task: Text2ImageTask,
        pipeline: &StableDiffusionPipeline,
        readable: bool,
    ) -> DiffuserResult<()> {
        let mut scheduler = EulerDiscreteScheduler::stable_diffusion_v1_optimized_default().unwrap();
        let imgs = pipeline.txt2img(
            task.positive,
            &mut scheduler,
            StableDiffusionTxt2ImgOptions {
                steps: task.step,
                negative_prompt: Some(Prompt::from(task.negative)),
                callback: Some(StableDiffusionCallback::Decoded {
                    frequency: 3,
                    cb: Box::new(|steps, timestamp, image| async {
                        for (index, image) in image.iter().enumerate() {
                            let png = match encode_png(image) {
                                Ok(png) => png,
                                Err(_) => continue,
                            };
                            self.relpy_task2image(
                                Text2ImageReply {
                                    id: task.id.clone(),
                                    index,
                                    step: steps,
                                    width: task.width,
                                    height: task.height,
                                    png,
                                },
                                readable,
                            )
                            .await;
                        }
                        true
                    }),
                }),
                ..Default::default()
            },
        )?;
        for (i, img) in imgs.iter().enumerate() {
            let png = match encode_png(img) {
                Ok(png) => png,
                Err(_) => continue,
            };
            self.relpy_task2image(
                Text2ImageReply { id: task.id.clone(), index: i, step: task.step, width: task.width, height: task.height, png },
                readable,
            )
            .await;
        }
        Ok(())
    }
    async fn relpy_task2image(&mut self, reply: Text2ImageReply, readable: bool) {
        match readable {
            true => {
                let text = serde_json::to_string(&reply.as_response()).unwrap();
                if let Err(e) = self.sender.send(Message::Text(text)).await {
                    error!("Error sending task: {}", e)
                }
            }
            false => {
                unimplemented!()
            }
        }
    }
}

fn encode_png(image: &DynamicImage) -> DiffuserResult<Vec<u8>> {
    let mut png = vec![];
    let encoder = PngEncoder::new(&mut png);
    encoder.write_image(image.to_rgb8().as_bytes(), image.width(), image.height(), ColorType::Rgb8)?;
    Ok(png)
}
