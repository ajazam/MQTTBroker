mod builder;
mod deser;
mod validation;

use crate::packets::reason_codes::CONNECTACK;
use crate::packets::{PacketTypes, Properties};
use crate::properties::Property;

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
}

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

#[cfg(test)]
pub mod test {
    use crate::packets::connack::builder::ConnAckBuilder;
    use crate::packets::connack::ConnAck;
    use crate::packets::reason_codes::CONNECTACK;
    use crate::packets::{BuilderLifecycle, Decoder, Encoder, Properties};
    use crate::primitive_types::{Byte, TwoByteInteger};
    use crate::properties::Property;

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
