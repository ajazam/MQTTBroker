extern crate core;

use crate::packets::auth::Auth;
use crate::packets::connack::ConnAck;
use crate::packets::disconnect::Disconnect;
use crate::packets::pingreq::PingReq;
use crate::packets::pingresp::PingResp;
use crate::packets::puback::PubAck;
use crate::packets::pubcomp::PubComp;
use crate::packets::publish::Publish;
use crate::packets::pubrec::PubRec;
use crate::packets::pubrel::PubRel;
use crate::packets::suback::SubAck;
use crate::packets::subscribe::Subscribe;
use crate::packets::unsuback::UnsubAck;
use std::error::Error;

pub mod decode;
pub mod encode;
pub mod net;
pub mod packets;
pub mod primitive_types;
pub mod properties;

#[derive(Debug, PartialEq, Eq)]
pub enum ControlPacket {
    Auth(Auth),
    ConnAck(ConnAck),
    Connect(Connect),
    Disconnect(Disconnect),
    PingReq(PingReq),
    PingResp(PingResp),
    PubAck(PubAck),
    PubComp(PubComp),
    Publish(Publish),
    PubRec(PubRec),
    PubRel(PubRel),
    SubAck(SubAck),
    Subscribe(Subscribe),
    UnsubAck(UnsubAck),
    Unsubscribe(UnSubscribe),
}

use bytes::{Buf, BufMut, BytesMut};
use decode::varint;
use nu_pretty_hex::PrettyHex;
use packets::connect::Connect;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;
use tracing::instrument;

// input channel of BytesMut
// output channel of MQTT enum control packet
// control_packet_deserialier

// input MQTT control packet enum
// output channel
// control_packet_serialiser

async fn control_packet_to_frames(
    mut bytes_from_mqtt_receiver: Receiver<BytesMut>,
    mqtt_control_packet_bytes_sender: Sender<BytesMut>,
) -> Result<(), Box<dyn Error>> {
    // loop
    // read packet from
    // concat packet to buffer
    // if not enough bytes then loop
    // check packet type.
    // read length field. If not enough bytes for length field the loop
    // get length from length field
    // are there enough bytes to satisfy length field, no loop
    // read enough bytes for control packet
    // valid bytes and deserialise packet to control packet enum
    // send packet to mqtt_control_packet_sender

    let mut buffer = BytesMut::with_capacity(2000);
    let mut iter = 0;
    while let Some(bytes) = bytes_from_mqtt_receiver.recv().await {
        iter += 1;
        println!("iteration {}", iter);
        buffer.put(bytes);
        if buffer.len() < 3 {
            println!("buffer is less than 3");
            continue;
        }
        let mut buf_copy = buffer.clone();
        let packet_type = buf_copy.get_u8();
        println!("got packet type");
        let bytes_before_varint = buf_copy.remaining();

        let remaining_length = match varint(&mut buf_copy) {
            Ok(n) => n,
            Err(e) => break,
        };
        println!("gor remaining length");

        let bytes_after_varint = buf_copy.remaining();
        println!("post remaining");
        let bytes_size_of_remaining_length = bytes_before_varint - bytes_after_varint;

        let total_bytes_in_control_packet =
            1 + bytes_size_of_remaining_length + remaining_length.0 as usize;

        println!("total bytes {}", total_bytes_in_control_packet);
        println!("buf_copy.len {}", buf_copy.len());
        println!("buffer len is {}", buffer.len());
        if buffer.len() < total_bytes_in_control_packet {
            // THIS IS WRONG.........
            println!("buf_copy.len() < total_bytes_in_control_packet");
            continue;
        }
        println!("pre send");
        println!("serialised packet is {:?}", buffer.hex_dump());
        mqtt_control_packet_bytes_sender
            .send(buffer.split_to(total_bytes_in_control_packet))
            .await?;
    }
    println!("exiting...################################################################################################");
    Ok(())
}

use crate::packets::unsubscribe::UnSubscribe;
use crate::packets::{Decoder, PacketTypes};

