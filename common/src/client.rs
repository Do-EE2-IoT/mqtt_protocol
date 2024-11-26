use std::io;

use crate::tcp_stream_handler::ClientStreamHandler;
use std::net::SocketAddr;
use std::net::SocketAddrV4;

pub struct Client {
    stream: ClientStreamHandler,
    client_id: u16,
    host: String,
    port: u16,
    username: Option<String>,
    password: Option<String>,
    keep_alive: u8,
    clean_session: u8,
}

impl Client {
    pub async fn new(
        client_id: u16,
        host: String, // Host sẽ được lấy từ tham số đầu vào
        port: u16,
        username: Option<String>,
        password: Option<String>,
        keep_alive: u8,
        clean_session: u8,
    ) -> Result<Self, String> {
        // Kết hợp host và port thành SocketAddr
        let host_and_port = match format!("{}:{}", host, port).parse::<SocketAddr>() {
            Ok(addr) => addr,
            Err(_) => return Err("Invalid host or port".to_string()),
        };

        // Thực hiện kết nối tới broker
        let stream = match ClientStreamHandler::connect(&host_and_port.to_string()).await {
            Ok(s) => s,
            Err(e) => return Err(format!("Failed to connect to broker: {}", e)),
        };

        // Trả về một instance của MqttClient
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
