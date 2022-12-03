use std::{ops::Deref, sync::LazyLock};

use serde_json::to_string;

use waifu_diffuser_types::{DiffuserResponse, DiffuserResult};

use super::*;

mod text2image;

static SINGLETON: LazyLock<WaifuDiffuserServer> = LazyLock::new(|| WaifuDiffuserServer {
    environment: OrtEnvironment::default().into_arc(),
    connections: Default::default(),
});

impl WaifuDiffuserServer {
    pub fn instance() -> &'static WaifuDiffuserServer {
        SINGLETON.deref()
    }
    pub fn send_message(&self, user: &Uuid, message: Message) -> Option<bool> {
        let sender = self.connections.get(&user)?;
        sender.send_message(message)
    }
    pub fn send_response(&self, user: &Uuid, response: DiffuserResponse, readable: bool) -> DiffuserResult<()> {
        let sender = match self.connections.get(&user) {
            Some(s) => s,
            None => {
                unimplemented!("")
            }
        };
        match readable {
            true => match to_string(&response) {
                Ok(o) => sender.sender.send(Message::Text(o))?,
                Err(_) => {
                    unimplemented!("")
                }
            },
            false => unimplemented!("Binary response not implemented"),
        }
    }
}
