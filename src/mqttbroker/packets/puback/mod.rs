mod builder;
mod serde;
mod validation;

use crate::mqttbroker::packets::reason_codes::PUBACK;
use crate::mqttbroker::packets::PacketTypes;
use crate::mqttbroker::properties::Property;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PubAck {
    //fixed header
    pub packet_type: u8,

    pub packet_type_low_nibble: u8,

    //variable header
    pub packet_id: u16,
    pub reason_code: PUBACK,
    // not available if remaining length in fixed header
    // is 2, which means there is only a packet_id in variable header. If there is no Reason code then 0x00(Success) used by the client.
    pub variable_header_properties: Option<Vec<Property>>,
    //no payload
}

impl Default for PubAck {
    fn default() -> Self {
        PubAck {
            packet_type: PacketTypes::Puback as u8,
            packet_type_low_nibble: 0,
            packet_id: 0,
            reason_code: PUBACK::Success,
            variable_header_properties: None,
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::mqttbroker::packets::puback::builder::PubAckBuilder;
    use crate::mqttbroker::packets::puback::PubAck;
    use crate::mqttbroker::packets::reason_codes::PUBACK;
    use crate::mqttbroker::packets::{BuilderLifecycle, Decoder, Encoder, Properties};
    use crate::mqttbroker::primitive_types::Byte;
    use crate::mqttbroker::properties::Property;

    #[test]
    pub fn should_encode_decode_packet() {
        let mut original_packet = PubAckBuilder::new();
        let props = vec![Property::PayloadFormatIndicator(Byte(1))];
        original_packet = original_packet.set_packet_id(79);
        original_packet = original_packet.set_reason_code(PUBACK::NotAuthorized);
        original_packet.set_variable_header_properties(Some(props));

        let built_packet = original_packet.build().unwrap();
        let mut serialized_packet = PubAck::encode(
            built_packet.packet_type,
            built_packet.packet_type_low_nibble,
            &built_packet,
        )
        .unwrap();

        let deserialized_packet = PubAck::decode(&mut serialized_packet).unwrap();

        assert_eq!(built_packet, deserialized_packet);
    }
}
