use std::sync::mpsc::Sender;

use image::{codecs::png::PngEncoder, ColorType, DynamicImage, EncodableLayout, ImageEncoder};

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
