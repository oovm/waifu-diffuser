use std::{sync::Arc, time::Duration};

use dashmap::DashMap;
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use log::*;
use pyke_diffusers::OrtEnvironment;
use tokio::{net::TcpStream, sync::Mutex, time::interval};
use tokio_tungstenite::{accept_async_with_config, tungstenite::Error, WebSocketStream};
use tungstenite::{protocol::WebSocketConfig, Message, Result};
use uuid::Uuid;

mod context;
mod sender;
mod server;

pub struct WaifuDiffuserServer {
    environment: Arc<OrtEnvironment>,
    connections: DashMap<Uuid, WaifuDiffuserSession>,
}

#[derive(Default, Clone, Debug)]
pub struct WaifuDiffuserServerConfig {
    ping_interval: Duration,
    ping_message: String,
}

pub struct WaifuDiffuserSession {
    user_id: Uuid,
    receiver: SplitStream<WebSocketStream<TcpStream>>,
    sender: WaifuDiffuserSender,
}

#[derive(Clone)]
pub struct WaifuDiffuserSender {
    user_id: Uuid,
    shared: Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>,
}
