use common::client::{self, Client};
use common::connect::ConnackPacket;
use common::package::decode::decode;
use common::tcp_stream_handler::tokio;


#[tokio::main]
async fn main() -> Result<(), String> {
    let mut client = Client::new(
        "Nguyen Van Do".to_string(),
        "127.0.0.1".to_string(),
        1885,
        None,
        None,
        60,
        0,
    )
    .await
    .expect("Must give suitable parameter to init connection with broker!");
    if let Err(e) = client.connect().await {
        println!("{e}");
    }

    if let Err(e) = client.subscribe("/hello", 0).await {
        println!("{e}");
    }
    if let Err(e) = client.subscribe("/hello", 0).await {
        println!("{e}");
    }
    if let Err(e) = client.subscribe("/hello", 0).await {
        println!("{e}");
    }
    if let Err(e) = client.subscribe("/hello", 0).await {
        println!("{e}");
    }

    if let Err(e) = client.publish("/hello", "Hanoi univesity", 0, 0, 0).await {
        println!("{e}");
    }

    if let Err(e) = client.wait_publish_message().await {
        println!("{e}");
    }

    if let Err(e) = client.unsubscribe("/hello").await {
        print!("{e}");
    }

    if let Err(e) = client.ping().await {
        print!("{e}");
    }

    if let Err(e) = client.disconnect().await {
        println!("{e}");
    }

    Ok(())
}
