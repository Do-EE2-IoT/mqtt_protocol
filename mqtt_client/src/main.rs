use std::time::Duration;

use common::connect::{ConnackPacket, ConnectPacket, Disconnect};
use common::package::decode::decode;
use common::package::encode::{encode, Encode};
use common::ping::{self, PingPacket, PingResPacket};
use common::pubsub::{
    PublishPacket, PublishPacketResponse, SubscribePacket, SubscriberPacketResponse,
    UnsubscribeAcknowledge, UnsubscribePacket,
};
use common::tcp_stream_handler::ClientStreamHandler;

#[tokio::main]
async fn main() -> Result<(), String> {
    let broker_address = "127.0.0.1:1885"; // Địa chỉ broker (sử dụng localhost)
    let mut stream = ClientStreamHandler::connect(broker_address)
        .await
        .expect("Can't connect with mqtt broker");
    println!("Đã kết nối tới broker tại {}", broker_address);

    let client_id = "rust_client";

    let connect_packet = ConnectPacket::new(60, client_id.to_string());
    let connect_packet_encode = encode(connect_packet);
    if let Err(e) = stream.send(connect_packet_encode).await {
        println!("{e}");
    }
    // đọc

    if let Ok(connect_ack) = stream.read().await {
        decode(ConnackPacket, connect_ack);
    }
    let subscribe_packet = SubscribePacket::new(10, "/hello", 0);
    if let Err(e) = stream.send(encode(subscribe_packet)).await {
        println!("{e}");
    }
    if let Ok(suback) = stream.read().await {
        decode(SubscriberPacketResponse, suback);
    }

    let publish_packet = PublishPacket::new("/hello", "How are you", 1, 0, 0, 0);
    if let Err(e) = stream.send(encode(publish_packet)).await {
        println!("{e}");
    }

    if let Ok(publish) = stream.read().await {
        decode(PublishPacketResponse, publish);
    }

    let pingreq = PingPacket;
    if let Err(e) = stream.send(encode(pingreq)).await{
        println!("Error: {e}");
    }

    if let Ok(pingres) = stream.read().await {
        decode(PingResPacket, pingres);
    }

    let unsub = UnsubscribePacket::new(10, "/hello");
    if let Err(e) = stream.send(encode(unsub)).await {
        println!("Error : {e}");
    }
    if let Ok(unsuback) = stream.read().await {
        decode(UnsubscribeAcknowledge, unsuback);
    }

    let disconnect: Disconnect = Disconnect;
    if let Err(e) = stream.send(encode(disconnect)).await {
        println!("Error: {e}");
    }

    Ok(())
}