async fn bytes_to_control_packets(
    mut control_packet_bytes_receiver: Receiver<BytesMut>,
    control_packet_received_sender: Sender<ControlPacket>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    while let Some(mut control_packet) = control_packet_bytes_receiver.recv().await {
        let control_packet_type = control_packet.first().unwrap() >> 4; // check if this consumes bytes
        println!(
            "-------------------------serialised packet is {}",
            control_packet.hex_dump()
        );

        let control_packet = match control_packet_type {
            1 => ControlPacket::Connect(Connect::decode(&mut control_packet)?),

            2 => ControlPacket::ConnAck(ConnAck::decode(&mut control_packet)?),

            3 => ControlPacket::Publish(Publish::decode(&mut control_packet)?),
            4 => ControlPacket::PubRec(PubRec::decode(&mut control_packet)?),

            5 => ControlPacket::PubAck(PubAck::decode(&mut control_packet)?),

            6 => ControlPacket::PubRel(PubRel::decode(&mut control_packet)?),

            7 => ControlPacket::PubComp(PubComp::decode(&mut control_packet)?),

            8 => ControlPacket::Subscribe(Subscribe::decode(&mut control_packet)?),

            9 => ControlPacket::SubAck(SubAck::decode(&mut control_packet)?),

            10 => ControlPacket::Unsubscribe(UnSubscribe::decode(&mut control_packet)?),

            11 => ControlPacket::UnsubAck(UnsubAck::decode(&mut control_packet)?),

            12 => ControlPacket::PingReq(PingReq::decode(&mut control_packet)?),

            13 => ControlPacket::PingResp(PingResp::decode(&mut control_packet)?),

            14 => ControlPacket::Disconnect(Disconnect::decode(&mut control_packet)?),

            15 => ControlPacket::Auth(Auth::decode(&mut control_packet)?),

            _ => {
                println!("No match found decoding packet");
                ControlPacket::Disconnect(Disconnect::decode(&mut control_packet)?)
            }
        };
        match control_packet_received_sender.send(control_packet).await {
            Ok(_) => (),
            _ => println!("Send error in control_packet_constructor"),
        }
    }

    Ok(())
}

use crate::packets::Encoder;

