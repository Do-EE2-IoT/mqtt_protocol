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
    use tokio::time::timeout;

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

    #[tokio::test]
    async fn test_connect_success() {
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

        // Giả lập phản hồi hợp lệ từ broker
        let result = client.connect(10).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_connect_invalid_response() {
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

        // Giả lập phản hồi không hợp lệ từ broker
        let result = client.connect(2).await;

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be check again"));
    }

    #[tokio::test]
    async fn test_connect_failed() {
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

        // Giả lập không thể kết nối đến broker
        let result = client.connect(2).await;

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Failed to connect to broker"));
    }

    #[tokio::test]
    async fn test_connect_read_error() {
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

        // Giả lập lỗi khi đọc từ stream
        let result = client.connect(2).await;

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Can't read anything from broker"));
    }
}
