use super::*;

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
        info!("Received a text message from {}", text);
        if let Err(e) = self.sender.send(Message::Text(text.to_string())).await {
            error!("Error sending pong: {}", e)
        }
        false
    }
    async fn on_receive_bytes(&mut self, bytes: Vec<u8>) -> bool {
        info!("Received a bytes message from {:?}", bytes);
        if let Err(e) = self.sender.send(Message::Binary(bytes.to_vec())).await {
            error!("Error sending pong: {}", e)
        }
        false
    }
}
