use crate::decode::{decode_property, varint};
use crate::packets::reason_codes::{DecodeReasonCode, SUBACK};
use crate::packets::suback::SubAck;
use crate::packets::{encode_properties, Decoder, Encoder, GeneratePacketParts};
use bytes::{Buf, BufMut, BytesMut};
use std::error::Error;

impl GeneratePacketParts for SubAck {
    fn generate_variable_header(&self) -> BytesMut {
        // need to optimise the capacity value
        let mut variable_header = BytesMut::with_capacity(200);
        // packet_id
        variable_header.put_u16(self.packet_id);

        // variable header properties
        variable_header = encode_properties(variable_header, &self.variable_header_properties);
        variable_header
    }

    fn generate_payload(&self) -> BytesMut {
        let mut payload = BytesMut::with_capacity(10);
        for r in self.reason_codes.clone() {
            payload.put_u8(r as u8);
        }
        payload
    }
}

impl Encoder<SubAck> for SubAck {}

impl Decoder<SubAck> for SubAck {
    fn decode(bytes: &mut BytesMut) -> Result<SubAck, Box<dyn Error + Send + Sync>> {
        let packet_type_with_flags = bytes.get_u8();
        let packet_type = packet_type_with_flags >> 4;
        let packet_type_low_nibble = packet_type_with_flags & 0x0f;
        //remaining length
        let _remaining_length = varint(bytes).unwrap();

        // packet_id

        let packet_id = bytes.get_u16();

        //variable header properties
        let variable_header_properties = decode_property(bytes);
        let mut reason_codes: Vec<SUBACK> = vec![];
        for r in bytes.to_vec() {
            reason_codes.push(SUBACK::decode(r)?);
        }
        Ok(SubAck {
            packet_type,
            packet_type_low_nibble,
            packet_id,
            variable_header_properties,
            reason_codes,
        })
    }
}
