use std::sync::{mpsc, mpsc::Sender};

use image::{codecs::png::PngEncoder, ColorType, DynamicImage, EncodableLayout, ImageEncoder};
use pyke_diffusers::{EulerDiscreteScheduler, SchedulerOptimizedDefaults, StableDiffusionTxt2ImgOptions};

use waifu_diffuser_types::{Text2ImageReply, Text2ImageTask};

use super::*;

impl WaifuDiffuserServer {
    pub fn text2image(&self, task: Text2ImageTask) {}
}

impl WaifuDiffuserSession {
    pub async fn emit_text2image(&mut self, task: Text2ImageTask, readable: bool) {
        if let Err(err) = self.run_text2image(task, readable).await {
            log::error!("Error: {:?}", err);
        }
    }
    async fn run_text2image(&mut self, task: Text2ImageTask, readable: bool) -> DiffuserResult<()> {
        let (tx, rx) = mpsc::channel();
        let tx = std::sync::Arc::new(std::sync::Mutex::new(tx));
        let mut scheduler = EulerDiscreteScheduler::stable_diffusion_v1_optimized_default()?;
        let pipeline = GLOBAL_RUNNER.load_diffuser().await?;
        std::thread::spawn(move || {
            let step = task.step;
            let options = StableDiffusionTxt2ImgOptions::default()
                .with_steps(task.step)
                .with_prompts(task.positive.as_str(), Some(task.negative.as_str()))
                .callback_decoded(1, move |steps, timestamp, image| {
                    send_channel(tx.clone(), task.clone(), steps, image);
                    true
                });
            let imgs = options.run(&pipeline.as_ref().unwrap(), &mut scheduler).unwrap();
            // send_channel(tx, task, step, imgs);
        });
        for item in rx {
            self.send_task2image(item, readable).await;
        }
        Ok(())
    }
    async fn send_task2image(&mut self, reply: Text2ImageReply, readable: bool) {
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
    encoder.write_image(image.to_rgb8().as_bytes(), image.width(), image.height(), ColorType::Rgb8).unwrap();
    Ok(png)
}

fn send_channel(
    channel: Arc<std::sync::Mutex<Sender<Text2ImageReply>>>,
    task: Text2ImageTask,
    steps: usize,
    image: Vec<DynamicImage>,
) {
    for (index, image) in image.iter().enumerate() {
        let png = match encode_png(image) {
            Ok(png) => png,
            Err(_) => continue,
        };
        let reply = task.as_reply(steps, index, png);
        match channel.clone().lock() {
            Ok(o) => {
                o.send(reply).ok();
            }
            Err(_) => {}
        }
    }
}
