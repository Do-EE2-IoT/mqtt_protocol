pub mod client;
pub mod connect;
pub mod input;
pub mod mqtt;
pub mod ping;
pub mod pubsub;
pub mod tcp_stream_handler;
pub mod utils;

#[cfg(test)]
mod tests {

    use crate::client::Client;
    use std::time::Duration;
    use tokio::time::sleep;

    mod connect_tests {
        use super::*;

        #[tokio::test]
        async fn test_connect_timeout() {
            let mut client = Client::new(
                "test_client".to_string(),
                "test.mosquitto.org".to_string(),
                1883,
                None,
                None,
                60,
                1,
            )
            .await
            .unwrap();

            let result = client.connect(100).await;

            assert!(result.is_err());
            assert_eq!(
                result.unwrap_err(),
                "Can't read anything from broker, must reconnect again".to_string()
            );
        }

        #[tokio::test]
        async fn test_connect_success() {
            let mut client = Client::new(
                "test_client".to_string(),
                "broker.hivemq.com".to_string(),
                1883,
                None,
                None,
                60,
                1,
            )
            .await
            .unwrap();

            let result = client.connect(15000).await;
            assert!(result.is_ok());
        }
    }

    // Test Pub/Sub
    mod pubsub_tests {
        use super::*;

        #[tokio::test]
        async fn test_ping_pub_sub() {
            let mut client = Client::new(
                "test_client".to_string(),
                "white-dev.aithings.vn".to_string(),
                1883,
                None,
                None,
                60,
                1,
            )
            .await
            .unwrap();

            let result = client.connect(10000).await;
            assert!(result.is_ok());

            let result = client.ping().await;
            assert!(result.is_ok());

            let result = client.subscribe("/qos0", 0).await;
            assert!(result.is_ok());

            let result = client
                .publish("/qos0", "Hello my friend qos 0", 0, 0, 0)
                .await;
            assert!(result.is_ok());

            let result = client.wait_publish_message().await;
            assert!(result.is_ok());

            let result = client
                .publish("/qos1", "Hello my friend qos 1", 1, 0, 0)
                .await;
            sleep(Duration::from_secs(1)).await;
            assert!(result.is_ok());

            let result = client
                .publish("/qos2", "Hello my friend qos 2", 2, 0, 0)
                .await;
            sleep(Duration::from_secs(1)).await;
            assert!(result.is_ok());

            let result = client.unsubscribe("/hello").await;
            assert!(result.is_ok());
        }
    }
}
