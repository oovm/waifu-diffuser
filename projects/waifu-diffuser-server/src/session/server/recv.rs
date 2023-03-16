use super::*;

impl WaifuDiffuserReceiver {
    // return should break
    pub async fn on_receive(&self, sender: WaifuDiffuserSender) -> bool {
        let message = match self.shared.try_lock() {
            Ok(mut o) => match o.next().await {
                Some(Ok(s)) => s,
                Some(Err(e)) => {
                    log::error!("{e}");
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
