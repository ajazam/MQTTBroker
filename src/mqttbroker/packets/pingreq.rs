use crate::decode::varint;
use crate::mqttbroker::packets::{
    BuilderLifecycle, Decoder, Encoder, GeneratePacketParts, PacketTypes,
};
use bytes::{Buf, BytesMut};
use std::io::Error;

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

pub mod encode_validation {}

pub mod decode_validate {}

impl Default for PingReq {
    fn default() -> Self {
        PingReq {
            packet_type: PacketTypes::Pingreq as u8,
            packet_type_low_nibble: 0,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PingReqBuilder {
    pub packet: PingReq,
}

impl GeneratePacketParts for PingReq {
    fn generate_variable_header(&self) -> BytesMut {
        // no variable header
        BytesMut::with_capacity(0)
    }

    fn generate_payload(&self) -> BytesMut {
        // no payload
        BytesMut::with_capacity(0)
    }
}

impl BuilderLifecycle<PingReq, Error> for PingReqBuilder {
    fn new() -> Self {
        Default::default()
    }

    fn build(self) -> Result<PingReq, Error> {
        let packet = self.packet;
        Ok(packet)
    }
}

impl Encoder<PingReq> for PingReq {}

impl Decoder<PingReq> for PingReq {
    fn decode(bytes: &mut BytesMut) -> anyhow::Result<PingReq> {
        // fixed header
        let packet_with_flags = bytes.get_u8();
        let packet_type = packet_with_flags >> 4;
        let packet_type_low_nibble = packet_with_flags & 0x0f;
        // remaining length
        let _packet_size = varint(bytes).unwrap();

        // variable header

        // no payload

        Ok(PingReq {
            packet_type,
            packet_type_low_nibble,
        })
    }
}
#[cfg(test)]
pub mod test {
    use crate::mqttbroker::packets::pingreq::{PingReq, PingReqBuilder};
    use crate::mqttbroker::packets::{BuilderLifecycle, Decoder, Encoder};

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
