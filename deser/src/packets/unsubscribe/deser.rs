use crate::decode::{decode_property, utf8_string, varint};
use crate::encode::utf8_encoded_string;
use crate::packets::unsubscribe::UnSubscribe;
use crate::packets::{encode_properties, Decoder, Encoder, GeneratePacketParts};
use bytes::{Buf, BufMut, BytesMut};
use std::error::Error;

impl GeneratePacketParts for UnSubscribe {
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
        for tf in self.topic_filters.clone() {
            utf8_encoded_string("Topic Filter", &tf, &mut payload)
        }
        payload
    }
}

impl Encoder<UnSubscribe> for UnSubscribe {}

impl Decoder<UnSubscribe> for UnSubscribe {
    fn decode(bytes: &mut BytesMut) -> Result<UnSubscribe, Box<dyn Error + Send + Sync>> {
        let packet_type_with_flags = bytes.get_u8();
        let packet_type = packet_type_with_flags >> 4;
        let packet_type_low_nibble = packet_type_with_flags & 0x0f;

        // remaining length
        let _remaining_length = varint(bytes).unwrap();

        // packet_id
        let packet_id = bytes.get_u16();

        // variable_header_properties
        let variable_header_properties = decode_property(bytes);

        let mut topic_filters: Vec<String> = vec![];

        while !bytes.is_empty() {
            topic_filters.push(utf8_string(String::from("Topic Filter"), bytes).unwrap());
        }

        Ok(UnSubscribe {
            packet_type,
            packet_type_low_nibble,
            packet_id,
            variable_header_properties,
            topic_filters,
        })
    }
}
