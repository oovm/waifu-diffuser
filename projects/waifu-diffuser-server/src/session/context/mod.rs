use tokio::task::JoinHandle;

use waifu_diffuser_types::{DiffuserError, DiffuserResult, DiffuserTaskKind};

use super::*;

impl WaifuDiffuserServer {
    pub async fn connect(&self, stream: TcpStream, user: Uuid) -> DiffuserResult<WaifuDiffuserSender> {
        let peer = stream.peer_addr()?;
        info!("New web socket connection: {}", peer);
        let config = WebSocketConfig {
            max_send_queue: None,
            max_message_size: Some(64 << 20),
            max_frame_size: Some(16 << 20),
            accept_unmasked_frames: false,
        };
        let ws_stream = accept_async_with_config(stream, Some(config)).await?;
        let (sender, receiver) = WaifuDiffuserSender::new(ws_stream, user.clone());
        // let interval = interval(Duration::from_millis(10000));
        let session = WaifuDiffuserSession { user_id: user, receiver, sender };
        let sender = session.sender.clone();
        self.connections.insert(user, session);
        Ok(sender)
    }
}

impl WaifuDiffuserSession {
    pub async fn start(&mut self) -> JoinHandle<()> {
        let interval = interval(Duration::from_millis(10000));
        tokio::spawn(async {
            loop {
                tokio::select! {
                    m = self.receiver.next() => {
                        if self.on_receive(m).await {
                            break;
                        }
                    }
                    _ = interval.tick() => {
                        self.do_ping().await
                    }
                }
            }
        })
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
    pub async fn send_message(&self, message: Message) -> DiffuserResult<()> {
        let mut sender = self.sender.shared.lock().await;
        sender.send(message).await?;
        Ok(())
    }
    async fn do_ping(&self) {
        let ping = "WaifuDiffuser".as_bytes().to_vec();
        if let Err(e) = self.send_message(Message::Ping(ping)).await {
            error!("Error sending ping: {}", e)
        }
    }
    async fn do_pong(&self, ping: Vec<u8>) -> bool {
        if let Err(e) = self.send_message(Message::Pong(ping)).await {
            error!("Error sending pong: {}", e)
        }
        false
    }
    async fn on_receive_texts(&mut self, text: String) -> bool {
        match serde_json::from_str::<DiffuserTaskKind>(&text) {
            Ok(task) => self.sender.emit_task(task, true).await,
            Err(e) => self.sender.emit_error(DiffuserError::from(e), true).await,
        };
        false
    }
    async fn on_receive_bytes(&mut self, bytes: Vec<u8>) -> bool {
        let _ = bytes;
        unimplemented!();
        false
    }
}
