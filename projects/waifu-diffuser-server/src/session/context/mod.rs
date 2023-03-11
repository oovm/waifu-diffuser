use waifu_diffuser_types::{DiffuserError, DiffuserTask, Text2ImageTask};

use super::*;

impl WaifuDiffuserSession {
    pub async fn new(stream: TcpStream) -> std::result::Result<WaifuDiffuserSession, tungstenite::Error> {
        let peer = stream.peer_addr()?;
        info!("New web socket connection: {}", peer);
        let config = WebSocketConfig {
            max_send_queue: None,
            max_message_size: Some(64 << 20),
            max_frame_size: Some(16 << 20),
            accept_unmasked_frames: false,
        };
        let ws_stream = accept_async_with_config(stream, Some(config)).await?;
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();
        let interval = interval(Duration::from_millis(10000));
        Ok(WaifuDiffuserSession { ping: interval, sender: ws_sender, receiver: ws_receiver })
    }
}

impl WaifuDiffuserSession {
    pub async fn start(&mut self) {
        loop {
            tokio::select! {
                m = self.receiver.next() => {
                    if self.on_receive(m).await {
                        break;
                    }
                }
                _ = self.ping.tick() => {
                    self.do_ping().await
                }
            }
        }
    }
    // return should break
    pub async fn on_receive(&mut self, message: Option<Result<Message, Error>>) -> bool {
        match message {
            Some(Ok(msg)) => match msg {
                Message::Text(text) => self.on_receive_texts(text).await,
                Message::Binary(bytes) => self.on_receive_bytes(bytes).await,
                Message::Ping(v) => self.do_pong(v).await,
                Message::Pong(_) => false,
                Message::Close(_) => true,
                Message::Frame(_) => false,
            },
            Some(Err(e)) => {
                error!("Error processing connection: {}", e);
                false
            }
            None => true,
        }
    }
    async fn do_ping(&mut self) {
        let ping = "WaifuDiffuser".as_bytes().to_vec();
        if let Err(e) = self.sender.send(Message::Ping(ping)).await {
            error!("Error sending ping: {}", e)
        }
    }
    async fn do_pong(&mut self, ping: Vec<u8>) -> bool {
        if let Err(e) = self.sender.send(Message::Pong(ping)).await {
            error!("Error sending pong: {}", e)
        }
        false
    }
    async fn on_receive_texts(&mut self, text: String) -> bool {
        match serde_json::from_str::<DiffuserTask>(&text) {
            Ok(task) => self.emit_task(task, true).await,
            Err(e) => self.emit_error(DiffuserError::from(e), true).await,
        };
        false
    }
    async fn on_receive_bytes(&mut self, bytes: Vec<u8>) -> bool {
        let _ = bytes;
        unimplemented!();
        false
    }
}

impl WaifuDiffuserSession {
    async fn emit_error(&mut self, error: DiffuserError, readable: bool) {
        let text = serde_json::to_string(&error).unwrap();
        if let Err(e) = self.sender.send(Message::Text(text)).await {
            error!("Error sending error: {}", e)
        }
    }

    async fn emit_task(&mut self, task: DiffuserTask, readable: bool) {
        match task {
            DiffuserTask::Text2Image(e) => {
                self.emit_text2image(*e, readable).await;
            }
            DiffuserTask::CollectLog(e) => {}
        }
    }
    async fn emit_text2image(&mut self, task: Text2ImageTask, readable: bool) {
        let text = serde_json::to_string(&task).unwrap();
        if let Err(e) = self.sender.send(Message::Text(text)).await {
            error!("Error sending task: {}", e)
        }
    }
}
