use std::io;

use crate::{
    connect::{ConnackPacket, ConnectPacket},
    package::{decode::decode, encode::encode, types::ControlPackets},
    pubsub::{PublishPacket, PublishPacketGet, QosLevel, SubackPacket, SubscribePacket},
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
    ) -> Result<(), String> {
        let packet_id = 1;
        let publishpacket = PublishPacket::new(topic, message, packet_id, dup, qos, retain);
        if let Err(e) = self.stream.send(encode(publishpacket)).await {
            return Err(format!("Publish Fail with Error: {e}"));
        }
        if qos == QosLevel::Qos1 as u8 {
            todo!();
        }
        if qos == QosLevel::Qos2 as u8 {
            todo!();
        }

        Ok(())
    }

    pub async fn subscribe(&mut self, topic: &str, qos: u8) -> Result<(), String> {
        let packet_id = 1;
        let subpacket = SubscribePacket::new(packet_id, topic, qos);
        if let Err(e) = self.stream.send(encode(subpacket)).await {
            println!("{e}");
            return Err("Can't send Subscribe packet to broker".to_string());
        }

        if let Ok(data) = self.stream.read().await {
            decode(SubackPacket, data);
        } else {
            return Err(
                "Can't get Suback packet packet from broker, must send sub packet again"
                    .to_string(),
            );
        }

        Ok(())
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
        if let Ok(packet) = self.stream.read().await {
            if packet[0] == ControlPackets::Publish as u8 {
                println!("Get Publish message");
                decode(PublishPacketGet, packet);
            } else {
                println!("Packet[0] = {}", packet[0]);
                println!("Not define packet, must check again");
            }
        }

        Ok(())
    }
}
