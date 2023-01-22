use crate::decode::{decode_property, varint};
use crate::mqttbroker::packets::reason_codes::{DecodeReasonCode, PUBCOMP};
use crate::mqttbroker::packets::{
    encode_properties, reason_codes, BuilderLifecycle, Decoder, Encoder, GeneratePacketParts,
    PacketTypes, Properties,
};
use crate::mqttbroker::properties::Property;
use bytes::{Buf, BufMut, BytesMut};
use std::io::Error;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PubComp {
    //fixed header
    pub packet_type: u8,

    pub packet_type_low_nibble: u8,

    //variable header
    pub packet_id: u16,
    pub reason_code: PUBCOMP,
    // if the remaining length is 4 then property length is zero
    pub variable_header_properties: Option<Vec<Property>>,
    //no payload
}

pub mod encode_validation {}

pub mod decode_validate {}

impl Default for PubComp {
    fn default() -> Self {
        PubComp {
            packet_type: PacketTypes::Pubcomp as u8,
            packet_type_low_nibble: 0,
            packet_id: 0,
            reason_code: PUBCOMP::Success,
            variable_header_properties: None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PubCompBuilder {
    pub packet: PubComp,
}

impl PubCompBuilder {
    pub fn set_packet_id(mut self, packet_id: u16) -> Self {
        self.packet.packet_id = packet_id;
        self
    }

    pub fn set_reason_code(mut self, reason_codes: PUBCOMP) -> Self {
        self.packet.reason_code = reason_codes;
        self
    }
}

impl Properties for PubCompBuilder {
    fn packet_type(&self) -> PacketTypes {
        PacketTypes::Pubcomp
    }

    fn packet_type_string(&self) -> String {
        String::from("PUBCOMP")
    }

    fn variable_header_properties(&self) -> &Option<Vec<Property>> {
        &self.packet.variable_header_properties
    }

    fn set_variable_header_properties(&mut self, p: Option<Vec<Property>>) {
        self.packet.variable_header_properties = p;
    }
}

impl GeneratePacketParts for PubComp {
    fn generate_variable_header(&self) -> BytesMut {
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

impl BuilderLifecycle<PubComp, Error> for PubCompBuilder {
    fn new() -> Self {
        Default::default()
    }

    fn build(self) -> Result<PubComp, Error> {
        let packet = self.packet;
        Ok(packet)
    }
}

impl Encoder<PubComp> for PubComp {}

impl Decoder<PubComp> for PubComp {
    fn decode(bytes: &mut BytesMut) -> anyhow::Result<PubComp> {
        // fixed header
        let packet_with_flags = bytes.get_u8();
        let packet_type = packet_with_flags >> 4;
        let packet_type_low_nibble = packet_with_flags & 0x0f;
        // remaining length
        let _packet_size = varint(bytes).unwrap();

        // packet identifier
        let packet_id = bytes.get_u16();
        // reason code
        let reason_code = PUBCOMP::decode(bytes.get_u8())?;

        // variable header properties
        let variable_header_properties = decode_property(bytes);

        Ok(PubComp {
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
    use crate::mqttbroker::packets::pubcomp::{PubComp, PubCompBuilder};
    use crate::mqttbroker::packets::reason_codes::{DecodeReasonCode, PUBCOMP};
    use crate::mqttbroker::packets::{BuilderLifecycle, Decoder, Encoder, Properties};
    use crate::mqttbroker::primitive_types::{Utf8EncodedString, Utf8StringPair};
    use crate::mqttbroker::properties::Property;

    #[test]
    pub fn show_encode_decode_packet() {
        let mut original_packet = PubCompBuilder::new();
        let props = vec![
            Property::ReasonString(Utf8EncodedString(String::from("Cant be bothered"))),
            Property::User(Utf8StringPair(String::from("hello"), String::from("world"))),
        ];

        original_packet = original_packet.set_packet_id(0x3210);
        original_packet = original_packet.set_reason_code(PUBCOMP::PacketIdentifierNotFound);
        original_packet.set_variable_header_properties(Some(props));

        let build_packet = original_packet.build().unwrap();
        let mut serialized_packet = PubComp::encode(
            build_packet.packet_type,
            build_packet.packet_type_low_nibble,
            &build_packet,
        )
        .unwrap();
        let deserialized_packet = PubComp::decode(&mut serialized_packet).unwrap();

        assert_eq!(build_packet, deserialized_packet)
    }
}
