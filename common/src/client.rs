use std::io;

use crate::{
    connect::{ConnackPacket, ConnectPacket},
    package::{decode::decode, encode::encode},
    tcp_stream_handler::ClientStreamHandler,
};
use std::net::SocketAddr;

pub struct Client {
    stream: ClientStreamHandler,
    client_id: String,
    host: String,
    port: u16,
    username: Option<String>,
    password: Option<String>,
    keep_alive: u16,
    clean_session: u8,
}

impl Client {
    pub async fn new(
        client_id: String,
        host: String,
        port: u16,
        username: Option<String>,
        password: Option<String>,
        keep_alive: u16,
        clean_session: u8,
    ) -> Result<Self, String> {
        let host_and_port = match format!("{}:{}", host, port).parse::<SocketAddr>() {
            Ok(addr) => addr,
            Err(_) => return Err("Invalid host or port".to_string()),
        };

        let stream = match ClientStreamHandler::connect(&host_and_port.to_string()).await {
            Ok(s) => s,
            Err(e) => return Err(format!("Failed to connect to broker: {}", e)),
        };

        Ok(Self {
            stream,
            client_id,
            host,
            port,
            username,
            password,
            keep_alive,
            clean_session,
        })
    }

    pub async fn connect(&mut self) -> Result<Vec<u8>, String> {
        let connect = ConnectPacket::new(self.keep_alive, self.client_id.clone());
        if let Err(e) = self.stream.send(encode(connect)).await {
            return Err(format!("{e}"));
        }

        if let Ok(buffer) = self.stream.read().await {
            Ok(buffer)
        } else {
            Err("Can't read Connack from broker".to_string())
        }
    }

    pub async fn publish(
        &mut self,
        topic: &str,
        message: &str,
        qos: u8,
        dup: u8,
        retain: u8,
    ) -> io::Result<()> {
        todo!();
    }

    pub async fn subscribe(&mut self, topic: &str, qos: u8) -> io::Result<()> {
        todo!();
    }

    pub async fn unsubscribe(&mut self, topic: &str) -> io::Result<()> {
        todo!();
    }

    pub async fn ping(&mut self) -> io::Result<()> {
        todo!();
    }

    pub async fn disconnect(&mut self) -> io::Result<()> {
        todo!();
    }

    pub async fn wait_publish_message(&mut self) -> io::Result<()> {
        todo!();
    }
}
