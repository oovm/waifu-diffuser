use super::*;

impl WaifuDiffuserSender {
    pub async fn on_receive_texts(&self, text: String) -> bool {
        match serde_json::from_str::<DiffuserTask>(&text) {
            Ok(task) => self.emit_task(task).await,
            Err(e) => self.emit_error(DiffuserError::from(e), true).await,
        };
        false
    }
    pub async fn on_receive_bytes(&self, bytes: Vec<u8>) -> bool {
        let _ = bytes;
        unimplemented!();
    }
    pub async fn do_ping(&self) {
        let ping = "WaifuDiffuser".as_bytes().to_vec();
        if let Err(e) = self.send(Message::Ping(ping)).await {
            log::error!("Error sending ping: {}", e)
        }
    }
    pub async fn do_pong(&self, ping: Vec<u8>) -> bool {
        if let Err(e) = self.send(Message::Pong(ping)).await {
            log::error!("Error sending pong: {}", e)
        }
        false
    }
}

impl WaifuDiffuserSender {
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
    pub async fn emit_task(&self, task: DiffuserTask) {
        let result = match &task.body {
            DiffuserTaskKind::Text2Image(_) => StableDiffusionWorker::instance().accept_task(task).await,
            DiffuserTaskKind::CollectLog(_) => {
                unimplemented!()
            }
        };
        if let Err(result) = result {
            log::error!("{result}",)
        }
    }
}
