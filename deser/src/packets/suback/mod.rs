mod builder;
mod deser;
mod validation;

use crate::packets::reason_codes::SUBACK;
use crate::packets::{PacketTypes, Properties};
use crate::properties::Property;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SubAck {
    //fixed header
    pub packet_type: u8,
    pub packet_type_low_nibble: u8,

    //variable header
    pub packet_id: u16,
    pub variable_header_properties: Option<Vec<Property>>,

    //payload
    pub reason_codes: Vec<SUBACK>,
}

impl Default for SubAck {
    fn default() -> Self {
        SubAck {
            packet_type: PacketTypes::Suback as u8,
            packet_type_low_nibble: 0,
            packet_id: 0,
            variable_header_properties: None,
            reason_codes: vec![],
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::packets::reason_codes::SUBACK;
    use crate::packets::suback::builder::SubAckBuilder;
    use crate::packets::suback::SubAck;
    use crate::packets::{BuilderLifecycle, Decoder, Encoder, Properties};
    use crate::primitive_types::Utf8EncodedString;
    use crate::properties::Property;

    #[test]
    pub fn should_encode_decode_packet() {
        let mut original_packet = SubAckBuilder::new();
        let props = vec![Property::ContentType(Utf8EncodedString(String::from(
            "hello",
        )))];
        original_packet = original_packet.set_packet_id(100);
        let reason_codes = vec![
            SUBACK::WildcardSubscriptionsNotSupported,
            SUBACK::SubscriptionIdentifiersNotSupported,
        ];
        original_packet = original_packet.set_reason_code(reason_codes);
        original_packet.set_variable_header_properties(Some(props));
        let built_packet = original_packet.build().unwrap();

        let mut serialized_packet = SubAck::encode(
            built_packet.packet_type,
            built_packet.packet_type_low_nibble,
            &built_packet,
        )
        .unwrap();

        let deserialized_packet = SubAck::decode(&mut serialized_packet).unwrap();

        assert_eq!(built_packet, deserialized_packet);
    }
}
