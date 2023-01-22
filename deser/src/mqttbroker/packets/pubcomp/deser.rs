use crate::decode::{decode_property, varint};
use crate::mqttbroker::packets::pubcomp::PubComp;
use crate::mqttbroker::packets::reason_codes::{DecodeReasonCode, PUBCOMP};
use crate::mqttbroker::packets::{encode_properties, Decoder, Encoder, GeneratePacketParts};
use bytes::{Buf, BufMut, BytesMut};

impl GeneratePacketParts for PubComp {
    fn generate_variable_header(&self) -> BytesMut {
        let mut variable_header = BytesMut::with_capacity(200);
        // packet_identifier
        variable_header.put_u16(self.packet_id);
        // reason code
        variable_header.put_u8(self.reason_code.clone() as u8);
        // variable header properties
        variable_header = encode_properties(variable_header, &self.variable_header_properties);

        variable_header
    }

    fn generate_payload(&self) -> BytesMut {
        // no payload
        BytesMut::with_capacity(0)
    }
}

impl Encoder<PubComp> for PubComp {}

impl Decoder<PubComp> for PubComp {
    fn decode(bytes: &mut BytesMut) -> anyhow::Result<PubComp> {
        // fixed header
        let packet_with_flags = bytes.get_u8();
        let packet_type = packet_with_flags >> 4;
        let packet_type_low_nibble = packet_with_flags & 0x0f;
        // remaining length
        let _packet_size = varint(bytes).unwrap();

        // packet identifier
        let packet_id = bytes.get_u16();
        // reason code
        let reason_code = PUBCOMP::decode(bytes.get_u8())?;

        // variable header properties
        let variable_header_properties = decode_property(bytes);

        Ok(PubComp {
            packet_type,
            packet_type_low_nibble,
            packet_id,
            reason_code,
            variable_header_properties,
        })
    }
}
