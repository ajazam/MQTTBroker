use crate::decode::{decode_property, varint};
use crate::mqttbroker::packets::auth::Auth;
use crate::mqttbroker::packets::reason_codes::{DecodeReasonCode, AUTH};
use crate::mqttbroker::packets::{encode_properties, Decoder, Encoder, GeneratePacketParts};
use bytes::{Buf, BufMut, BytesMut};

impl GeneratePacketParts for Auth {
    fn generate_variable_header(&self) -> BytesMut {
        // need to optimise the capacity value
        let mut variable_header = BytesMut::with_capacity(200);
        // reason_code
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

impl Encoder<Auth> for Auth {}

impl Decoder<Auth> for Auth {
    fn decode(bytes: &mut BytesMut) -> anyhow::Result<Auth> {
        // fixed header
        let packet_with_flags = bytes.get_u8();
        let packet_type = packet_with_flags >> 4;
        let packet_type_low_nibble = packet_with_flags & 0x0f;

        let packet_size = varint(bytes).unwrap();
        // reason code
        let reason_code = AUTH::decode(bytes.get_u8())?;

        // variable header properties

        let variable_header_properties = decode_property(bytes);

        Ok(Auth {
            packet_type,
            packet_type_low_nibble,
            reason_code,
            variable_header_properties,
        })
    }
}
