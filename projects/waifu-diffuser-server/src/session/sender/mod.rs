use waifu_diffuser_types::DiffuserTask;

use super::*;

impl WaifuDiffuserReceiver {
    // return should break
    pub async fn on_receive(&self, sender: WaifuDiffuserSender) -> bool {
        let message = match self.shared.try_lock() {
            Ok(mut o) => match o.next().await {
                Some(Ok(s)) => s,
                Some(Err(e)) => {
                    unimplemented!("{e}");
                    return true;
                }
                None => return true,
            },
            Err(_) => {
                return false;
            }
        };
        match message {
            Message::Text(text) => sender.on_receive_texts(text).await,
            Message::Binary(bytes) => sender.on_receive_bytes(bytes).await,
            Message::Ping(v) => sender.do_pong(v).await,
            Message::Pong(_) => false,
            Message::Close(_) => true,
            Message::Frame(_) => false,
        }
    }
}

impl WaifuDiffuserSender {
    pub async fn on_receive_texts(&self, text: String) -> bool {
        match serde_json::from_str::<DiffuserTask>(&text) {
            Ok(task) => self.emit_task(task, true).await,
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
            error!("Error sending ping: {}", e)
        }
    }
    pub async fn do_pong(&self, ping: Vec<u8>) -> bool {
        if let Err(e) = self.send(Message::Pong(ping)).await {
            error!("Error sending pong: {}", e)
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
    pub async fn emit_task(&self, task: DiffuserTask, readable: bool) {
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
