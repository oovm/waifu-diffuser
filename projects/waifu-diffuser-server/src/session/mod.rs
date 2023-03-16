use std::{
    ops::Deref,
    sync::{Arc, LazyLock},
    time::Duration,
};

use dashmap::DashMap;
use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use pyke_diffusers::OrtEnvironment;
use serde_json::to_string;
use tokio::{net::TcpStream, sync::Mutex, time::interval};
use tokio_tungstenite::{accept_async_with_config, WebSocketStream};
use tungstenite::{protocol::WebSocketConfig, Message};
use waifu_diffuser_types::Uuid;

use waifu_diffuser_types::{DiffuserError, DiffuserResponse, DiffuserResult, DiffuserTask, DiffuserTaskKind};

use crate::StableDiffusionWorker;

mod config;
mod context;
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
    readable: bool,
    receiver: WaifuDiffuserReceiver,
    sender: WaifuDiffuserSender,
}

#[derive(Clone)]
pub struct WaifuDiffuserReceiver {
    shared: Arc<Mutex<SplitStream<WebSocketStream<TcpStream>>>>,
}

#[derive(Clone)]
pub struct WaifuDiffuserSender {
    shared: Arc<Mutex<SplitSink<WebSocketStream<TcpStream>, Message>>>,
}
