use crate::decode::{decode_property, varint};
use crate::mqttbroker::packets::reason_codes::{DecodeReasonCode, UNSUBACK};
use crate::mqttbroker::packets::{
    encode_properties, BuilderLifecycle, Decoder, Encoder, GeneratePacketParts, PacketTypes,
    Properties,
};
use crate::mqttbroker::properties::Property;
use bytes::{Buf, BufMut, BytesMut};
use std::arch::x86_64::_mm_pause;
use std::env::var;
use std::io::Error;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UnsubAck {
    //fixed header
    packet_type: u8,
    packet_type_low_nibble: u8,

    //variable header
    packet_id: u16,
    variable_header_properties: Option<Vec<Property>>,

    //payload
    topic_filters: Vec<UNSUBACK>,
}

pub mod encode_validation {}

pub mod decode_validate {}

impl Default for UnsubAck {
    fn default() -> Self {
        UnsubAck {
            packet_type: PacketTypes::Unsuback as u8,
            packet_type_low_nibble: 0,
            packet_id: 0,
            variable_header_properties: None,
            topic_filters: vec![],
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct UnSubAckBuilder {
    pub packet: UnsubAck,
}

impl UnSubAckBuilder {
    pub fn set_packet_id(mut self, packet_id: u16) -> Self {
        self.packet.packet_id = packet_id;
        self
    }

    pub fn set_topic_filters(mut self, topic_filter: Vec<UNSUBACK>) -> Self {
        self.packet.topic_filters = topic_filter;
        self
    }
}

impl Properties for UnSubAckBuilder {
    fn packet_type(&self) -> PacketTypes {
        PacketTypes::Unsuback
    }

    fn packet_type_string(&self) -> String {
        String::from("UNSUBACK")
    }

    fn variable_header_properties(&self) -> &Option<Vec<Property>> {
        &self.packet.variable_header_properties
    }

    fn set_variable_header_properties(&mut self, p: Option<Vec<Property>>) {
        self.packet.variable_header_properties = p;
    }
}

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

impl BuilderLifecycle<UnsubAck, Error> for UnSubAckBuilder {
    fn new() -> Self {
        Default::default()
    }

    fn build(self) -> Result<UnsubAck, Error> {
        let unsuback_packet = self.packet;
        Ok(unsuback_packet)
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

#[cfg(test)]
pub mod test {
    use crate::mqttbroker::packets::reason_codes::UNSUBACK;
    use crate::mqttbroker::packets::unsuback::{UnSubAckBuilder, UnsubAck};
    use crate::mqttbroker::packets::{BuilderLifecycle, Decoder, Encoder, Properties};
    use crate::mqttbroker::primitive_types::Utf8EncodedString;
    use crate::mqttbroker::properties::Property;

    #[test]
    pub fn should_encode_decode() {
        let mut original_packet = UnSubAckBuilder::new();
        let props = vec![Property::ReasonString(Utf8EncodedString(String::from(
            "hello",
        )))];
        original_packet = original_packet.set_packet_id(1001);
        original_packet.set_variable_header_properties(Some(props));
        let topic_filters = vec![UNSUBACK::Success];

        original_packet = original_packet.set_topic_filters(topic_filters);
        let built_packet = original_packet.build().unwrap();

        let mut serialized_packet = UnsubAck::encode(
            built_packet.packet_type,
            built_packet.packet_type_low_nibble,
            &built_packet,
        )
        .unwrap();

        let deserialized_packet = UnsubAck::decode(&mut serialized_packet).unwrap();

        assert_eq!(built_packet, deserialized_packet);
    }
}
