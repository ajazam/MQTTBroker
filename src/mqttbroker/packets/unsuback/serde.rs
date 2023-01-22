use crate::decode::{decode_property, varint};
use crate::mqttbroker::packets::reason_codes::{DecodeReasonCode, UNSUBACK};
use crate::mqttbroker::packets::unsuback::UnsubAck;
use crate::mqttbroker::packets::{encode_properties, Decoder, Encoder, GeneratePacketParts};
use bytes::{Buf, BufMut, BytesMut};

impl GeneratePacketParts for UnsubAck {
    fn generate_variable_header(&self) -> BytesMut {
        let mut variable_header = BytesMut::with_capacity(200);

        //packet_id
        variable_header.put_u16(self.packet_id);

        //variable header properties
        variable_header = encode_properties(variable_header, &self.variable_header_properties);
        variable_header
    }

    fn generate_payload(&self) -> BytesMut {
        let mut payload = BytesMut::with_capacity(10);
        for r in self.topic_filters.clone() {
            payload.put_u8(r as u8);
        }

        payload
    }
}

impl Encoder<UnsubAck> for UnsubAck {}

impl Decoder<UnsubAck> for UnsubAck {
    fn decode(bytes: &mut BytesMut) -> anyhow::Result<UnsubAck> {
        let packet_type_with_flags = bytes.get_u8();
        let packet_type = packet_type_with_flags >> 4;
        let packet_type_low_nibble = packet_type_with_flags & 0x0f;
        //remaning length
        let remaining_length = varint(bytes).unwrap();

        // packet_id
        let packet_id = bytes.get_u16();

        // variable_header_properties
        let variable_header_properties = decode_property(bytes);

        // payload
        let mut topic_filters: Vec<UNSUBACK> = vec![];
        for tf in bytes.to_vec() {
            topic_filters.push(UNSUBACK::decode(tf)?);
        }
        Ok(UnsubAck {
            packet_type,
            packet_type_low_nibble,
            packet_id,
            variable_header_properties,
            topic_filters,
        })
    }
}
