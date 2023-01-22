use crate::decode::{decode_property, varint};
use crate::mqttbroker::packets::reason_codes::{DecodeReasonCode, AUTH};
use crate::mqttbroker::packets::{
    encode_properties, BuilderLifecycle, Decoder, Encoder, GeneratePacketParts, PacketTypes,
    Properties,
};
use crate::mqttbroker::properties::Property;
use bytes::{Buf, BufMut, BytesMut};
use std::io::Error;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Auth {
    //fixed header
    packet_type: u8,

    packet_type_low_nibble: u8,

    // variable header
    reason_code: AUTH,
    variable_header_properties: Option<Vec<Property>>,
    //payload
    // no payload
}

pub mod encode_validation {}

pub mod decode_validate {}

impl Default for Auth {
    fn default() -> Self {
        Auth {
            packet_type: PacketTypes::Auth as u8,
            packet_type_low_nibble: 0,
            reason_code: AUTH::Success,
            variable_header_properties: None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct AuthBuilder {
    pub packet: Auth,
}

impl AuthBuilder {
    pub fn set_reason_code(mut self, reason_code: AUTH) -> Self {
        self.packet.reason_code = reason_code;
        self
    }
}

impl Properties for AuthBuilder {
    fn packet_type(&self) -> PacketTypes {
        PacketTypes::Auth
    }

    fn packet_type_string(&self) -> String {
        String::from("AUTH")
    }

    fn variable_header_properties(&self) -> &Option<Vec<Property>> {
        &self.packet.variable_header_properties
    }

    fn set_variable_header_properties(&mut self, p: Option<Vec<Property>>) {
        self.packet.variable_header_properties = p;
    }
}

impl GeneratePacketParts for Auth {
    fn generate_variable_header(&self) -> BytesMut {
        // need to optimise the capacity value
        let mut variable_header = BytesMut::with_capacity(200);
        // reason_code
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

impl BuilderLifecycle<Auth, Error> for AuthBuilder {
    fn new() -> Self {
        Default::default()
    }

    fn build(self) -> Result<Auth, Error> {
        let packet = self.packet;
        Ok(packet)
    }
}

impl Encoder<Auth> for Auth {}

impl Decoder<Auth> for Auth {
    fn decode(bytes: &mut BytesMut) -> anyhow::Result<Auth> {
        // fixed header
        let packet_with_flags = bytes.get_u8();
        let packet_type = packet_with_flags >> 4;
        let packet_type_low_nibble = packet_with_flags & 0x0f;
        let packet_size = varint(bytes).unwrap();
        // reason code
        let reason_code = AUTH::decode(bytes.get_u8())?;

        // variable header properties

        let variable_header_properties = decode_property(bytes);

        Ok(Auth {
            packet_type,
            packet_type_low_nibble,
            reason_code,
            variable_header_properties,
        })
    }
}

#[cfg(test)]
pub mod test {
    use crate::mqttbroker::packets::auth::{Auth, AuthBuilder};
    use crate::mqttbroker::packets::reason_codes::AUTH;
    use crate::mqttbroker::packets::{BuilderLifecycle, Decoder, Encoder, Properties};
    use crate::mqttbroker::primitive_types::Utf8EncodedString;
    use crate::mqttbroker::properties::Property;

    #[test]
    pub fn should_encode_decode_packet() {
        let mut original_packet = AuthBuilder::new();
        let props = vec![Property::AuthenticationMethod(Utf8EncodedString(
            String::from("my secret method"),
        ))];

        original_packet = original_packet.set_reason_code(AUTH::ContinueAuthentication);
        original_packet.set_variable_header_properties(Some(props));

        let build_packet = original_packet.build().unwrap();
        let mut serialized_packet = Auth::encode(
            build_packet.packet_type,
            build_packet.packet_type_low_nibble,
            &build_packet,
        )
        .unwrap();
        let deserialized_packet = Auth::decode(&mut serialized_packet).unwrap();

        assert_eq!(build_packet, deserialized_packet);
    }
}
