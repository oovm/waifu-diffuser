use std::{
    sync::{Arc, LazyLock},
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

use waifu_diffuser_types::DiffuserTask;

mod server;

mod context;

pub static GLOBAL_RUNNER: LazyLock<WaifuDiffuserServer> = LazyLock::new(|| {
    let environment = OrtEnvironment::default().into_arc();
    WaifuDiffuserServer { environment, diffuser: Mutex::new(None) }
});

pub struct WaifuDiffuserServer {
    environment: Arc<OrtEnvironment>,
    diffuser: Mutex<Option<StableDiffusionPipeline>>,
}

pub struct WaifuDiffuserSession {
    ping: Interval,
    sender: SplitSink<WebSocketStream<TcpStream>, Message>,
    receiver: SplitStream<WebSocketStream<TcpStream>>,
}
