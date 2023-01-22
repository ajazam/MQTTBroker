mod builder;
mod deser;
mod validation;

use crate::mqttbroker::packets::reason_codes::DISCONNECT;
use crate::mqttbroker::packets::PacketTypes;
use crate::mqttbroker::properties::Property;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Disconnect {
    //fixed header
    packet_type: u8,
    packet_type_low_nibble: u8,

    // variable header
    reason_code: DISCONNECT,

    variable_header_properties: Option<Vec<Property>>,
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
    use crate::mqttbroker::packets::disconnect::builder::DisconnectBuilder;
    use crate::mqttbroker::packets::disconnect::Disconnect;
    use crate::mqttbroker::packets::reason_codes::DISCONNECT;
    use crate::mqttbroker::packets::{BuilderLifecycle, Decoder, Encoder, Properties};
    use crate::mqttbroker::primitive_types::FourByteInteger;
    use crate::mqttbroker::properties::Property;

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
