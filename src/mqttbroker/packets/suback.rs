use crate::decode::{decode_property, varint};
use crate::mqttbroker::packets::reason_codes::{DecodeReasonCode, SUBACK};
use crate::mqttbroker::packets::{
    encode_properties, reason_codes, BuilderLifecycle, Decoder, Encoder, GeneratePacketParts,
    PacketTypes, Properties,
};
use crate::mqttbroker::properties::Property;
use bytes::{Buf, BufMut, BytesMut};
use std::env::var;
use std::io::Error;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SubAck {
    //fixed header
    packet_type: u8,
    packet_type_low_nibble: u8,

    //variable header
    packet_id: u16,
    variable_header_properties: Option<Vec<Property>>,

    //payload
    reason_codes: Vec<SUBACK>,
}

pub mod encode_validation {}

pub mod decode_validate {}

impl Default for SubAck {
    fn default() -> Self {
        SubAck {
            packet_type: PacketTypes::Suback as u8,
            packet_type_low_nibble: 0,
            packet_id: 0,
            variable_header_properties: None,
            reason_codes: vec![],
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SubAckBuilder {
    pub packet: SubAck,
}

impl SubAckBuilder {
    pub fn set_packet_id(mut self, packet_id: u16) -> Self {
        self.packet.packet_id = packet_id;
        self
    }
    pub fn set_reason_code(mut self, reason_code: Vec<SUBACK>) -> Self {
        // need to have a check here for a valid reason code.
        self.packet.reason_codes = reason_code;
        self
    }
}

impl Properties for SubAckBuilder {
    fn packet_type(&self) -> PacketTypes {
        PacketTypes::Suback
    }

    fn packet_type_string(&self) -> String {
        String::from("SUBACK")
    }

    fn variable_header_properties(&self) -> &Option<Vec<Property>> {
        &self.packet.variable_header_properties
    }

    fn set_variable_header_properties(&mut self, p: Option<Vec<Property>>) {
        self.packet.variable_header_properties = p;
    }
}

impl GeneratePacketParts for SubAck {
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
        for r in self.reason_codes.clone() {
            payload.put_u8(r as u8);
        }
        payload
    }
}

impl BuilderLifecycle<SubAck, Error> for SubAckBuilder {
    fn new() -> Self {
        Default::default()
    }

    fn build(self) -> Result<SubAck, Error> {
        let suback_packet = self.packet;
        Ok(suback_packet)
    }
}

impl Encoder<SubAck> for SubAck {}

impl Decoder<SubAck> for SubAck {
    fn decode(bytes: &mut BytesMut) -> anyhow::Result<SubAck> {
        let packet_type_with_flags = bytes.get_u8();
        let packet_type = packet_type_with_flags >> 4;
        let packet_type_low_nibble = packet_type_with_flags & 0x0f;
        //remaining length
        let _remaining_length = varint(bytes).unwrap();

        // packet_id

        let packet_id = bytes.get_u16();

        //variable header properties
        let variable_header_properties = decode_property(bytes);
        let mut reason_codes: Vec<SUBACK> = vec![];
        for r in bytes.to_vec() {
            reason_codes.push(SUBACK::decode(r)?);
        }
        Ok(SubAck {
            packet_type,
            packet_type_low_nibble,
            packet_id,
            variable_header_properties,
            reason_codes,
        })
    }
}

#[cfg(test)]
pub mod test {
    use crate::mqttbroker::packets::reason_codes::SUBACK;
    use crate::mqttbroker::packets::suback::{SubAck, SubAckBuilder};
    use crate::mqttbroker::packets::{BuilderLifecycle, Decoder, Encoder, Properties};
    use crate::mqttbroker::primitive_types::Utf8EncodedString;
    use crate::mqttbroker::properties::Property;

    #[test]
    pub fn should_encode_decode_packet() {
        let mut original_packet = SubAckBuilder::new();
        let props = vec![Property::ContentType(Utf8EncodedString(String::from(
            "hello",
        )))];
        original_packet = original_packet.set_packet_id(100);
        let reason_codes = vec![
            SUBACK::WildcardSubscriptionsNotSupported,
            SUBACK::SubscriptionIdentifiersNotSupported,
        ];
        original_packet = original_packet.set_reason_code(reason_codes);
        original_packet.set_variable_header_properties(Some(props));
        let built_packet = original_packet.build().unwrap();

        let mut serialized_packet = SubAck::encode(
            built_packet.packet_type,
            built_packet.packet_type_low_nibble,
            &built_packet,
        )
        .unwrap();

        let deserialized_packet = SubAck::decode(&mut serialized_packet).unwrap();

        assert_eq!(built_packet, deserialized_packet);
    }
}
