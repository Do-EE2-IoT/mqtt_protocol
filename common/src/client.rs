use std::io;

use crate::{
    connect::{ConnackPacket, ConnectPacket, DisconnectPacket},
    mqtt::{decode::decode, encode::encode, types::ControlPackets},
    ping::{PingPacket, PingResPacket},
    pubsub::{
        PublishPacket, PublishPacketGet, QosLevel, SubackPacket, SubscribePacket, UnsubackPacket,
        UnsubscribePacket,
    },
    tcp_stream_handler::ClientStreamHandler,
    utils::handle_packet,
};
use tokio::time::timeout;
use tokio::time::Duration;

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
        let host_and_port = format!("{}:{}", host, port);
        println!("{}", host_and_port);

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

    pub async fn connect(&mut self) -> Result<(), String> {
        let connect = ConnectPacket::new(self.keep_alive, self.client_id.clone());
        if let Err(e) = self.stream.send(encode(connect)).await {
            return Err(format!("{e}"));
        }

        match timeout(Duration::from_secs(10), self.stream.read()).await {
            Ok(Ok(packet)) => {
                if packet[0] == ControlPackets::Connack as u8 {
                    decode(ConnackPacket, packet);
                    Ok(())
                } else {
                    Err(format!(
                        "Get another packet {}, must be check again",
                        packet[0]
                    ))
                }
            }
            Ok(Err(e)) => {
                println!("{e}");
                Err("Can't read anything from broker, must reconnect again".to_string())
            }
            Err(_) => {
                println!("Timeout");
                Err("Can't read anything from broker, must reconnect again".to_string())
            }
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
        let packet_id = 0x0002;
        let subpacket = SubscribePacket::new(packet_id, topic, qos);
        if let Err(e) = self.stream.send(encode(subpacket)).await {
            println!("{e}");
            return Err("Can't send Subscribe packet to broker".to_string());
        }
        let mut retries = 0;
        let max_retries = 5;

        while retries < max_retries {
            match timeout(Duration::from_millis(2000), self.stream.read()).await {
                Ok(Ok(packet)) => {
                    if packet[0] == ControlPackets::Suback as u8 {
                        if packet_id == ((packet[2] as u16) << 8) | (packet[3] as u16) {
                            decode(SubackPacket, packet);
                            return Ok(());
                        } else {
                            println!("Received Suback with another packet ID, ignoring...");
                        }
                    } else {
                        handle_packet(packet);
                    }
                }
                Ok(Err(e)) => {
                    println!("Error while reading packet: {e}");
                    return Err("Failed to read packet".to_string());
                }
                Err(_) => {
                    // println!("Timeout while waiting for Suback");
                }
            }

            retries += 1;
        }

        Err("Can't get suback".to_string())
    }

    pub async fn unsubscribe(&mut self, topic: &str) -> Result<(), String> {
        let packet_id = 3;
        let unsubpacket = UnsubscribePacket::new(packet_id, topic);
        if let Err(e) = self.stream.send(encode(unsubpacket)).await {
            println!("Error: {e}");
            return Err("Can't sen Unsub packet for broker. Must send again".to_string());
        }
        let mut retries = 0;
        let max_retries = 5;
        while retries < max_retries {
            match timeout(Duration::from_millis(2000), self.stream.read()).await {
                Ok(Ok(packet)) => {
                    if packet[0] == ControlPackets::Unsuback as u8 {
                        if packet_id == ((packet[2] as u16) << 8) | (packet[3] as u16) {
                            decode(UnsubackPacket, packet);
                            return Ok(());
                        } else {
                            println!("Get another packet id, ignore");
                        }
                    } else {
                        handle_packet(packet);
                        println!("Not ready get unsuback packet, wait");
                    }
                }

                Ok(Err(e)) => {
                    println!("{e}");
                    return Err("Can't get unsuback from broker, must send again ".to_string());
                }
                Err(_) => {} //println!("Timout get unsuback"), // Must be check timeout
            }
            retries += 1;
        }
        Err("Can't get unsuback".to_string())
    }

    pub async fn ping(&mut self) -> Result<(), String> {
        let pingpkg = PingPacket;
        if let Err(e) = self.stream.send(encode(pingpkg)).await {
            println!("{e}");
            return Err("Can't send ping packet to broker".to_string());
        }
        let mut retries = 0;
        let max_retries = 5;
        while retries < max_retries {
            match timeout(Duration::from_millis(2000), self.stream.read()).await {
                Ok(Ok(packet)) => {
                    if packet[0] == ControlPackets::Pingresp as u8 {
                        decode(PingResPacket, packet);
                        return Ok(());
                    } else {
                        handle_packet(packet);
                        println!("Not ready to get pingres, wait...");
                    }
                }
                Ok(Err(e)) => {
                    print!("{e}");
                    return Err("Can't read pingres from broker".to_string());
                }
                Err(_) => {}
            }
            retries += 1;
        }
        Err("Can't Pingres from broker".to_string())
    }

    pub async fn disconnect(&mut self) -> Result<(), String> {
        let disconnect = DisconnectPacket;
        if let Err(e) = self.stream.send(encode(disconnect)).await {
            println!("{e}");
            Err("Can't send request disconnect to broker, must send again".to_string())
        } else {
            Ok(())
        }
    }

    pub async fn wait_publish_message(&mut self) -> io::Result<()> {
        if let Ok(packet) = self.stream.read().await {
            if packet[0] == ControlPackets::Publish as u8 {
                println!("Get Publish message");
                decode(PublishPacketGet, packet);
            }
        }

        Ok(())
    }
}