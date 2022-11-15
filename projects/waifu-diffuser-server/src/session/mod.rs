use std::{
    mem::MaybeUninit,
    net::SocketAddr,
    sync::{Mutex, Once},
    time::Duration,
};

use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use log::*;
use tokio::{
    net::TcpStream,
    time::{interval, Interval},
};
use tokio_tungstenite::{accept_async, accept_async_with_config, tungstenite::Error, WebSocketStream};
use tungstenite::{protocol::WebSocketConfig, Message, Result};

mod server;

mod context;

pub struct WaifuDiffuserServer {
    inner: Mutex<u8>,
}

pub struct WaifuDiffuserSession {
    ping: Interval,
    sender: SplitSink<WebSocketStream<TcpStream>, Message>,
    receiver: SplitStream<WebSocketStream<TcpStream>>,
}
