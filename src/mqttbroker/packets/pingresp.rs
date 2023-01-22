use crate::decode::varint;
use crate::mqttbroker::packets::{
    BuilderLifecycle, Decoder, Encoder, GeneratePacketParts, PacketTypes,
};
use bytes::{Buf, BytesMut};
use std::io::Error;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PingResp {
    //fixed header
    packet_type: u8,
    packet_type_low_nibble: u8,
    // variable header
    // no variable header

    //payload
    // no payload
}

pub mod encode_validation {}

pub mod decode_validate {}

impl Default for PingResp {
    fn default() -> Self {
        PingResp {
            packet_type: PacketTypes::Pingresp as u8,
            packet_type_low_nibble: 0,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PingRespBuilder {
    pub packet: PingResp,
}

impl GeneratePacketParts for PingResp {
    fn generate_variable_header(&self) -> BytesMut {
        // no variable header
        BytesMut::with_capacity(0)
    }

    fn generate_payload(&self) -> BytesMut {
        // no payload
        BytesMut::with_capacity(0)
    }
}

impl BuilderLifecycle<PingResp, Error> for PingRespBuilder {
    fn new() -> Self {
        Default::default()
    }

    fn build(self) -> Result<PingResp, Error> {
        let packet = self.packet;
        Ok(packet)
    }
}

impl Encoder<PingResp> for PingResp {}

impl Decoder<PingResp> for PingResp {
    fn decode(bytes: &mut BytesMut) -> anyhow::Result<PingResp> {
        // fixed header
        let packet_with_flags = bytes.get_u8();
        let packet_type = packet_with_flags >> 4;
        let packet_type_low_nibble = packet_with_flags & 0x0f;
        // remaining length
        let _packet_size = varint(bytes).unwrap();

        // variasble header

        // no payload

        Ok(PingResp {
            packet_type,
            packet_type_low_nibble,
        })
    }
}
#[cfg(test)]
pub mod test {
    use crate::mqttbroker::packets::pingresp::{PingResp, PingRespBuilder};
    use crate::mqttbroker::packets::{BuilderLifecycle, Decoder, Encoder};

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
