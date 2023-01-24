use crate::decode::{decode_property, varint};
use crate::mqttbroker::packets::puback::PubAck;
use crate::mqttbroker::packets::reason_codes::{DecodeReasonCode, PUBACK};
use crate::mqttbroker::packets::{encode_properties, Decoder, Encoder, GeneratePacketParts};
use bytes::{Buf, BufMut, BytesMut};

impl GeneratePacketParts for PubAck {
    fn generate_variable_header(&self) -> BytesMut {
        // need to optimise the capacity value
        let mut variable_header = BytesMut::with_capacity(200);
        //packet identifier
        variable_header.put_u16(self.packet_id);
        //reason code
        variable_header.put_u8(self.reason_code.clone() as u8);
        //property
        variable_header = encode_properties(variable_header, &self.variable_header_properties);

        variable_header
    }

    fn generate_payload(&self) -> BytesMut {
        // no payload
        BytesMut::with_capacity(0)
    }
}

impl Encoder<PubAck> for PubAck {}

impl Decoder<PubAck> for PubAck {
    fn decode(bytes: &mut BytesMut) -> anyhow::Result<PubAck> {
        // fixed header
        let packet_type_with_flags = bytes.get_u8();
        let packet_type = packet_type_with_flags >> 4;
        let packet_type_low_nibble = packet_type_with_flags & 0x0f;
        // remaining length
        let _packet_size = varint(bytes).unwrap();
        // packet identifier
        let packet_id = bytes.get_u16();
        // reason code
        let reason_code = PUBACK::decode(bytes.get_u8())?;
        let variable_header_properties = decode_property(bytes);

        Ok(PubAck {
            packet_type,
            packet_type_low_nibble,
            packet_id,
            reason_code,
            variable_header_properties,
        })
    }
}
