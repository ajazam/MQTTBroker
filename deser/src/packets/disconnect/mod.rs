mod builder;
mod deser;
mod validation;

use crate::packets::reason_codes::DISCONNECT;
use crate::packets::PacketTypes;
use crate::properties::Property;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Disconnect {
    //fixed header
    pub packet_type: u8,
    pub packet_type_low_nibble: u8,

    // variable header
    pub reason_code: DISCONNECT,

    pub variable_header_properties: Option<Vec<Property>>,
    // no payload
}

impl Default for Disconnect {
    fn default() -> Self {
        Disconnect {
            packet_type: PacketTypes::Disconnect as u8,
            packet_type_low_nibble: 0,
            reason_code: DISCONNECT::NormalDisconnection,
            variable_header_properties: None,
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::packets::disconnect::builder::DisconnectBuilder;
    use crate::packets::disconnect::Disconnect;
    use crate::packets::reason_codes::DISCONNECT;
    use crate::packets::{BuilderLifecycle, Decoder, Encoder, Properties};
    use crate::primitive_types::FourByteInteger;
    use crate::properties::Property;

    #[test]
    pub fn show_encode_decode() {
        let mut original_packet = DisconnectBuilder::new();
        let props = vec![Property::SessionExpiryInterval(FourByteInteger(0x110011))];

        original_packet =
            original_packet.set_reason_code(DISCONNECT::WildcardSubscriptionsNotSupported);
        original_packet.set_variable_header_properties(Some(props));

        let build_packet = original_packet.build().unwrap();
        let mut serialized_packet = Disconnect::encode(
            build_packet.packet_type,
            build_packet.packet_type_low_nibble,
            &build_packet,
        )
        .unwrap();
        let deserialized_packet = Disconnect::decode(&mut serialized_packet).unwrap();

        assert_eq!(build_packet, deserialized_packet);
    }
}
