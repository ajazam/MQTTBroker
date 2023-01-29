mod builder;
mod deser;
mod validation;

use crate::packets::reason_codes::PUBCOMP;
use crate::packets::PacketTypes;
use crate::properties::Property;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PubComp {
    //fixed header
    pub packet_type: u8,

    pub packet_type_low_nibble: u8,

    //variable header
    pub packet_id: u16,
    pub reason_code: PUBCOMP,
    // if the remaining length is 4 then property length is zero
    pub variable_header_properties: Option<Vec<Property>>,
    //no payload
}

impl Default for PubComp {
    fn default() -> Self {
        PubComp {
            packet_type: PacketTypes::Pubcomp as u8,
            packet_type_low_nibble: 0,
            packet_id: 0,
            reason_code: PUBCOMP::Success,
            variable_header_properties: None,
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::packets::pubcomp::builder::PubCompBuilder;
    use crate::packets::pubcomp::PubComp;
    use crate::packets::reason_codes::{DecodeReasonCode, PUBCOMP};
    use crate::packets::{BuilderLifecycle, Decoder, Encoder, Properties};
    use crate::primitive_types::{Utf8EncodedString, Utf8StringPair};
    use crate::properties::Property;

    #[test]
    pub fn show_encode_decode_packet() {
        let mut original_packet = PubCompBuilder::new();
        let props = vec![
            Property::ReasonString(Utf8EncodedString(String::from("Cant be bothered"))),
            Property::User(Utf8StringPair(String::from("hello"), String::from("world"))),
        ];

        original_packet = original_packet.set_packet_id(0x3210);
        original_packet = original_packet.set_reason_code(PUBCOMP::PacketIdentifierNotFound);
        original_packet.set_variable_header_properties(Some(props));

        let build_packet = original_packet.build().unwrap();
        let mut serialized_packet = PubComp::encode(
            build_packet.packet_type,
            build_packet.packet_type_low_nibble,
            &build_packet,
        )
        .unwrap();
        let deserialized_packet = PubComp::decode(&mut serialized_packet).unwrap();

        assert_eq!(build_packet, deserialized_packet)
    }
}
