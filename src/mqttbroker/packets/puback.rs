use crate::decode::{decode_property, varint};
use crate::mqttbroker::packets::reason_codes::{DecodeReasonCode, ReasonCodeError, PUBACK};
use crate::mqttbroker::packets::{
    encode_properties, BuilderLifecycle, Decoder, Encoder, GeneratePacketParts, PacketTypes,
    Properties,
};
use crate::mqttbroker::properties::Property;
use bytes::{Buf, BufMut, BytesMut};
use std::io::Error;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PubAck {
    //fixed header
    pub packet_type: u8,

    pub packet_type_low_nibble: u8,

    //variable header
    pub packet_id: u16,
    pub reason_code: PUBACK,
    // not available if remaining length in fixed header
    // is 2, which means there is only a packet_id in variable header. If there is no Reason code then 0x00(Success) used by the client.
    pub variable_header_properties: Option<Vec<Property>>,
    //no payload
}

pub mod encode_validation {}

pub mod decode_validate {}

impl Default for PubAck {
    fn default() -> Self {
        PubAck {
            packet_type: PacketTypes::Puback as u8,
            packet_type_low_nibble: 0,
            packet_id: 0,
            reason_code: PUBACK::Success,
            variable_header_properties: None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct PubAckBuilder {
    pub packet: PubAck,
}

impl PubAckBuilder {
    pub fn set_packet_id(mut self, packet_id: u16) -> Self {
        self.packet.packet_id = packet_id;
        self
    }

    pub fn set_reason_code(mut self, reason_code: PUBACK) -> Self {
        self.packet.reason_code = reason_code;
        self
    }
}

impl Properties for PubAckBuilder {
    fn packet_type(&self) -> PacketTypes {
        PacketTypes::Puback
    }

    fn packet_type_string(&self) -> String {
        String::from("PUBACK")
    }

    fn variable_header_properties(&self) -> &Option<Vec<Property>> {
        &self.packet.variable_header_properties
    }

    fn set_variable_header_properties(&mut self, p: Option<Vec<Property>>) {
        self.packet.variable_header_properties = p;
    }
}

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

impl BuilderLifecycle<PubAck, Error> for PubAckBuilder {
    fn new() -> Self {
        Default::default()
    }

    fn build(self) -> Result<PubAck, Error> {
        let puback_packet = self.packet;
        Ok(puback_packet)
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
#[cfg(test)]
pub mod test {
    use crate::mqttbroker::packets::puback::{PubAck, PubAckBuilder};
    use crate::mqttbroker::packets::reason_codes::PUBACK;
    use crate::mqttbroker::packets::{BuilderLifecycle, Decoder, Encoder, Properties};
    use crate::mqttbroker::primitive_types::Byte;
    use crate::mqttbroker::properties::Property;

    #[test]
    pub fn should_encode_decode_packet() {
        let mut original_packet = PubAckBuilder::new();
        let props = vec![Property::PayloadFormatIndicator(Byte(1))];
        original_packet = original_packet.set_packet_id(79);
        original_packet = original_packet.set_reason_code(PUBACK::NotAuthorized);
        original_packet.set_variable_header_properties(Some(props));

        let built_packet = original_packet.build().unwrap();
        let mut serialized_packet = PubAck::encode(
            built_packet.packet_type,
            built_packet.packet_type_low_nibble,
            &built_packet,
        )
        .unwrap();

        let deserialized_packet = PubAck::decode(&mut serialized_packet).unwrap();

        assert_eq!(built_packet, deserialized_packet);
    }
}
