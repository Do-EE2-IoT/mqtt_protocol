use common::client::Client;
use common::input::Input;
use common::input::{ConsoleInput, InputUser};
use common::tcp_stream_handler::tokio;

#[tokio::main]
async fn main() -> Result<(), String> {
    let mut console_input = ConsoleInput {
        buffer: String::new(),
    };

    let mut client = Client::new(
        "Nguyen Van Do 2".to_string(),
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

    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(60));

    loop {
        tokio::select! {
            _ = interval.tick() => {
                if let Err(e) = client.ping().await {
                    println!("{e}");
                }
            },
            _ = client.wait_publish_message() => {},
            input = console_input.pop() => {
                if let Ok(data) = input {
                    match data {
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
    }

    Ok(())
}
