mod builder;
mod deser;
mod validation;

use crate::packets::reason_codes::PUBREC;
use crate::packets::PacketTypes;
use crate::properties::Property;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PubRec {
    //fixed header
    pub packet_type: u8,

    pub packet_type_low_nibble: u8,

    //variable header
    pub packet_id: u16,
    pub reason_code: PUBREC,
    // if the remaining length is 4 then property length is zero
    pub variable_header_properties: Option<Vec<Property>>,
    //no payload
}

impl Default for PubRec {
    fn default() -> Self {
        PubRec {
            packet_type: PacketTypes::Pubrec as u8,
            packet_type_low_nibble: 0,
            packet_id: 0,
            reason_code: PUBREC::Success,
            variable_header_properties: None,
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::packets::pubrec::builder::PubRecBuilder;
    use crate::packets::pubrec::PubRec;
    use crate::packets::reason_codes::PUBREC;
    use crate::packets::{BuilderLifecycle, Decoder, Encoder, Properties};
    use crate::primitive_types::Utf8StringPair;
    use crate::properties::Property;

    #[test]
    pub fn show_encode_decode_packet() {
        let mut original_packet = PubRecBuilder::new();
        let props = vec![Property::User(Utf8StringPair(
            String::from("abc"),
            String::from("abc"),
        ))];
        original_packet = original_packet.set_packet_id(0x1234);
        original_packet = original_packet.set_reason_code(PUBREC::PacketIdentifierInUse);
        original_packet.set_variable_header_properties(Some(props));

        let build_packet = original_packet.build().unwrap();
        let mut serialized_packet = PubRec::encode(
            build_packet.packet_type,
            build_packet.packet_type_low_nibble,
            &build_packet,
        )
        .unwrap();

        let deserialized_packet = PubRec::decode(&mut serialized_packet).unwrap();

        assert_eq!(build_packet, deserialized_packet);
    }
}
