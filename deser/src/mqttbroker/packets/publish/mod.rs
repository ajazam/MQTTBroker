mod builder;
mod deser;
mod validation;

use crate::mqttbroker::packets::{
    encode_properties, Decoder, Encoder, GeneratePacketParts, PacketTypes, Properties,
};
use crate::mqttbroker::properties::Property;
use bytes::{Buf, BufMut, BytesMut};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Publish {
    //fixed header
    packet_type: u8,
    packet_type_low_nibble: u8,

    //variable_header
    topic_name: String,
    packet_id: Option<u16>,
    variable_header_properties: Option<Vec<Property>>,

    //payload
    application_message: Option<Vec<u8>>,
}

impl Publish {
    pub fn dup(&self) -> bool {
        self.packet_type_low_nibble & 8 == 8
    }

    pub fn qos(&self) -> Qos {
        let qos = (self.packet_type_low_nibble >> 1) & 3;

        if qos == 0 {
            Qos::Q0
        } else if qos == 1 {
            Qos::Q1(self.packet_id.unwrap())
        } else if qos == 2 {
            Qos::Q2(self.packet_id.unwrap())
        } else {
            Qos::Reserved
        }
    }

    pub fn qos_number(&self) -> u8 {
        match self.qos() {
            Qos::Q0 => 0,
            Qos::Q1(_) => 1,
            Qos::Q2(_) => 2,
            Qos::Reserved => 3,
        }
    }

    pub fn packet_id(self) -> Option<u16> {
        self.packet_id
    }

    pub fn retain(&self) -> bool {
        self.packet_type_low_nibble & 1 == 1
    }
}
#[repr(u8)]
pub enum Qos {
    Q0 = 0u8,
    Q1(u16) = 1u8,
    Q2(u16) = 2u8,
    Reserved = 3u8,
}

impl Default for Publish {
    fn default() -> Self {
        Publish {
            packet_type: PacketTypes::Publish as u8,
            packet_type_low_nibble: 0,
            topic_name: "".to_string(),
            packet_id: None,
            variable_header_properties: None,
            application_message: None,
        }
    }
}

use crate::decode::{decode_property, utf8_string, varint};
use crate::encode::utf8_encoded_string;

use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PublishError {
    #[error("There is no Packet ID for qos level {0}. Require a Packet ID for qos > 0")]
    NoPacketIdForQos(u8),
    #[error("Dup flag is set to 1 when Qos is 0. Dup flag should be set to 0")]
    DupInvalidForQos0,
    #[error("Malformed packet. Both Qos bits are set to 1")]
    BothQosBitsAreSet,
    #[error("Topic name not present")]
    TopicNotPresent,
}

#[cfg(test)]
pub mod test {
    use crate::mqttbroker::packets::publish::{Publish, Qos};
    use pretty_hex::{pretty_hex, PrettyHex};

    use crate::mqttbroker::packets::publish::builder::PublishBuilder;
    use crate::mqttbroker::packets::{BuilderLifecycle, Decoder, Encoder, Properties};
    use crate::mqttbroker::primitive_types::TwoByteInteger;
    use crate::mqttbroker::properties::Property;

    #[test]
    pub fn test_encode_decode_publish_packet() {
        let mut packet_builder = PublishBuilder::new();
        let props = vec![Property::TopicAlias(TwoByteInteger(70))];
        let res = packet_builder.set_properties(&props);
        packet_builder =
            packet_builder.set_topic_with_payload(String::from("abcdef"), Some(vec![1u8, 2, 3, 4]));
        packet_builder = packet_builder.set_qos(Qos::Q1(100));
        let built_packet = packet_builder.build().unwrap();
        let mut serialized_packet = Publish::encode(
            built_packet.packet_type,
            built_packet.packet_type_low_nibble,
            &built_packet,
        )
        .unwrap();
        let copy = serialized_packet.clone().to_vec();
        let deserialized_packet = Publish::decode(&mut serialized_packet).unwrap();

        println!("serialized packet is {copy:?}");
        println!("hex is {}", copy.hex_dump());

        assert_eq!(built_packet, deserialized_packet);
    }
}
