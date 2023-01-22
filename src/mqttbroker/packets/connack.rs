use crate::decode::{decode_property, property, varint};
use crate::mqttbroker::packets::connect::Connect;
use crate::mqttbroker::packets::reason_codes::CONNECTACK;
use crate::mqttbroker::packets::PacketTypes::Connack;
use crate::mqttbroker::packets::{
    encode_fixed_header, encode_properties, prepend_size_to_properties, reason_codes,
    BuilderLifecycle, Decoder, Encoder, GeneratePacketParts, PacketTypes, Properties,
};
use crate::mqttbroker::properties::Property;
use bytes::{Buf, BufMut, BytesMut};
use std::io::Error;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ConnAck {
    //fixed header
    packet_type: u8,
    packet_type_low_nibble: u8,

    // variable header
    connect_ack_flags: u8,
    connect_reason_code: u8,
    variable_header_properties: Option<Vec<Property>>,
    //payload
    // no payload
}

impl ConnAck {
    const SESSION_PRESENT: u8 = 1;
    pub fn session_present(&mut self) -> bool {
        self.connect_ack_flags & ConnAck::SESSION_PRESENT == ConnAck::SESSION_PRESENT
    }
    // pub fn set_session_present(&mut self, session_present: bool) {
    //     if session_present {
    //         self.connect_ack_flags = ConnAck::SESSION_PRESENT;
    //     }
    //     self.connect_ack_flags = 0;
    // }

    // pub fn set_connect_reason_code(mut self, reason_code: reason_codes::CONNECTACK) {
    //     self.connect_reason_code = reason_code as u8;
    // }
}

pub mod encode_validation {} // do this

pub mod decode_validation {} // do this

impl Default for ConnAck {
    fn default() -> Self {
        ConnAck {
            packet_type: (PacketTypes::Connack as u8),
            packet_type_low_nibble: 0,
            connect_ack_flags: 0,
            connect_reason_code: CONNECTACK::Success as u8,
            variable_header_properties: None,
        }
    }
}
#[derive(Debug, Clone, Default)]
pub struct ConnAckBuilder {
    pub packet: ConnAck,
}

impl ConnAckBuilder {
    pub fn set_session_present(mut self, session_present: bool) -> Self {
        if session_present {
            self.packet.connect_ack_flags = 1
        } else {
            self.packet.connect_ack_flags = 0
        }

        self
    }

    pub fn set_connect_reason_code(mut self, reason_code: CONNECTACK) -> Self {
        self.packet.connect_reason_code = reason_code as u8;
        self
    }
}

impl BuilderLifecycle<ConnAck, Error> for ConnAckBuilder {
    fn new() -> Self {
        Default::default()
    }

    fn build(self) -> Result<ConnAck, Error> {
        Ok(self.packet)
    }
}

impl Properties for ConnAckBuilder {
    fn packet_type(&self) -> PacketTypes {
        Connack
    }

    fn packet_type_string(&self) -> String {
        String::from("CONNACK")
    }

    fn variable_header_properties(&self) -> &Option<Vec<Property>> {
        &self.packet.variable_header_properties
    }

    fn set_variable_header_properties(&mut self, p: Option<Vec<Property>>) {
        self.packet.variable_header_properties = p;
    }
}

impl GeneratePacketParts for ConnAck {
    fn generate_variable_header(&self) -> BytesMut {
        let mut variable_header = BytesMut::with_capacity(200);
        variable_header.put_u8(self.connect_ack_flags);
        variable_header.put_u8(self.connect_reason_code);

        encode_properties(variable_header, &self.variable_header_properties)
    }

    fn generate_payload(&self) -> BytesMut {
        BytesMut::with_capacity(0)
    }
}

impl Encoder<ConnAck> for ConnAck {}

impl Decoder<ConnAck> for ConnAck {
    fn decode(bytes: &mut BytesMut) -> anyhow::Result<ConnAck> {
        let packet_type_with_flags = bytes.get_u8();
        let packet_type = packet_type_with_flags >> 4;
        let packet_type_flags = packet_type_with_flags & 0x0f;
        let packet_size = varint(bytes).unwrap();
        let connect_ack_flags = bytes.get_u8();
        let connect_reason_code = bytes.get_u8();

        let variable_header_properties = decode_property(bytes);

        Ok(ConnAck {
            packet_type,
            packet_type_low_nibble: packet_type_flags,
            connect_ack_flags,
            connect_reason_code,
            variable_header_properties,
        })
    }
}
#[cfg(test)]
pub mod test {
    use crate::mqttbroker::packets::connack::{ConnAck, ConnAckBuilder};
    use crate::mqttbroker::packets::reason_codes::CONNECTACK;
    use crate::mqttbroker::packets::{BuilderLifecycle, Decoder, Encoder, Properties};
    use crate::mqttbroker::primitive_types::{Byte, TwoByteInteger};
    use crate::mqttbroker::properties::Property;

    #[test]
    pub fn should_encode_decode_connack_packet() {
        let mut packet_builder = ConnAckBuilder::new();

        packet_builder =
            packet_builder.set_connect_reason_code(CONNECTACK::BadAuthenticationMethod);
        packet_builder = packet_builder.set_session_present(true);
        let props = vec![
            Property::ReceiveMaximum(TwoByteInteger(100)),
            Property::MaximumQos(Byte(1)),
        ];

        let res = packet_builder.set_properties(&props);

        let built_packet = packet_builder.build().unwrap();

        let mut serialized_packet = ConnAck::encode(
            built_packet.packet_type,
            built_packet.packet_type_low_nibble,
            &built_packet,
        )
        .unwrap();

        let deserialized_packet = ConnAck::decode(&mut serialized_packet).unwrap();

        assert_eq!(built_packet, deserialized_packet);
    }
}
