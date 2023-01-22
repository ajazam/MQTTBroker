use crate::decode::{decode_property, varint};
use crate::mqttbroker::packets::disconnect::Disconnect;
use crate::mqttbroker::packets::reason_codes::{DecodeReasonCode, DISCONNECT};
use crate::mqttbroker::packets::{encode_properties, Decoder, Encoder, GeneratePacketParts};
use bytes::{Buf, BufMut, BytesMut};

impl GeneratePacketParts for Disconnect {
    fn generate_variable_header(&self) -> BytesMut {
        // need to optimise the capacity value
        let mut variable_header = BytesMut::with_capacity(200);
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

impl Encoder<Disconnect> for Disconnect {}

impl Decoder<Disconnect> for Disconnect {
    fn decode(bytes: &mut BytesMut) -> anyhow::Result<Disconnect> {
        // fixed header
        let packet_with_flags = bytes.get_u8();
        let packet_type = packet_with_flags >> 4;
        let packet_type_low_nibble = packet_with_flags & 0x0f;

        // remaining length
        let _packet_size = varint(bytes).unwrap();

        // reason_code
        let reason_code = DISCONNECT::decode(bytes.get_u8())?;

        // variable_header_properties
        let variable_header_properties = decode_property(bytes);

        // no_payload

        Ok(Disconnect {
            packet_type,
            packet_type_low_nibble,
            reason_code,
            variable_header_properties,
        })
    }
}
