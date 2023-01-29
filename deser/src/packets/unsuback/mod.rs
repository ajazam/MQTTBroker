mod builder;
pub mod deser;
mod validation;

use crate::packets::reason_codes::UNSUBACK;
use crate::packets::PacketTypes;
use crate::properties::Property;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UnsubAck {
    //fixed header
    packet_type: u8,
    packet_type_low_nibble: u8,

    //variable header
    packet_id: u16,
    variable_header_properties: Option<Vec<Property>>,

    //payload
    topic_filters: Vec<UNSUBACK>,
}

impl Default for UnsubAck {
    fn default() -> Self {
        UnsubAck {
            packet_type: PacketTypes::Unsuback as u8,
            packet_type_low_nibble: 0,
            packet_id: 0,
            variable_header_properties: None,
            topic_filters: vec![],
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::packets::reason_codes::UNSUBACK;
    use crate::packets::unsuback::builder::UnSubAckBuilder;
    use crate::packets::unsuback::UnsubAck;
    use crate::packets::{BuilderLifecycle, Decoder, Encoder, Properties};
    use crate::primitive_types::Utf8EncodedString;
    use crate::properties::Property;

    #[test]
    pub fn should_encode_decode() {
        let mut original_packet = UnSubAckBuilder::new();
        let props = vec![Property::ReasonString(Utf8EncodedString(String::from(
            "hello",
        )))];
        original_packet = original_packet.set_packet_id(1001);
        original_packet.set_variable_header_properties(Some(props));
        let topic_filters = vec![UNSUBACK::Success];

        original_packet = original_packet.set_topic_filters(topic_filters);
        let built_packet = original_packet.build().unwrap();

        let mut serialized_packet = UnsubAck::encode(
            built_packet.packet_type,
            built_packet.packet_type_low_nibble,
            &built_packet,
        )
        .unwrap();

        let deserialized_packet = UnsubAck::decode(&mut serialized_packet).unwrap();

        assert_eq!(built_packet, deserialized_packet);
    }
}
