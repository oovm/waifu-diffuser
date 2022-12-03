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
    StableDiffusionWorker::spawn().await.expect("Can't start stable diffusion worker");
    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream));
    }
}

async fn accept_connection(stream: TcpStream) {
    let user = Uuid::new_v4();
    let server = WaifuDiffuserServer::instance();
    match server.connect(stream, user).await {
        Ok(_) => {}
        Err(e) => {
            unimplemented!("{e}")
        }
    }
    server.start(user).await;
}
