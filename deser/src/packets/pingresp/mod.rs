mod builder;
mod deser;
mod validation;

use crate::packets::PacketTypes;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PingResp {
    //fixed header
    pub packet_type: u8,
    pub packet_type_low_nibble: u8,
    // variable header
    // no variable header

    //payload
    // no payload
}

impl Default for PingResp {
    fn default() -> Self {
        PingResp {
            packet_type: PacketTypes::Pingresp as u8,
            packet_type_low_nibble: 0,
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::packets::pingresp::builder::PingRespBuilder;
    use crate::packets::pingresp::PingResp;
    use crate::packets::{BuilderLifecycle, Decoder, Encoder};

    #[test]
    pub fn should_encode_decode_packet() {
        let original_packet = PingRespBuilder::new();
        let build_packet = original_packet.build().unwrap();
        let mut serialized_packet = PingResp::encode(
            build_packet.packet_type,
            build_packet.packet_type_low_nibble,
            &build_packet,
        )
        .unwrap();

        let deserialized_packet = PingResp::decode(&mut serialized_packet).unwrap();

        assert_eq!(build_packet, deserialized_packet);
    }
}
