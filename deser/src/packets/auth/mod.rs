mod builder;
mod deser;
mod validation;

use crate::packets::reason_codes::AUTH;
use crate::packets::PacketTypes;
use crate::properties::Property;

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

#[cfg(test)]
pub mod test {
    use crate::packets::auth::builder::AuthBuilder;
    use crate::packets::auth::Auth;
    use crate::packets::reason_codes::AUTH;
    use crate::packets::{BuilderLifecycle, Decoder, Encoder, Properties};
    use crate::primitive_types::Utf8EncodedString;
    use crate::properties::Property;

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
