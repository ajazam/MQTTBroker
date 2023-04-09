#[cfg(test)]
mod server_test {
    use bytes::{BufMut, BytesMut};
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpStream;
    use tokio::spawn;
    use tokio::sync::mpsc;
    use MQTTBroker::connection_listener;

    #[tokio::test]
    async fn should_act_as_loopback() {
        // bind to a socket Server

        //connect to a socket.

        // send connect to a socket

        assert!(true)
    }

    #[tokio::test]
    async fn should_send_data_over_streams() {
        let (from_mqtt_client_sender, mut from_mqtt_client_receiver) =
            mpsc::channel::<BytesMut>(10);
        let (to_mqtt_client_sender, to_mqtt_client_receiver) = mpsc::channel::<BytesMut>(10);
        spawn(connection_listener(
            "127.0.0.1:1883",
            from_mqtt_client_sender,
            to_mqtt_client_receiver,
        ));

        let mut stream = TcpStream::connect("127.0.0.1:1883").await.unwrap();

        let _ = stream.write_all(b"hello").await;
        let received = from_mqtt_client_receiver.recv().await.unwrap();

        assert_eq!(b"hello".as_slice(), received);

        let mut to_send = BytesMut::with_capacity(100);
        to_send.put(&b"world"[..]);

        to_mqtt_client_sender.send(to_send).await;

        let mut received = BytesMut::with_capacity(100);

        stream.read_buf(&mut received).await;

        assert_eq!(b"world".as_slice(), received);

        // client connection to socket

        //

        // not finished
    }
}
