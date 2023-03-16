use super::*;

mod recv;
mod send;

static SINGLETON: LazyLock<WaifuDiffuserServer> = LazyLock::new(|| WaifuDiffuserServer {
    environment: OrtEnvironment::default().into_arc(),
    connections: Default::default(),
});

impl WaifuDiffuserServer {
    pub fn instance() -> &'static WaifuDiffuserServer {
        SINGLETON.deref()
    }
    pub fn environment() -> Arc<OrtEnvironment> {
        SINGLETON.environment.clone()
    }
    pub async fn send_message(&self, user: &Uuid, message: Message) -> DiffuserResult<()> {
        match self.connections.get(&user) {
            None => {
                unimplemented!()
            }
            Some(s) => {
                let mut sender = s.sender.shared.lock().await;
                sender.send(message).await?;
            }
        }

        Ok(())
    }
    pub async fn send_response(&self, user: &Uuid, response: DiffuserResponse) -> DiffuserResult<()> {
        let session = match self.connections.get(&user) {
            Some(s) => s,
            None => Err(DiffuserError::custom_error("User not found", -1004))?,
        };
        match session.readable {
            true => match serde_json::to_string(&response) {
                Ok(o) => session.sender.send(Message::Text(o)).await?,
                Err(_) => {
                    unimplemented!("")
                }
            },
            false => unimplemented!("Binary response not implemented"),
        }
        Ok(())
    }
}
