use crate::decode::{decode_property, varint};
use crate::mqttbroker::packets::reason_codes::{DecodeReasonCode, PUBREC};
use crate::mqttbroker::packets::{
    encode_properties, BuilderLifecycle, Decoder, Encoder, GeneratePacketParts, PacketTypes,
    Properties,
};
use crate::mqttbroker::properties::Property;
use bytes::{Buf, BufMut, BytesMut};
use std::io::Error;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PubRec {
    //fixed header
    pub packet_type: u8,

    pub packet_type_low_nibble: u8,

    //variable header
    pub packet_id: u16,
    pub reason_code: PUBREC,
    // if the remaining length is 4 then property length is zero
    pub variable_header_properties: Option<Vec<Property>>,
    //no payload
}

pub mod encode_validation {}

pub mod decode_validate {}

impl Default for PubRec {
    fn default() -> Self {
        PubRec {
            packet_type: PacketTypes::Pubrec as u8,
            packet_type_low_nibble: 0,
            packet_id: 0,
            reason_code: PUBREC::Success,
            variable_header_properties: None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PubRecBuilder {
    pub packet: PubRec,
}

impl PubRecBuilder {
    pub fn set_packet_id(mut self, packet_id: u16) -> Self {
        self.packet.packet_id = packet_id;
        self
    }

    pub fn set_reason_code(mut self, reason_code: PUBREC) -> Self {
        self.packet.reason_code = reason_code;
        self
    }
}

impl Properties for PubRecBuilder {
    fn packet_type(&self) -> PacketTypes {
        PacketTypes::Puback
    }

    fn packet_type_string(&self) -> String {
        String::from("PUBREC")
    }

    fn variable_header_properties(&self) -> &Option<Vec<Property>> {
        &self.packet.variable_header_properties
    }

    fn set_variable_header_properties(&mut self, p: Option<Vec<Property>>) {
        self.packet.variable_header_properties = p;
    }
}

impl GeneratePacketParts for PubRec {
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

impl BuilderLifecycle<PubRec, Error> for PubRecBuilder {
    fn new() -> Self {
        Default::default()
    }

    fn build(self) -> Result<PubRec, Error> {
        let packet = self.packet;
        Ok(packet)
    }
}

impl Encoder<PubRec> for PubRec {}

impl Decoder<PubRec> for PubRec {
    fn decode(bytes: &mut BytesMut) -> anyhow::Result<PubRec> {
        // fixed header
        let packet_with_flags = bytes.get_u8();
        let packet_type = packet_with_flags >> 4;
        let packet_type_low_nibble = packet_with_flags & 0x0f;
        // remaining length
        let _packet_size = varint(bytes).unwrap();

        // packet identifier
        let packet_id = bytes.get_u16();
        // reason code
        let reason_code = PUBREC::decode(bytes.get_u8())?;

        // variable header properties
        let variable_header_properties = decode_property(bytes);

        Ok(PubRec {
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
    use crate::mqttbroker::packets::pubrec::{PubRec, PubRecBuilder};
    use crate::mqttbroker::packets::reason_codes::PUBREC;
    use crate::mqttbroker::packets::{BuilderLifecycle, Decoder, Encoder, Properties};
    use crate::mqttbroker::primitive_types::Utf8StringPair;
    use crate::mqttbroker::properties::Property;

    #[test]
    pub fn show_encode_decode_packet() {
        let mut original_packet = PubRecBuilder::new();
        let props = vec![Property::User(Utf8StringPair(
            String::from("abc"),
            String::from("abc"),
        ))];
        original_packet = original_packet.set_packet_id(0x1234);
        original_packet = original_packet.set_reason_code(PUBREC::PacketIdentifierInUse);
        original_packet.set_variable_header_properties(Some(props));

        let build_packet = original_packet.build().unwrap();
        let mut serialized_packet = PubRec::encode(
            build_packet.packet_type,
            build_packet.packet_type_low_nibble,
            &build_packet,
        )
        .unwrap();

        let deserialized_packet = PubRec::decode(&mut serialized_packet).unwrap();

        assert_eq!(build_packet, deserialized_packet);
    }
}
