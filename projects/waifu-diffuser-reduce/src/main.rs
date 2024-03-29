use futures_util::{
    stream::{Next, SplitStream},
    SinkExt, StreamExt,
};
use log::*;
use std::{net::SocketAddr, time::Duration};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Error, WebSocketStream};
use tungstenite::{Message, Result};

async fn accept_connection(peer: SocketAddr, stream: TcpStream) {
    if let Err(e) = handle_connection(peer, stream).await {
        match e {
            Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
            err => error!("Error processing connection: {}", err),
        }
    }
}

pub struct WaifuDiffuserServer {}

impl WaifuDiffuserServer {
    pub fn new() -> Self {
        Self {}
    }
    pub async fn on_receive(&self, message: Option<Result<Message, Error>>) -> bool {
        match message {
            None => {}
            Some(s) => {}
        }
    }
}

async fn receive_handle(context: &mut WaifuDiffuserServer, message: Option<Result<Message, Error>>) -> Option<bool> {
    match message? {
        Ok(_) => {}
        Err(s) => {
            error!("Error processing connection: {}", s);
        }
    }
}

pub struct Application {}

async fn handle_connection(peer: SocketAddr, stream: TcpStream) -> Result<()> {
    let ws_stream = accept_async(stream).await.expect("Failed to accept");
    info!("New WebSocket connection: {}", peer);
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
    match ws_receiver.next().await {
        None => {}
        Some(s) => {}
    }

    let mut interval = tokio::time::interval(Duration::from_millis(1000));
    loop {
        tokio::select! {
            msg = ws_receiver.next() => {
                match msg {
                    Some(msg) => {
                        let msg = msg?;
                        if msg.is_text() ||msg.is_binary() {
                            ws_sender.send(msg).await?;
                        } else if msg.is_close() {
                            break;
                        }
                    }
                    None => break,
                }
            }
            _ = interval.tick() => {
                ws_sender.send(Message::Text("tick".to_owned())).await?;
            }
        }
    }
    Ok(())
}

#[tokio::main]
pub async fn main() {
    env_logger::init();

    let addr = "127.0.0.1:9002";
    let listener = TcpListener::bind(&addr).await.expect("Can't listen");
    info!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream.peer_addr().expect("connected streams should have a peer address");
        info!("Peer address: {}", peer);

        tokio::spawn(accept_connection(peer, stream));
    }
}
