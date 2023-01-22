use crate::decode::varint;
use crate::mqttbroker::packets::pingresp::PingResp;
use crate::mqttbroker::packets::{Decoder, Encoder};
use bytes::{Buf, BytesMut};

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
