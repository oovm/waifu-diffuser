use std::{net::SocketAddr, time::Duration};

use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use log::*;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Error, WebSocketStream};
use tungstenite::{Message, Result};

mod models;

async fn accept_connection(peer: SocketAddr, stream: TcpStream) {
    if let Err(e) = handle_connection(peer, stream).await {
        match e {
            Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
            err => error!("Error processing connection: {}", err),
        }
    }
}

pub struct WaifuDiffuserServer {}

pub struct WaifuDiffuserSession<'i> {
    server: &'i mut WaifuDiffuserServer,
    sender: SplitSink<WebSocketStream<TcpStream>, Message>,
    receiver: SplitStream<WebSocketStream<TcpStream>>,
}

impl<'i> WaifuDiffuserSession<'i> {
    // return should break
    pub async fn on_receive(&mut self, message: Option<Result<Message, Error>>) -> bool {
        let message = match message {
            Some(Ok(msg)) => msg,
            Some(Err(e)) => {
                error!("Error processing connection: {}", e);
                return false;
            }
            None => true,
        };
        match message {
            Message::Text(_) => {}
            Message::Binary(_) => {}
            Message::Ping(_) => {}
            Message::Pong(_) => {}
            Message::Close(_) => {}
            Message::Frame(_) => {}
        }

        false
    }
    pub async fn start(&mut self) {
        let mut interval = tokio::time::interval(Duration::from_millis(1000));
        loop {
            tokio::select! {
                msg = ws_receiver.next() => {
                    if !self.on_receive(msg).await {
                        break;
                    }
                }
                _ = interval.tick() => {
                    ws_sender.send(Message::Text("tick".to_owned())).await?;
                }
            }
        }
    }
}

impl WaifuDiffuserServer {
    pub fn new() -> Self {
        Self {}
    }

    async fn handle_connection(&mut self, peer: SocketAddr, stream: TcpStream) -> Result<()> {
        let ws_stream = accept_async(stream).await.expect("Failed to accept");
        info!("New WebSocket connection: {}", peer);
        let (mut ws_sender, mut ws_receiver) = ws_stream.split();
        WaifuDiffuserSession { server: self, sender: ws_sender, receiver: ws_receiver };

        Ok(())
    }
}

async fn receive_handle(context: &mut WaifuDiffuserServer, message: Option<Result<Message, Error>>) -> Option<bool> {
    match message? {
        Ok(Message::Text(text)) => {
            info!("Received a text message from {}", text);
            ws_sender.send(Message::Text(text)).await?;
        }
        Ok(Message::Binary(bytes)) => {
            info!("Received a text message from {:?}", bytes);
            ws_sender.send(Message::Binary(bytes)).await?;
        }
        Ok(s) => {
            error!("Received an unsupported message: {s:?}");
        }
        Err(s) => {
            error!("Error processing connection: {}", s);
        }
    }
    return Some(true);
}

pub struct Application {}

#[tokio::main]
pub async fn main() {
    env_logger::init();

    let addr = "127.0.0.1:9002";
    let listener = TcpListener::bind(&addr).await.expect("Can't listen");
    let mut server = WaifuDiffuserServer::new();
    info!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream.peer_addr().expect("connected streams should have a peer address");
        info!("Peer address: {}", peer);
        tokio::spawn(server.accept_connection(peer, stream));
    }
}
