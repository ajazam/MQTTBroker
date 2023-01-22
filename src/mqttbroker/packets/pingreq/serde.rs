use crate::decode::varint;
use crate::mqttbroker::packets::pingreq::PingReq;
use crate::mqttbroker::packets::{Decoder, Encoder};
use bytes::{Buf, BytesMut};

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
