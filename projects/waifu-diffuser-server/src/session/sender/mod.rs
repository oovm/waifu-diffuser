use waifu_diffuser_types::{DiffuserError, DiffuserTaskKind};

use super::*;

impl WaifuDiffuserSender {
    pub fn new(wss: WebSocketStream<TcpStream>) -> (Self, SplitStream<WebSocketStream<TcpStream>>) {
        let (sender, receiver) = wss.split();
        (Self { shared: Arc::new(Mutex::new(sender)) }, receiver)
    }
    pub async fn emit_error(&mut self, error: DiffuserError, readable: bool) {
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
            DiffuserTaskKind::Text2Image(e) => {
                self.emit_text2image(*e, readable).await;
            }
            DiffuserTaskKind::CollectLog(_e) => {}
        }
    }
}