#[instrument(name = "control_packet_to_bytes")]
async fn control_packet_to_bytes(
    mut control_packet_to_bytes_receiver: Receiver<ControlPacket>,
    serialised_control_packet_sender: Sender<BytesMut>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    tracing::info!("inside control_packet_to_bytes");

    while let Some(control_packet) = control_packet_to_bytes_receiver.recv().await {
        tracing::info!("inside while loop");
        let bytes = match control_packet {
            ControlPacket::Connect(packet) => {
                println!("encoded connect packet");
                println!("{:?}", packet);
                Connect::encode(packet.packet_type, packet.packet_type_low_nibble, &packet)?
            }
            ControlPacket::ConnAck(packet) => {
                ConnAck::encode(packet.packet_type, packet.packet_type_low_nibble, &packet)?
            }
            ControlPacket::Publish(packet) => {
                Publish::encode(packet.packet_type, packet.packet_type_low_nibble, &packet)?
            }
            ControlPacket::PubRec(packet) => {
                PubRec::encode(packet.packet_type, packet.packet_type_low_nibble, &packet)?
            }
            ControlPacket::PubAck(packet) => {
                PubAck::encode(packet.packet_type, packet.packet_type_low_nibble, &packet)?
            }
            ControlPacket::PubRel(packet) => {
                PubRel::encode(packet.packet_type, packet.packet_type_low_nibble, &packet)?
            }
            ControlPacket::PubComp(packet) => {
                PubComp::encode(packet.packet_type, packet.packet_type_low_nibble, &packet)?
            }
            ControlPacket::Subscribe(packet) => {
                Subscribe::encode(packet.packet_type, packet.packet_type_low_nibble, &packet)?
            }
            ControlPacket::SubAck(packet) => {
                SubAck::encode(packet.packet_type, packet.packet_type_low_nibble, &packet)?
            }
            ControlPacket::Unsubscribe(packet) => {
                UnSubscribe::encode(packet.packet_type, packet.packet_type_low_nibble, &packet)?
            }
            ControlPacket::UnsubAck(packet) => {
                UnsubAck::encode(packet.packet_type, packet.packet_type_low_nibble, &packet)?
            }
            ControlPacket::PingReq(packet) => {
                PingReq::encode(packet.packet_type, packet.packet_type_low_nibble, &packet)?
            }
            ControlPacket::PingResp(packet) => {
                PingResp::encode(packet.packet_type, packet.packet_type_low_nibble, &packet)?
            }
            ControlPacket::Disconnect(packet) => {
                Disconnect::encode(packet.packet_type, packet.packet_type_low_nibble, &packet)?
            }
            ControlPacket::Auth(packet) => {
                Auth::encode(packet.packet_type, packet.packet_type_low_nibble, &packet)?
            }
        };
        println!("in control_packet_to_bytes_receiver {:?}", bytes.hex_dump());
        serialised_control_packet_sender.send(bytes).await?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packets::connect::builder::ConnectBuilder;
    use crate::properties::Property;
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn should_return_bytes_for_control_packet() {
        let bytes = vec![1, 2, 1, 2];
        let (tx, rx) = mpsc::channel(10);
        let (control_packet_bytes_sender, mut control_packet_bytes_receiver) = mpsc::channel(10);
        tokio::spawn(async {
            bytes_to_control_packets(rx, control_packet_bytes_sender).await;
        });
        tx.send(BytesMut::from(bytes.as_slice())).await;

        let received = control_packet_bytes_receiver.recv().await;

        assert_eq!(bytes.as_slice(), vec![1, 2, 1, 2]);
    }

    use crate::packets::BuilderLifecycle;

    #[tokio::test]
    async fn should_serialise_and_deserialise_connect_packet() {
        // construct a subscriber that prints formatted traces to stdout
        let subscriber = tracing_subscriber::FmtSubscriber::new();
        // use that subscriber to process traces emitted after this point
        tracing::subscriber::set_global_default(subscriber);

        tracing::info!("inside");
        let original_packet = get_test_packet();

        let (control_packet_to_bytes_sender, control_packet_bytes_receiver) =
            mpsc::channel::<ControlPacket>(10);

        let (serialised_control_packet_sender, serialised_control_packet_receiver) =
            mpsc::channel::<BytesMut>(10);

        tokio::spawn(control_packet_to_bytes(
            control_packet_bytes_receiver,
            serialised_control_packet_sender,
        ));

        let (control_packet_sender, mut control_packet_receiver) =
            mpsc::channel::<ControlPacket>(10);

        tokio::spawn(bytes_to_control_packets(
            serialised_control_packet_receiver,
            control_packet_sender,
        ));

        control_packet_to_bytes_sender
            .send(ControlPacket::Connect(original_packet.clone()))
            .await;

        println!("post control_packet_to_bytes_sender");
        let received_control_packet = control_packet_receiver.recv().await.unwrap();
        println!("post received_control_packet");

        assert_eq!(
            ControlPacket::Connect(original_packet),
            received_control_packet
        );
    }

    pub fn get_test_packet() -> Connect {
        let mut original_packet = ConnectBuilder::new();

        // variable header fields - start
        let props: Vec<Property> = vec![];

        original_packet = original_packet.set_keep_alive(1000);
        // original_packet.set_properties(&props);
        // variable header fields - end

        // payload fields - start
        original_packet = original_packet.password(Some("hello".to_string()));
        original_packet = original_packet.client_id("ID".to_string());
        original_packet = original_packet.set_keep_alive(1000);
        let res = original_packet.will_message(&vec![], "topic".to_string(), vec![1, 2, 3, 4]);
        let original_packet = original_packet.packet;

        original_packet
    }
}
