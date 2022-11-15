use super::*;
use tokio::time::Interval;

impl WaifuDiffuserSession {
    pub async fn new(stream: TcpStream) -> std::result::Result<WaifuDiffuserSession, tungstenite::Error> {
        let peer = stream.peer_addr()?;
        info!("New WebSocket connection: {}", peer);
        let config = WebSocketConfig {
            max_send_queue: None,
            max_message_size: Some(64 << 20),
            max_frame_size: Some(16 << 20),
            accept_unmasked_frames: false,
        };
        let ws_stream = accept_async_with_config(stream, Some(config)).await?;
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();
        Ok(WaifuDiffuserSession { sender: ws_sender, receiver: ws_receiver })
    }
}

impl WaifuDiffuserSession {
    pub async fn start(&mut self) {
        let mut interval: Interval = interval(Duration::from_millis(10000));
        loop {
            tokio::select! {
                m = self.receiver.next() => {
                    if self.on_receive(m).await {
                        break;
                    }
                }
                _ = interval.tick() => {
                    self.sender.send(Message::Ping("WaifuDiffuser".as_bytes().to_vec())).await.ok();
                }
            }
        }
    }
    // return should break
    pub async fn on_receive(&mut self, message: Option<Result<Message, Error>>) -> bool {
        let message = match message {
            Some(Ok(msg)) => msg,
            Some(Err(e)) => {
                error!("Error processing connection: {}", e);
                return false;
            }
            None => return true,
        };
        match message {
            Message::Text(text) => {
                if let Err(e) = self.sender.send(Message::Text(v)).await {
                    error!("Error sending pong: {}", e)
                }
            }
            Message::Binary(bytes) => {
                if let Err(e) = self.sender.send(Message::Binary(v)).await {
                    error!("Error sending pong: {}", e)
                }
            }
            Message::Ping(v) => {
                if let Err(e) = self.sender.send(Message::Pong(v)).await {
                    error!("Error sending pong: {}", e)
                }
            }
            Message::Pong(_) => {}
            Message::Close(_) => return true,
            Message::Frame(_) => {}
        }
        false
    }
}
