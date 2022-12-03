use tokio::task::JoinHandle;

use waifu_diffuser_types::DiffuserResult;

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
        let (sender, receiver) = ws_stream.split();

        let session = WaifuDiffuserSession {
            user_id: user,
            sender: WaifuDiffuserSender { shared: Arc::new(Mutex::new(sender)) },
            receiver: WaifuDiffuserReceiver { shared: Arc::new(Mutex::new(receiver)) },
        };
        let sender = session.sender.clone();
        self.connections.insert(user, session);
        Ok(sender)
    }
    pub async fn start(&self, user: Uuid) -> bool {
        self.try_start(user).await.is_some()
    }
    async fn try_start(&self, user: Uuid) -> Option<()> {
        let connect = self.connections.get(&user)?;
        let send = connect.sender.clone();
        let recv = connect.receiver.clone();
        start_handler(send, recv).await.await.ok()
    }
}

pub async fn start_handler(sender: WaifuDiffuserSender, receiver: WaifuDiffuserReceiver) -> JoinHandle<()> {
    let mut interval = interval(Duration::from_millis(10000));
    tokio::spawn(async move {
        loop {
            tokio::select! {
                stop = receiver.on_receive(sender.clone()) => {
                    if stop {
                        break;
                    }
                }
                _ = interval.tick() => {
                    sender.do_ping().await
                }
            }
        }
    })
}
