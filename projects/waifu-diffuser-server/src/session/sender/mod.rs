use crate::StableDiffusionWorker;
use waifu_diffuser_types::{DiffuserError, DiffuserResult, DiffuserTaskKind};
use crate::models::StableDiffusionWorker;

use super::*;

impl WaifuDiffuserSender {
    pub fn new(wss: WebSocketStream<TcpStream>, user: Uuid) -> (Self, SplitStream<WebSocketStream<TcpStream>>) {
        let (sender, receiver) = wss.split();
        (Self { user_id: user, shared: Arc::new(Mutex::new(sender)) }, receiver)
    }
    pub async fn send(&self, msg: Message) -> DiffuserResult<()> {
        Ok(self.shared.lock().await.send(msg).await?)
    }
    pub async fn emit_error(&self, error: DiffuserError, readable: bool) {
        match readable {
            true => {
                let text = serde_json::to_string(&error).unwrap();
                self.send(Message::Text(text)).await.ok();
            }
            false => {
                unimplemented!()
            }
        }
    }
    pub async fn emit_task(&mut self, task: DiffuserTaskKind, readable: bool) {
        match task {
            DiffuserTaskKind::Text2Image(e) => StableDiffusionWorker::instance().,
            DiffuserTaskKind::CollectLog(_e) => {}
        }
    }
}
