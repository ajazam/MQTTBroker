use crate::decode::{decode_property, varint};
use crate::packets::pubrel::PubRel;
use crate::packets::reason_codes::{DecodeReasonCode, PUBREL};
use crate::packets::{encode_properties, Decoder, Encoder, GeneratePacketParts};
use bytes::{Buf, BufMut, BytesMut};
use std::error::Error;

impl GeneratePacketParts for PubRel {
    fn generate_variable_header(&self) -> BytesMut {
        // need to optimise the capacity value
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

impl Encoder<PubRel> for PubRel {}

impl Decoder<PubRel> for PubRel {
    fn decode(bytes: &mut BytesMut) -> Result<PubRel, Box<dyn Error + Send + Sync>> {
        // fixed header
        let packet_with_flags = bytes.get_u8();
        let packet_type = packet_with_flags >> 4;
        let packet_type_low_nibble = packet_with_flags & 0x0f;
        // remaining length
        let _packet_size = varint(bytes).unwrap();

        // packet_id
        let packet_id = bytes.get_u16();

        // reason_code
        let reason_code = PUBREL::decode(bytes.get_u8())?;

        // variable_header_properties
        let variable_header_properties = decode_property(bytes);

        // no_payload

        Ok(PubRel {
            packet_type,
            packet_type_low_nibble,
            packet_id,
            reason_code,
            variable_header_properties,
        })
    }
}
