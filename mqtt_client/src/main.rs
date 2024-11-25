use common::connect::{ConnackPacket, ConnectPacket};
use common::package::decode::decode;
use common::package::encode::encode;
use common::pubsub::PublishPacket;
use common::tcp_stream_handler::ClientStreamHandler;
use futures::TryFutureExt;
use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Kết nối tới MQTT broker
    let broker_address = "127.0.0.1:1885"; // Địa chỉ broker (sử dụng localhost)
    let mut stream = ClientStreamHandler::connect(broker_address).unwrap();
    println!("Đã kết nối tới broker tại {}", broker_address);

    // Tạo gói CONNECT
    let client_id = "rust_client"; // Client ID (phải là chuỗi không rỗng)

    let connect_packet = ConnectPacket::new(60, client_id.to_string());
    let connect_packet_encode = encode(connect_packet);
    if let Err(e) = stream.send(connect_packet_encode) {
        println!("{e}");
    }

    // Đọc phản hồi từ broker (CONNACK)
    let connect_ack = stream.read().unwrap();
    decode(ConnackPacket, connect_ack);

    let publish_packet = PublishPacket::new("/hello", "How are you", 1,0,0,0);
    if let Err(e) = stream.send(encode(publish_packet)) {
        println!("{e}");
    }


    Ok(())
}
