#[cfg(test)]
mod test {
    use crate::packets::connect::builder::*;
    use crate::packets::connect::Connect;
    use crate::packets::BuilderLifecycle;
    use crate::packets::Decoder;
    use crate::packets::Encoder;
    use crate::packets::Properties;
    use crate::primitive_types::{Byte, FourByteInteger};
    use crate::properties::Property;
    use bytes::BytesMut;
    use std::time::Duration;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::{TcpListener, TcpStream};
    use tokio::time::sleep;

    #[tokio::test]
    async fn should_send_bytes_from_client_to_server() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let server_port = listener.local_addr().unwrap().port();
        tokio::spawn(async move {
            sleep(Duration::from_millis(10)).await;
            let server_address = format!("127.0.0.1:{server_port}");
            let mut stream = TcpStream::connect(server_address).await.unwrap();
            stream.write_all(b"1234").await.unwrap();
        });

        let (mut socket, _) = listener.accept().await.unwrap();
        let mut buf = [0; 4];
        let n = match socket.read(&mut buf).await {
            Ok(n) => {
                println!("***************************************   {buf:?}");
                n
            }
            Err(_) => 0,
        };
        assert_eq!(&buf, b"1234");
    }

    #[tokio::test]
    async fn should_send_connect_packet_from_client_to_server() {
        let mut original_packet = ConnectBuilder::new();

        // variable header fields - start
        let props = vec![
            Property::PayloadFormatIndicator(Byte(1)),
            Property::SessionExpiryInterval(FourByteInteger(231)),
            Property::SharedSubscriptionAvailable(Byte(1)),
        ];

        original_packet = original_packet.set_keep_alive(1000);
        original_packet.set_properties(&props);
        // variable header fields - end

        // payload fields - start
        original_packet = original_packet.password(Some("hello".to_string()));
        original_packet = original_packet.client_id("ID".to_string());
        original_packet = original_packet.set_keep_alive(1000);
        let res = original_packet.will_message(&vec![], "topic".to_string(), vec![1, 2, 3, 4]);
        // payload fields - end

        let built_packet = original_packet.build().unwrap();

        let serialized_packet = Connect::encode(
            built_packet.packet_type,
            built_packet.packet_type_low_nibble,
            &built_packet,
        )
        .unwrap();

        let serialized_packet_2 = serialized_packet.clone();

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let server_port = listener.local_addr().unwrap().port();

        tokio::spawn(async move {
            sleep(Duration::from_millis(10)).await;
            let server_address = format!("127.0.0.1:{server_port}");
            let mut stream = TcpStream::connect(server_address).await.unwrap();
            stream.write_all(&serialized_packet_2).await.unwrap();
        });

        let (mut socket, _) = listener.accept().await.unwrap();

        let mut buf = [0; 200];

        let n = match socket.read(&mut buf).await {
            Ok(n) => {
                println!("+++++ read from socket is {buf:?}");
                n
            }
            Err(_) => 0,
        };

        let mut packets_bytes: Vec<u8> = Vec::with_capacity(n);
        // let mut packets_bytes = BytesMut::with_capacity(n);

        for i in 0..n {
            packets_bytes.push(buf[i])
        }
        let mut packets_bytes = BytesMut::from(packets_bytes.as_slice());
        let decoded_connect_packet = Connect::decode(&mut packets_bytes).unwrap();

        // assert_eq!(packets_bytes, serialized_packet.to_vec());
        assert_eq!(built_packet, decoded_connect_packet);
    }
}
