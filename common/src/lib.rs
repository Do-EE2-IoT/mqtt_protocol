pub mod client;
pub mod connect;
pub mod input;
pub mod mqtt;
pub mod ping;
pub mod pubsub;
pub mod tcp_stream_handler;
pub mod utils;

#[cfg(test)] // Đảm bảo rằng chỉ khi test thì module này mới được biên dịch
mod tests {
    use crate::client::Client;

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

        let result = client.connect(1).await;

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Can't read anything from broker, must reconnect again".to_string()
        );
    }

}
