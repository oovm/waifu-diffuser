use log::info;
use tokio::net::{TcpListener, TcpStream};

use waifu_diffuser::{StableDiffusionWorker, WaifuDiffuserSession};

pub struct Application {}

#[tokio::main]
pub async fn main() {
    env_logger::init();
    let addr = "127.0.0.1:9527";
    let listener = TcpListener::bind(&addr).await.expect("Can't listen");
    info!("Listening on: {}", addr);
    StableDiffusionWorker::spawn();
    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream));
    }
}

async fn accept_connection(stream: TcpStream) {
    let mut session = WaifuDiffuserSession::new(stream).await.unwrap();
    session.start().await;
}
