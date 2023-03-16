use std::{path::Path, str::FromStr};

use log::info;
use tokio::net::{TcpListener, TcpStream};

use waifu_diffuser::{StableDiffusionWorker, WaifuDiffuserServer};
use waifu_diffuser_types::Uuid;

pub struct Application {}

#[tokio::main]
pub async fn main() {
    env_logger::init();
    let addr = "127.0.0.1:9527";
    let listener = TcpListener::bind(&addr).await.expect("Can't listen");
    info!("Listening on: {}", addr);
    let models = Path::new(env!("MODEL_DIR")).join("aom-v3.0-safe-fp16");
    tokio::spawn(async move {
        StableDiffusionWorker::instance().load_model(&models).await.expect("failed to load model");
    });
    StableDiffusionWorker::spawn();
    while let Ok((stream, _)) = listener.accept().await {
        accept_connection(stream).await;
    }
}

async fn accept_connection(stream: TcpStream) {
    let user = Uuid::from_str("5661458e-a187-4878-b8d8-22192ef9d7e3").unwrap();
    let readable = true;
    let server = WaifuDiffuserServer::instance();
    match server.connect(stream, user, readable).await {
        Ok(_) => {}
        Err(e) => {
            unimplemented!("{e}")
        }
    }
    match server.start(user).await {
        Ok(_) => {}
        Err(e) => {
            unimplemented!("{e}")
        }
    }
}
