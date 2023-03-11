use std::{
    mem::MaybeUninit,
    sync::{Arc, Once},
    time::Duration,
};

use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use log::*;
use pyke_diffusers::{
    ArenaExtendStrategy, CUDADeviceOptions, CuDNNConvolutionAlgorithmSearch, DiffusionDevice, DiffusionDeviceControl,
    OrtEnvironment, SchedulerOptimizedDefaults, StableDiffusionOptions, StableDiffusionPipeline,
};
use tokio::{
    net::TcpStream,
    sync::Mutex,
    time::{interval, Interval},
};
use tokio_tungstenite::{accept_async_with_config, tungstenite::Error, WebSocketStream};
use tungstenite::{protocol::WebSocketConfig, Message, Result};

mod server;

mod context;

pub struct WaifuDiffuserServer {
    environment: Arc<OrtEnvironment>,
    diffuser: Mutex<Option<StableDiffusionPipeline>>,
}

pub struct WaifuDiffuserSession {
    ping: Interval,
    sender: SplitSink<WebSocketStream<TcpStream>, Message>,
    receiver: SplitStream<WebSocketStream<TcpStream>>,
}
