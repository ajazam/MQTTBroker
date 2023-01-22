use crate::decode::{decode_property, varint};
use crate::mqttbroker::packets::connack::ConnAck;
use crate::mqttbroker::packets::{encode_properties, Decoder, Encoder, GeneratePacketParts};
use bytes::{Buf, BufMut, BytesMut};

impl GeneratePacketParts for ConnAck {
    fn generate_variable_header(&self) -> BytesMut {
        let mut variable_header = BytesMut::with_capacity(200);
        variable_header.put_u8(self.connect_ack_flags);
        variable_header.put_u8(self.connect_reason_code);

        encode_properties(variable_header, &self.variable_header_properties)
    }

    fn generate_payload(&self) -> BytesMut {
        BytesMut::with_capacity(0)
    }
}

impl Encoder<ConnAck> for ConnAck {}

impl Decoder<ConnAck> for ConnAck {
    fn decode(bytes: &mut BytesMut) -> anyhow::Result<ConnAck> {
        let packet_type_with_flags = bytes.get_u8();
        let packet_type = packet_type_with_flags >> 4;
        let packet_type_flags = packet_type_with_flags & 0x0f;
        let packet_size = varint(bytes).unwrap();
        let connect_ack_flags = bytes.get_u8();
        let connect_reason_code = bytes.get_u8();

        let variable_header_properties = decode_property(bytes);

        Ok(ConnAck {
            packet_type,
            packet_type_low_nibble: packet_type_flags,
            connect_ack_flags,
            connect_reason_code,
            variable_header_properties,
        })
    }
}
