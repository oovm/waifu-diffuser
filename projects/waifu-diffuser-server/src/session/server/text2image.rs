use pyke_diffusers::{
    EulerDiscreteScheduler, OrtEnvironment, SchedulerOptimizedDefaults, StableDiffusionCallback, StableDiffusionPipeline,
    StableDiffusionTxt2ImgOptions,
};

use waifu_diffuser_types::{Text2ImageReply, Text2ImageTask};

use super::*;

impl WaifuDiffuserServer {
    pub fn text2image(&self, task: Text2ImageTask) {}
}

impl WaifuDiffuserSession {
    async fn emit_text2image(&mut self, task: Text2ImageTask, readable: bool) {
        let mut scheduler = EulerDiscreteScheduler::stable_diffusion_v1_optimized_default()?;
        let pipeline: &StableDiffusionPipeline;

        let imgs = pipeline.txt2img(
            "rust robot holding a torch",
            &mut scheduler,
            StableDiffusionTxt2ImgOptions {
                steps: 21,
                negative_prompt: None,
                callback: Some(StableDiffusionCallback::Decoded {
                    frequency: 3,
                    cb: Box::new(|index, timestamp, image| {
                        let image = image.first().unwrap();
                        println!("Generated {} images", index);
                        image.clone().into_rgb8().save(format!("target/result-{:03}.png", index)).unwrap();
                        true
                    }),
                }),
                ..Default::default()
            },
        )?;
        imgs[0].to_rgb8().save("target/result.png")?;
        Ok(())

        for i in 1..=task.step {




            let answer =
                Text2ImageReply { id: task.id.clone(), index: 0, step: i, width: task.width, height: task.height, png: vec![] };
            match readable {
                true => {
                    let text = serde_json::to_string(&answer.as_response()).unwrap();
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
}
