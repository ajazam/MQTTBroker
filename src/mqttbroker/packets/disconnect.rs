use crate::decode::{decode_property, varint};
use crate::mqttbroker::packets::reason_codes::{DecodeReasonCode, DISCONNECT};
use crate::mqttbroker::packets::{
    encode_properties, BuilderLifecycle, Decoder, Encoder, GeneratePacketParts, PacketTypes,
    Properties,
};
use crate::mqttbroker::properties::Property;
use bytes::{Buf, BufMut, BytesMut};
use std::env::var;
use std::io::Error;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Disconnect {
    //fixed header
    packet_type: u8,
    packet_type_low_nibble: u8,

    // variable header
    reason_code: DISCONNECT,

    variable_header_properties: Option<Vec<Property>>,
    // no payload
}

pub mod encode_validation {}

pub mod decode_validate {}

impl Default for Disconnect {
    fn default() -> Self {
        Disconnect {
            packet_type: PacketTypes::Disconnect as u8,
            packet_type_low_nibble: 0,
            reason_code: DISCONNECT::NormalDisconnection,
            variable_header_properties: None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct DisconnectBuilder {
    pub packet: Disconnect,
}

impl DisconnectBuilder {
    pub fn set_reason_code(mut self, reason_code: DISCONNECT) -> Self {
        self.packet.reason_code = reason_code;
        self
    }
}

impl Properties for DisconnectBuilder {
    fn packet_type(&self) -> PacketTypes {
        PacketTypes::Disconnect
    }

    fn packet_type_string(&self) -> String {
        String::from("DISCONNECT")
    }

    fn variable_header_properties(&self) -> &Option<Vec<Property>> {
        &self.packet.variable_header_properties
    }

    fn set_variable_header_properties(&mut self, p: Option<Vec<Property>>) {
        self.packet.variable_header_properties = p;
    }
}

impl GeneratePacketParts for Disconnect {
    fn generate_variable_header(&self) -> BytesMut {
        // need to optimise the capacity value
        let mut variable_header = BytesMut::with_capacity(200);
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

impl BuilderLifecycle<Disconnect, Error> for DisconnectBuilder {
    fn new() -> Self {
        Default::default()
    }

    fn build(self) -> Result<Disconnect, Error> {
        let packet = self.packet;
        Ok(packet)
    }
}

impl Encoder<Disconnect> for Disconnect {}

impl Decoder<Disconnect> for Disconnect {
    fn decode(bytes: &mut BytesMut) -> anyhow::Result<Disconnect> {
        // fixed header
        let packet_with_flags = bytes.get_u8();
        let packet_type = packet_with_flags >> 4;
        let packet_type_low_nibble = packet_with_flags & 0x0f;

        // remaining length
        let _packet_size = varint(bytes).unwrap();

        // reason_code
        let reason_code = DISCONNECT::decode(bytes.get_u8())?;

        // variable_header_properties
        let variable_header_properties = decode_property(bytes);

        // no_payload

        Ok(Disconnect {
            packet_type,
            packet_type_low_nibble,
            reason_code,
            variable_header_properties,
        })
    }
}

#[cfg(test)]
pub mod test {
    use crate::mqttbroker::packets::disconnect::{Disconnect, DisconnectBuilder};
    use crate::mqttbroker::packets::reason_codes::DISCONNECT;
    use crate::mqttbroker::packets::{BuilderLifecycle, Decoder, Encoder, Properties};
    use crate::mqttbroker::primitive_types::FourByteInteger;
    use crate::mqttbroker::properties::Property;

    #[test]
    pub fn show_encode_decode() {
        let mut original_packet = DisconnectBuilder::new();
        let props = vec![Property::SessionExpiryInterval(FourByteInteger(0x110011))];

        original_packet =
            original_packet.set_reason_code(DISCONNECT::WildcardSubscriptionsNotSupported);
        original_packet.set_variable_header_properties(Some(props));

        let build_packet = original_packet.build().unwrap();
        let mut serialized_packet = Disconnect::encode(
            build_packet.packet_type,
            build_packet.packet_type_low_nibble,
            &build_packet,
        )
        .unwrap();
        let deserialized_packet = Disconnect::decode(&mut serialized_packet).unwrap();

        assert_eq!(build_packet, deserialized_packet);
    }
}
