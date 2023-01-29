mod builder;
mod deser;
mod validation;

use crate::packets::PacketTypes;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PingReq {
    //fixed header
    packet_type: u8,
    packet_type_low_nibble: u8,
    // variable header
    // no variable header

    //payload
    // no payload
}

impl Default for PingReq {
    fn default() -> Self {
        PingReq {
            packet_type: PacketTypes::Pingreq as u8,
            packet_type_low_nibble: 0,
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::packets::pingreq::builder::PingReqBuilder;
    use crate::packets::pingreq::PingReq;
    use crate::packets::{BuilderLifecycle, Decoder, Encoder};

    #[test]
    pub fn should_encode_decode_packet() {
        let original_packet = PingReqBuilder::new();
        let build_packet = original_packet.build().unwrap();
        let mut serialized_packet = PingReq::encode(
            build_packet.packet_type,
            build_packet.packet_type_low_nibble,
            &build_packet,
        )
        .unwrap();
        let deserialized_packet = PingReq::decode(&mut serialized_packet).unwrap();

        assert_eq!(build_packet, deserialized_packet);
    }
}
