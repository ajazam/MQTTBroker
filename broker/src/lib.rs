// packet specific tests

//

// global tests

use bytes::BytesMut;
use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use tokio::net::{TcpListener, ToSocketAddrs};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::task::JoinSet;

mod control_packets {
    use bytes::BytesMut;
    use deser::ControlPacket;
    use tokio::sync::mpsc::{Receiver, Sender};
    use tokio::task::JoinHandle;

    ///
    /// Binds a tcplistener to the specified address and return a joinhandle to the task  
    ///
    pub async fn sender_receiver_factory(
        addr: String,
    ) -> Option<(
        JoinHandle<()>,
        Receiver<(JoinHandle<()>, Sender<BytesMut>, Receiver<BytesMut>)>,
    )> {
        todo!()
    }

    // pub async fn convert_bytes_to_controlpackets(
    //     rx: Receiver<BytesMut>,
    // ) -> Option<Sender<ControlPacket>> {
    //     todo!()
    // }
    //
    // pub async fn convert_controlpacket_to_bytes(
    //     rx: Receiver<ControlPacket>,
    // ) -> Option<Sender<BytesMut>> {
    //     todo!()
    // }
}

pub async fn connection_listener(
    addr: impl ToSocketAddrs,
    from_mqtt_client_sender: Sender<BytesMut>,
    mut bytes_to_mqtt_client_receiver: Receiver<BytesMut>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let listener = TcpListener::bind(addr).await?;
    let (tx, mut rx) = mpsc::channel::<BytesMut>(32);

    let (socket, _) = listener.accept().await?;
    let (mut owned_read_half, mut owned_write_half) = socket.into_split();

    let mut set = JoinSet::new();

    set.spawn(async move {
        // Receives bytes from the tcp stream.
        let mut bytes_read = BytesMut::with_capacity(128);
        loop {
            match owned_read_half.read_buf(&mut bytes_read).await {
                Ok(n) if n > 0 => {
                    from_mqtt_client_sender
                        .send(bytes_read.clone())
                        .await
                        .unwrap();
                }

                _ => break,
            }
        }
    });

    set.spawn(async move {
        // Sends bytes to tcp stream
        while let Some(mut bytes) = bytes_to_mqtt_client_receiver.recv().await {
            match owned_write_half.write_buf(&mut bytes).await {
                Ok(n) if n > 0 => {
                    // no sure what todo here;
                }
                _ => break,
            }
        }

        let _ = owned_write_half.shutdown().await;
    }); // create write stream

    while let Some(_s) = set.join_next().await {}
    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {}

    pub async fn should_return_rx_tx_channels_for_a_new_client_connection() {}
}
