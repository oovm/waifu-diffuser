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
    pub async fn send_response(&self, user: &Uuid, response: DiffuserResponse, readable: bool) -> DiffuserResult<()> {
        let sender = match self.connections.get(&user) {
            Some(s) => s,
            None => {
                unimplemented!("")
            }
        };
        match readable {
            true => match to_string(&response) {
                Ok(o) => sender.sender.send(Message::Text(o)).await?,
                Err(_) => {
                    unimplemented!("")
                }
            },
            false => unimplemented!("Binary response not implemented"),
        }
        Ok(())
    }
}
