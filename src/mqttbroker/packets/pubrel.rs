use crate::decode::{decode_property, varint};
use crate::mqttbroker::packets::reason_codes::{DecodeReasonCode, PUBREC, PUBREL};
use crate::mqttbroker::packets::{
    encode_properties, BuilderLifecycle, Decoder, Encoder, GeneratePacketParts, PacketTypes,
    Properties,
};
use crate::mqttbroker::properties::Property;
use bytes::{Buf, BufMut, BytesMut};
use std::io::Error;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PubRel {
    //fixed header
    pub packet_type: u8,

    pub packet_type_low_nibble: u8,

    //variable header
    pub packet_id: u16,
    pub reason_code: PUBREL,
    // if the remaining length is 4 then property length is zero
    pub variable_header_properties: Option<Vec<Property>>,
    //payload
    //no payload
}

pub mod encode_validation {}

pub mod decode_validate {}

impl Default for PubRel {
    fn default() -> Self {
        PubRel {
            packet_type: PacketTypes::Pubrel as u8,
            packet_type_low_nibble: 0,
            packet_id: 0,
            reason_code: PUBREL::Success,
            variable_header_properties: None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PubRelBuilder {
    pub packet: PubRel,
}

impl PubRelBuilder {
    pub fn set_packet_id(mut self, packet_id: u16) -> Self {
        self.packet.packet_id = packet_id;
        self
    }

    pub fn set_reason_code(mut self, reason_code: PUBREL) -> Self {
        self.packet.reason_code = reason_code;
        self
    }
}

impl Properties for PubRelBuilder {
    fn packet_type(&self) -> PacketTypes {
        PacketTypes::Pubrel
    }

    fn packet_type_string(&self) -> String {
        String::from("PUBREL")
    }

    fn variable_header_properties(&self) -> &Option<Vec<Property>> {
        &self.packet.variable_header_properties
    }

    fn set_variable_header_properties(&mut self, p: Option<Vec<Property>>) {
        self.packet.variable_header_properties = p;
    }
}

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

impl BuilderLifecycle<PubRel, Error> for PubRelBuilder {
    fn new() -> Self {
        Default::default()
    }

    fn build(self) -> Result<PubRel, Error> {
        let packet = self.packet;
        Ok(packet)
    }
}

impl Encoder<PubRel> for PubRel {}

impl Decoder<PubRel> for PubRel {
    fn decode(bytes: &mut BytesMut) -> anyhow::Result<PubRel> {
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

#[cfg(test)]
pub mod test {
    use crate::mqttbroker::packets::pubrel::{PubRel, PubRelBuilder};
    use crate::mqttbroker::packets::reason_codes::PUBREL;
    use crate::mqttbroker::packets::{BuilderLifecycle, Decoder, Encoder, Properties};
    use crate::mqttbroker::primitive_types::{Utf8EncodedString, Utf8StringPair};
    use crate::mqttbroker::properties::Property;

    #[test]
    pub fn should_encode_decode_packet() {
        let mut original_packet = PubRelBuilder::new();
        let props = vec![
            Property::ReasonString(Utf8EncodedString(String::from("Cant be bothered"))),
            Property::User(Utf8StringPair(String::from("hello"), String::from("world"))),
        ];

        original_packet = original_packet.set_packet_id(0x3210);
        original_packet = original_packet.set_reason_code(PUBREL::PacketIdentifierNotFound);
        original_packet.set_variable_header_properties(Some(props));

        let build_packet = original_packet.build().unwrap();
        let mut serialized_packet = PubRel::encode(
            build_packet.packet_type,
            build_packet.packet_type_low_nibble,
            &build_packet,
        )
        .unwrap();
        let deserialized_packet = PubRel::decode(&mut serialized_packet).unwrap();

        assert_eq!(build_packet, deserialized_packet)
    }
}
