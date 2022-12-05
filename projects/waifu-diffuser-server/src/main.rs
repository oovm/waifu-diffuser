use std::path::Path;

use log::info;
use tokio::net::{TcpListener, TcpStream};
use uuid::Uuid;

use waifu_diffuser::{StableDiffusionWorker, WaifuDiffuserServer};

pub struct Application {}

#[tokio::main]
pub async fn main() {
    env_logger::init();
    let addr = "127.0.0.1:9527";
    let listener = TcpListener::bind(&addr).await.expect("Can't listen");
    info!("Listening on: {}", addr);
    let models = Path::new(env!("MODEL_DIR")).join("aom-v3.0-safe-fp16");
    StableDiffusionWorker::instance().load_model(&models).await.expect("failed to load model");
    StableDiffusionWorker::spawn();
    while let Ok((stream, _)) = listener.accept().await {
        accept_connection(stream).await;
    }
}

async fn accept_connection(stream: TcpStream) {
    let user = Uuid::new_v4();
    let server = WaifuDiffuserServer::instance();
    match server.connect(stream, user).await {
        Ok(_) => {
            info!("check point2");
        }
        Err(e) => {
            unimplemented!("{e}")
        }
    }
    match server.start(user).await {
        Ok(_) => {
            info!("check point3");
        }
        Err(e) => {
            unimplemented!("{e}")
        }
    }
}
