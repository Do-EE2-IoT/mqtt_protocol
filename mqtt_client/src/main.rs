use common::client::Client;
use common::input::Input;
use common::input::{ConsoleInput, InputUser};
use common::tcp_stream_handler::tokio;
use common::tcp_stream_handler::tokio::sync::mpsc::{Receiver, Sender};

async fn console_input_handle(tx: Sender<InputUser>) {
    let mut console_input = ConsoleInput {
        buffer: String::new(),
    };
    while let Ok(data) = console_input.pop().await {
        if let Err(e) = tx.send(data).await {
            println!("Can't use channel because of error {e}");
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let (tx, mut rx): (Sender<InputUser>, Receiver<InputUser>) = tokio::sync::mpsc::channel(1);
    tokio::spawn(console_input_handle(tx));
    let mut client = Client::new(
        "Nguyen Van Do".to_string(),
        "white-dev.aithings.vn".to_string(),
        1883,
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

    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(60));

    loop {
        tokio::select! {
        _ = interval.tick() => {
            if let Err(e) = client.ping().await {
                println!("{e}");
            }
        },
        _ = client.wait_publish_message() => {},
        Some(input) = rx.recv() => {
            match input {
                InputUser::Publish {
                    topic,
                    qos,
                    message,
                } => {
                    if let Err(e) = client.publish(topic.as_str(), message.as_str(), qos, 0, 0).await {
                        println!("{e}");
                    }
                },
                InputUser::Ping => {
                    if let Err(e) = client.ping().await {
                        println!("{e}");
                    }
                },
                InputUser::Subscribe {
                    topic,
                    qos,
                } => {
                    if let Err(e) = client.subscribe(topic.as_str(), qos).await {
                        println!("{e}");
                    }
                },
                InputUser::Disconnect => {
                    println!("Disconnect");
                    if let Err(e) = client.disconnect().await {
                        println!("{e}");
                    }
                },
                InputUser::Unsubscribe {
                    topic,
                } => {
                    if let Err(e) = client.unsubscribe(topic.as_str()).await {
                        println!("{e}");
                    }
                },
            }
            }
        }
    }
    Ok(())
}