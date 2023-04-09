mod builder;
mod deser;
mod validation;

use crate::packets::PacketTypes;
use crate::properties::Property;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UnSubscribe {
    //fixed header
    pub packet_type: u8,
    pub packet_type_low_nibble: u8,

    //variable header
    pub packet_id: u16,
    pub variable_header_properties: Option<Vec<Property>>,

    //payload
    pub topic_filters: Vec<String>,
}

impl Default for UnSubscribe {
    fn default() -> Self {
        UnSubscribe {
            packet_type: PacketTypes::Unsubscribe as u8,
            packet_type_low_nibble: 0,
            packet_id: 0,
            variable_header_properties: None,
            topic_filters: vec![],
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::packets::unsubscribe::builder::UnsubscribeBuilder;
    use crate::packets::unsubscribe::UnSubscribe;
    use crate::packets::{BuilderLifecycle, Decoder, Encoder, Properties};
    use crate::primitive_types::Utf8StringPair;
    use crate::properties::Property;

    #[test]
    pub fn should_encode_decode_packet() {
        let mut original_packet = UnsubscribeBuilder::new();
        let props = vec![Property::User(Utf8StringPair(
            String::from(""),
            String::from(""),
        ))];
        original_packet = original_packet.set_packet_id(101);
        original_packet.set_variable_header_properties(Some(props));
        let built_packet = original_packet.build().unwrap();
        let mut serialized_packet = UnSubscribe::encode(
            built_packet.packet_type,
            built_packet.packet_type_low_nibble,
            &built_packet,
        )
        .unwrap();

        let deserialized_packet = UnSubscribe::decode(&mut serialized_packet).unwrap();

        assert_eq!(built_packet, deserialized_packet);
    }
}
