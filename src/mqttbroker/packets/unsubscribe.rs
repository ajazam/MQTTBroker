use crate::decode::{decode_property, utf8_string, varint};
use crate::encode::utf8_encoded_string;
use crate::mqttbroker::packets::{
    encode_properties, BuilderLifecycle, Decoder, Encoder, GeneratePacketParts, PacketTypes,
    Properties,
};
use crate::mqttbroker::properties::Property;
use bytes::{Buf, BufMut, BytesMut};
use std::io::Error;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UnSubscribe {
    //fixed header
    packet_type: u8,
    packet_type_low_nibble: u8,

    //variable header
    packet_id: u16,
    variable_header_properties: Option<Vec<Property>>,

    //payload
    topic_filters: Vec<String>,
}

pub mod encode_validation {}

pub mod decode_validate {}

impl Default for UnSubscribe {
    fn default() -> Self {
        UnSubscribe {
            packet_type: PacketTypes::Unsubscribe as u8,
            packet_type_low_nibble: 0,
            packet_id: 0,
            variable_header_properties: None,
            topic_filters: vec![],
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct UnsubscribeBuilder {
    pub packet: UnSubscribe,
}

impl UnsubscribeBuilder {
    pub fn set_packet_id(mut self, packet_id: u16) -> Self {
        self.packet.packet_id = packet_id;
        self
    }

    pub fn set_topic_filters(mut self, topic_filters: Vec<String>) -> Self {
        self.packet.topic_filters = topic_filters;
        self
    }
}

impl Properties for UnsubscribeBuilder {
    fn packet_type(&self) -> PacketTypes {
        PacketTypes::Unsubscribe
    }

    fn packet_type_string(&self) -> String {
        String::from("UNSUBSCRIBE")
    }

    fn variable_header_properties(&self) -> &Option<Vec<Property>> {
        &self.packet.variable_header_properties
    }

    fn set_variable_header_properties(&mut self, p: Option<Vec<Property>>) {
        self.packet.variable_header_properties = p;
    }
}

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

impl BuilderLifecycle<UnSubscribe, Error> for UnsubscribeBuilder {
    fn new() -> Self {
        Default::default()
    }

    fn build(self) -> Result<UnSubscribe, Error> {
        let unsubscribe_packet = self.packet;
        Ok(unsubscribe_packet)
    }
}

impl Encoder<UnSubscribe> for UnSubscribe {}

impl Decoder<UnSubscribe> for UnSubscribe {
    fn decode(bytes: &mut BytesMut) -> anyhow::Result<UnSubscribe> {
        let packet_type_with_flags = bytes.get_u8();
        let packet_type = packet_type_with_flags >> 4;
        let packet_type_low_nibble = packet_type_with_flags & 0x0f;

        // remaining length
        let remaining_length = varint(bytes).unwrap();

        // packet_id
        let packet_id = bytes.get_u16();

        // variable_header_properties
        let variable_header_properties = decode_property(bytes);

        let mut topic_filters: Vec<String> = vec![];

        while (!bytes.is_empty()) {
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
#[cfg(test)]
pub mod test {
    use crate::mqttbroker::packets::unsubscribe::{UnSubscribe, UnsubscribeBuilder};
    use crate::mqttbroker::packets::{BuilderLifecycle, Decoder, Encoder, Properties};
    use crate::mqttbroker::primitive_types::Utf8StringPair;
    use crate::mqttbroker::properties::Property;

    #[test]
    pub fn should_encode_decode_packet() {
        let mut original_packet = UnsubscribeBuilder::new();
        let props = vec![Property::User(Utf8StringPair(
            String::from(""),
            String::from(""),
        ))];
        original_packet = original_packet.set_packet_id(101);
        original_packet.set_variable_header_properties(Some(props));
        let built_packet = original_packet.build().unwrap();
        let mut serialized_packet = UnSubscribe::encode(
            built_packet.packet_type,
            built_packet.packet_type_low_nibble,
            &built_packet,
        )
        .unwrap();

        let deserialized_packet = UnSubscribe::decode(&mut serialized_packet).unwrap();

        assert_eq!(built_packet, deserialized_packet);
    }
}
