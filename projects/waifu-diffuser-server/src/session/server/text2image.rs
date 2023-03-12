use std::sync::{mpsc, mpsc::SendError};

use image::{codecs::png::PngEncoder, ColorType, DynamicImage, EncodableLayout, ImageEncoder};
use pyke_diffusers::{
    EulerDiscreteScheduler, Prompt, SchedulerOptimizedDefaults, StableDiffusionCallback, StableDiffusionTxt2ImgOptions,
};

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
            let options = StableDiffusionTxt2ImgOptions {
                steps: task.step,
                negative_prompt: Some(Prompt::from(task.negative.clone())),
                callback: Some(StableDiffusionCallback::Decoded {
                    frequency: 3,
                    cb: Box::new(move |steps, timestamp, image| {
                        for (index, image) in image.iter().enumerate() {
                            let png = match encode_png(image) {
                                Ok(png) => png,
                                Err(_) => continue,
                            };
                            let reply = task.reply_with(steps, index, png);
                            tx.clone().lock().unwrap().send(reply).ok();
                        }
                        true
                    }),
                }),
                ..Default::default()
            };
            // let imgs = pipeline.as_ref().unwrap().txt2img(task.positive.clone(), &mut scheduler, options).unwrap();
            // for (i, img) in imgs.iter().enumerate() {
            //     let png = match encode_png(img) {
            //         Ok(png) => png,
            //         Err(_) => continue,
            //     };
            //     let reply = task.reply_with(task.step, i, png);
            //     tx.clone().lock().unwrap().send(reply).ok();
            // }
            Ok::<(), SendError<Text2ImageReply>>(())
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
