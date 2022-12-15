use crate::encode;
use crate::mqttbroker::packets::PacketTypes;
use crate::mqttbroker::primitive_types::{
    BinaryData, Byte, FourByteInteger, TwoByteInteger, Utf8EncodedString, Utf8StringPair,
    VariableByteInteger,
};
use bytes::{BufMut, BytesMut};
use std::collections::hash_map::Keys;

use std::collections::HashMap;
use std::convert::TryFrom;
use tracing::debug;

// #[derive(Debug, PartialEq, Eq, Hash, Clone)]
// pub enum PropertyType {
//     Byte { value: Byte },
//     FourByteInteger { value: FourByteInteger },
//     UTF8EncodedString { value: Utf8EncodedString },
//     BinaryData { value: BinaryData },
//     TwoByteInteger { value: TwoByteInteger },
//     UTF8StringPair { value: Utf8StringPair },
//     VariableByteInteger { value: VariableByteInteger },
// }
//
// impl PropertyType {}
//
// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub struct Property {
//     pub element_value: PropertyType,
//     pub property_identifier: u8,
// }
//
// impl Property {
//     pub fn new(ev: PropertyType, pi: u8) -> Self {
//         Property {
//             element_value: ev,
//             property_identifier: pi,
//         }
//     }
// }

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Property {
    PayloadFormatIndicator { value: Byte },
    MessageExpiryInterval { value: FourByteInteger },
    ContentType { value: Utf8EncodedString },
    ResponseTopic { value: Utf8EncodedString },
    CorrelationData { value: BinaryData },
    SubscriptionIdentifier { value: VariableByteInteger },
    SessionExpiryInterval { value: FourByteInteger },
    AssignedClientIdentifier { value: Utf8EncodedString },
    ServerKeepAlive { value: TwoByteInteger },
    AuthenticationMethod { value: Utf8EncodedString },
    AuthenticationData { value: BinaryData },
    RequestProblemInformation { value: Byte },
    WillDelayInterval { value: FourByteInteger },
    RequestResponseInformation { value: Byte },
    ResponseInformation { value: Utf8EncodedString },
    ServerReference { value: Utf8EncodedString },
    ReasonString { value: Utf8EncodedString },
    ReceiveMaximum { value: TwoByteInteger },
    TopicAliasMaximum { value: TwoByteInteger },
    TopicAlias { value: TwoByteInteger },
    MaximumQos { value: Byte },
    RetainAvailable { value: Byte },
    User { value: Utf8StringPair },
    MaximumPacketSize { value: FourByteInteger },
    WildcardSubscriptionAvailable { value: Byte },
    SubscriptionIdentifierAvailable { value: Byte },
    SharedSubscriptionAvailable { value: Byte },
}

trait PropertyType {
    fn property_identifier(&self) -> PropertyIdentifier;
}

impl PropertyType for Property {
    fn property_identifier(&self) -> PropertyIdentifier {
        match self {
            Property::PayloadFormatIndicator { .. } => PropertyIdentifier::PayloadFormatIndicator,
            Property::MessageExpiryInterval { .. } => PropertyIdentifier::MessageExpiryInterval,
            Property::ContentType { .. } => PropertyIdentifier::ContentType,
            Property::ResponseTopic { .. } => PropertyIdentifier::ResponseTopic,
            Property::CorrelationData { .. } => PropertyIdentifier::CorrelationData,
            Property::SubscriptionIdentifier { .. } => PropertyIdentifier::SubscriptionIdentifier,
            Property::SessionExpiryInterval { .. } => PropertyIdentifier::SessionExpiryInterval,
            Property::AssignedClientIdentifier { .. } => {
                PropertyIdentifier::AssignedClientIdentifier
            }
            Property::ServerKeepAlive { .. } => PropertyIdentifier::ServerKeepAlive,
            Property::AuthenticationMethod { .. } => PropertyIdentifier::AuthenticationMethod,
            Property::AuthenticationData { .. } => PropertyIdentifier::AuthenticationData,
            Property::RequestProblemInformation { .. } => {
                PropertyIdentifier::RequestProblemInformation
            }
            Property::WillDelayInterval { .. } => PropertyIdentifier::WillDelayInterval,
            Property::RequestResponseInformation { .. } => {
                PropertyIdentifier::RequestResponseInformation
            }
            Property::ResponseInformation { .. } => PropertyIdentifier::ResponseInformation,
            Property::ServerReference { .. } => PropertyIdentifier::ServerReference,
            Property::ReasonString { .. } => PropertyIdentifier::ReasonString,
            Property::ReceiveMaximum { .. } => PropertyIdentifier::ReceiveMaximum,
            Property::TopicAliasMaximum { .. } => PropertyIdentifier::TopicAliasMaximum,
            Property::TopicAlias { .. } => PropertyIdentifier::TopicAlias,
            Property::MaximumQos { .. } => PropertyIdentifier::MaximumQos,
            Property::RetainAvailable { .. } => PropertyIdentifier::RetainAvailable,
            Property::User { .. } => PropertyIdentifier::UserProperty,
            Property::MaximumPacketSize { .. } => PropertyIdentifier::MaximumPacketSize,
            Property::WildcardSubscriptionAvailable { .. } => {
                PropertyIdentifier::WildcardSubscriptionAvailable
            }
            Property::SubscriptionIdentifierAvailable { .. } => {
                PropertyIdentifier::SubscriptionIdentifierAvailable
            }
            Property::SharedSubscriptionAvailable { .. } => {
                PropertyIdentifier::SharedSubscriptionAvailable
            }
        }
    }
}

impl From<Property> for PropertyIdentifier {
    fn from(value: Property) -> PropertyIdentifier {
        match value {
            Property::PayloadFormatIndicator { .. } => PropertyIdentifier::PayloadFormatIndicator,
            Property::MessageExpiryInterval { .. } => PropertyIdentifier::MessageExpiryInterval,
            Property::ContentType { .. } => PropertyIdentifier::ContentType,
            Property::ResponseTopic { .. } => PropertyIdentifier::ResponseTopic,
            Property::CorrelationData { .. } => PropertyIdentifier::CorrelationData,
            Property::SubscriptionIdentifier { .. } => PropertyIdentifier::SubscriptionIdentifier,
            Property::SessionExpiryInterval { .. } => PropertyIdentifier::SessionExpiryInterval,
            Property::AssignedClientIdentifier { .. } => {
                PropertyIdentifier::AssignedClientIdentifier
            }
            Property::ServerKeepAlive { .. } => PropertyIdentifier::ServerKeepAlive,
            Property::AuthenticationMethod { .. } => PropertyIdentifier::AuthenticationMethod,
            Property::AuthenticationData { .. } => PropertyIdentifier::AuthenticationData,
            Property::RequestProblemInformation { .. } => {
                PropertyIdentifier::RequestProblemInformation
            }
            Property::WillDelayInterval { .. } => PropertyIdentifier::WillDelayInterval,
            Property::RequestResponseInformation { .. } => {
                PropertyIdentifier::RequestResponseInformation
            }
            Property::ResponseInformation { .. } => PropertyIdentifier::ResponseInformation,
            Property::ServerReference { .. } => PropertyIdentifier::ServerReference,
            Property::ReasonString { .. } => PropertyIdentifier::ReasonString,
            Property::ReceiveMaximum { .. } => PropertyIdentifier::ReceiveMaximum,
            Property::TopicAliasMaximum { .. } => PropertyIdentifier::TopicAliasMaximum,
            Property::TopicAlias { .. } => PropertyIdentifier::TopicAlias,
            Property::MaximumQos { .. } => PropertyIdentifier::MaximumQos,
            Property::RetainAvailable { .. } => PropertyIdentifier::RetainAvailable,
            Property::User { .. } => PropertyIdentifier::UserProperty,
            Property::MaximumPacketSize { .. } => PropertyIdentifier::MaximumPacketSize,
            Property::WildcardSubscriptionAvailable { .. } => {
                PropertyIdentifier::WildcardSubscriptionAvailable
            }
            Property::SubscriptionIdentifierAvailable { .. } => {
                PropertyIdentifier::SubscriptionIdentifierAvailable
            }
            Property::SharedSubscriptionAvailable { .. } => {
                PropertyIdentifier::SharedSubscriptionAvailable
            }
        }
    }
}

impl Property {
    pub fn encode(&self, encoded: &mut Vec<u8>) {
        match self {
            Property::PayloadFormatIndicator { value } => {
                Self::encode_byte(PropertyIdentifier::PayloadFormatIndicator, value, encoded)
            }

            Property::MessageExpiryInterval { value } => {
                Self::encode_four_byte_integer(
                    PropertyIdentifier::MessageExpiryInterval,
                    value,
                    encoded,
                );
            }

            Property::ContentType { value } => {
                Self::encode_utf8_encoded_string(PropertyIdentifier::ContentType, value, encoded);
            }

            Property::ResponseTopic { value } => {
                Self::encode_utf8_encoded_string(PropertyIdentifier::ResponseTopic, value, encoded);
            }

            Property::CorrelationData { value } => {
                Self::encode_binary_data(PropertyIdentifier::CorrelationData, value, encoded)
            }

            Property::SubscriptionIdentifier { value } => {
                Self::encode_variable_byte_integer(
                    PropertyIdentifier::SubscriptionIdentifier,
                    value,
                    encoded,
                );
            }

            Property::SessionExpiryInterval { value } => {
                Self::encode_four_byte_integer(
                    PropertyIdentifier::SessionExpiryInterval,
                    value,
                    encoded,
                );
            }

            Property::AssignedClientIdentifier { value } => {
                Self::encode_utf8_encoded_string(
                    PropertyIdentifier::AssignedClientIdentifier,
                    value,
                    encoded,
                );
            }

            Property::ServerKeepAlive { value } => {
                Self::encode_two_byte_integer(PropertyIdentifier::ServerKeepAlive, value, encoded);
            }

            Property::AuthenticationMethod { value } => {
                Self::encode_utf8_encoded_string(
                    PropertyIdentifier::AuthenticationMethod,
                    value,
                    encoded,
                );
            }

            Property::AuthenticationData { value } => {
                Self::encode_binary_data(PropertyIdentifier::AuthenticationData, value, encoded);
            }

            Property::RequestProblemInformation { value } => {
                Self::encode_byte(
                    PropertyIdentifier::RequestProblemInformation,
                    value,
                    encoded,
                );
            }

            Property::WillDelayInterval { value } => {
                Self::encode_four_byte_integer(
                    PropertyIdentifier::WillDelayInterval,
                    value,
                    encoded,
                );
            }

            Property::RequestResponseInformation { value } => {
                Self::encode_byte(
                    PropertyIdentifier::RequestResponseInformation,
                    value,
                    encoded,
                );
            }

            Property::ResponseInformation { value } => {
                Self::encode_utf8_encoded_string(
                    PropertyIdentifier::ResponseInformation,
                    value,
                    encoded,
                );
            }

            Property::ServerReference { value } => {
                Self::encode_utf8_encoded_string(
                    PropertyIdentifier::ServerReference,
                    value,
                    encoded,
                );
            }

            Property::ReasonString { value } => {
                Self::encode_utf8_encoded_string(PropertyIdentifier::ReasonString, value, encoded);
            }

            Property::ReceiveMaximum { value } => {
                Self::encode_two_byte_integer(PropertyIdentifier::ReceiveMaximum, value, encoded);
            }

            Property::TopicAliasMaximum { value } => {
                Self::encode_two_byte_integer(
                    PropertyIdentifier::TopicAliasMaximum,
                    value,
                    encoded,
                );
            }

            Property::TopicAlias { value } => {
                Self::encode_two_byte_integer(PropertyIdentifier::TopicAlias, value, encoded);
            }

            Property::MaximumQos { value } => {
                Self::encode_byte(PropertyIdentifier::MaximumQos, value, encoded);
            }

            Property::RetainAvailable { value } => {
                Self::encode_byte(PropertyIdentifier::RetainAvailable, value, encoded);
            }

            Property::User { value } => {
                Self::encode_utf8_string_pair(PropertyIdentifier::UserProperty, value, encoded);
            }

            Property::MaximumPacketSize { value } => {
                Self::encode_four_byte_integer(
                    PropertyIdentifier::MaximumPacketSize,
                    value,
                    encoded,
                );
            }

            Property::WildcardSubscriptionAvailable { value } => {
                Self::encode_byte(
                    PropertyIdentifier::WildcardSubscriptionAvailable,
                    value,
                    encoded,
                );
            }

            Property::SubscriptionIdentifierAvailable { value } => {
                Self::encode_byte(
                    PropertyIdentifier::SubscriptionIdentifierAvailable,
                    value,
                    encoded,
                );
            }

            Property::SharedSubscriptionAvailable { value } => {
                Self::encode_byte(
                    PropertyIdentifier::SharedSubscriptionAvailable,
                    value,
                    encoded,
                );
            }
        }
    }

    fn encode_byte(property_identifier: PropertyIdentifier, value: &Byte, encoded: &mut Vec<u8>) {
        encoded.put_u8(property_identifier as u8);
        encoded.put_u8(value.0);
    }

    fn encode_two_byte_integer(
        property_identifier: PropertyIdentifier,
        value: &TwoByteInteger,
        encoded: &mut Vec<u8>,
    ) {
        encoded.put_u8(property_identifier as u8);
        encoded.put_u16(value.0);
    }

    fn encode_four_byte_integer(
        property_identifier: PropertyIdentifier,
        value: &FourByteInteger,
        encoded: &mut Vec<u8>,
    ) {
        encoded.put_u8(property_identifier as u8);
        encoded.put_u32(value.0);
    }

    fn encode_variable_byte_integer(
        property_identifier: PropertyIdentifier,
        value: &VariableByteInteger,
        encoded: &mut Vec<u8>,
    ) {
        encoded.put_u8(property_identifier as u8);
        encoded.put_u32(value.0);
    }

    fn encode_utf8_encoded_string(
        property_identifier: PropertyIdentifier,
        value: &Utf8EncodedString,
        encoded: &mut Vec<u8>,
    ) {
        encoded.put_u8(property_identifier as u8);
        encoded.put_u16(value.0.len() as u16);
        encoded.put_slice(value.0.as_bytes());
    }

    fn encode_utf8_string_pair(
        property_identifier: PropertyIdentifier,
        value: &Utf8StringPair,
        encoded: &mut Vec<u8>,
    ) {
        encoded.put_u8(property_identifier as u8);
        encoded.put_u16(value.0.len() as u16);
        encoded.put_slice(value.0.as_bytes());
        encoded.put_u16(value.1.len() as u16);
        encoded.put_slice(value.1.as_bytes());
    }

    fn encode_binary_data(
        property_identifier: PropertyIdentifier,
        value: &BinaryData,
        encoded: &mut Vec<u8>,
    ) {
        encoded.put_u8(property_identifier as u8);
        encoded.put_u16(value.0.len() as u16);
        encoded.put_slice(value.0.as_slice());
    }
}

impl AsRef<PropertyIdentifier> for Property {
    fn as_ref(&self) -> &PropertyIdentifier {
        match self {
            Property::PayloadFormatIndicator { .. } => &PropertyIdentifier::PayloadFormatIndicator,
            Property::MessageExpiryInterval { .. } => &PropertyIdentifier::MessageExpiryInterval,
            Property::ContentType { .. } => &PropertyIdentifier::ContentType,
            Property::ResponseTopic { .. } => &PropertyIdentifier::ResponseTopic,
            Property::CorrelationData { .. } => &PropertyIdentifier::CorrelationData,
            Property::SubscriptionIdentifier { .. } => &PropertyIdentifier::SubscriptionIdentifier,
            Property::SessionExpiryInterval { .. } => &PropertyIdentifier::MessageExpiryInterval,
            Property::AssignedClientIdentifier { .. } => {
                &PropertyIdentifier::AssignedClientIdentifier
            }
            Property::ServerKeepAlive { .. } => &PropertyIdentifier::ServerKeepAlive,
            Property::AuthenticationMethod { .. } => &PropertyIdentifier::AuthenticationMethod,
            Property::AuthenticationData { .. } => &PropertyIdentifier::AuthenticationData,
            Property::RequestProblemInformation { .. } => {
                &PropertyIdentifier::RequestProblemInformation
            }
            Property::WillDelayInterval { .. } => &PropertyIdentifier::WillDelayInterval,
            Property::RequestResponseInformation { .. } => {
                &PropertyIdentifier::RequestResponseInformation
            }
            Property::ResponseInformation { .. } => &PropertyIdentifier::ResponseInformation,
            Property::ServerReference { .. } => &PropertyIdentifier::ServerReference,
            Property::ReasonString { .. } => &PropertyIdentifier::ReasonString,
            Property::ReceiveMaximum { .. } => &PropertyIdentifier::ReceiveMaximum,
            Property::TopicAliasMaximum { .. } => &PropertyIdentifier::TopicAliasMaximum,
            Property::TopicAlias { .. } => &PropertyIdentifier::TopicAlias,
            Property::MaximumQos { .. } => &PropertyIdentifier::MaximumQos,
            Property::RetainAvailable { .. } => &PropertyIdentifier::RetainAvailable,
            Property::User { .. } => &PropertyIdentifier::UserProperty,
            Property::MaximumPacketSize { .. } => &PropertyIdentifier::MaximumPacketSize,
            Property::WildcardSubscriptionAvailable { .. } => {
                &PropertyIdentifier::WildcardSubscriptionAvailable
            }
            Property::SubscriptionIdentifierAvailable { .. } => {
                &PropertyIdentifier::SubscriptionIdentifierAvailable
            }
            Property::SharedSubscriptionAvailable { .. } => {
                &PropertyIdentifier::SharedSubscriptionAvailable
            }
        }
    }
}

// impl Property {
//     pub fn encode(&self, encoded: &mut Vec<u8>) {
//         encoded.put_u8(self.property_identifier as u8);
//         match self.element_value {
//             PropertyType::Byte { ref value } => {
//                 encoded.put_u8(*value.as_ref());
//             }
//
//             PropertyType::FourByteInteger { ref value } => {
//                 encoded.put_u32(*value.as_ref());
//             }
//
//             PropertyType::UTF8EncodedString { ref value } => {
//                 let mut encoded_bytes = BytesMut::with_capacity(200);
//                 encode::encode_utf8_encoded_string(value.as_ref(), &mut encoded_bytes);
//                 encoded.put(encoded_bytes);
//             }
//
//             PropertyType::BinaryData { ref value } => {
//                 let mut encoded_bytes = BytesMut::with_capacity(200);
//                 let src = BytesMut::from(value.as_ref().as_slice());
//                 encode::encode_binary_data(&src, &mut encoded_bytes);
//                 encoded.put(encoded_bytes);
//             }
//
//             PropertyType::TwoByteInteger { ref value } => encoded.put_u16(*value.as_ref()),
//
//             PropertyType::UTF8StringPair { ref value } => {
//                 let mut encoded_bytes = BytesMut::with_capacity(200);
//                 encode::utf8_string_pair(&value.0, &value.1, &mut encoded_bytes);
//                 encoded.put(encoded_bytes);
//             }
//
//             PropertyType::VariableByteInteger { ref value } => {
//                 let mut encoded_bytes = BytesMut::with_capacity(200);
//                 encode::encode_variable_byte_integer(&value, &mut encoded_bytes);
//                 encoded.put(encoded_bytes);
//             }
//         }
//     }
// }

// impl From<Property> for u8 {
//     fn from(prop: Property) -> Self {
//         prop.property_identifier as u8
//     }
// }

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
#[repr(u8)]
pub enum PropertyIdentifier {
    PayloadFormatIndicator = 0x01,
    MessageExpiryInterval = 0x02,
    ContentType = 0x03,
    ResponseTopic = 0x08,
    CorrelationData = 0x09,
    SubscriptionIdentifier = 0x0b,
    SessionExpiryInterval = 0x11,
    AssignedClientIdentifier = 0x12,
    ServerKeepAlive = 0x13,
    AuthenticationMethod = 0x15,
    AuthenticationData = 0x16,
    RequestProblemInformation = 0x17,
    WillDelayInterval = 0x18,
    RequestResponseInformation = 0x19,
    ResponseInformation = 0x1a,
    ServerReference = 0x1c,
    ReasonString = 0x1f,
    ReceiveMaximum = 0x21,
    TopicAliasMaximum = 0x22,
    TopicAlias = 0x23,
    MaximumQos = 0x24,
    RetainAvailable = 0x25,
    UserProperty = 0x26,
    MaximumPacketSize = 0x27,
    WildcardSubscriptionAvailable = 0x28,
    SubscriptionIdentifierAvailable = 0x29,
    SharedSubscriptionAvailable = 0x2a,
}

impl From<PropertyIdentifier> for u8 {
    fn from(p: PropertyIdentifier) -> Self {
        p as u8
    }
}

// impl From<Property> for PropertyIdentifier {
//     fn from(p: Property) -> Self {
//         PropertyIdentifier::try_from(p.property_identifier as u8).unwrap()
//     }
// }

impl TryFrom<u8> for PropertyIdentifier {
    type Error = ();
    fn try_from(item: u8) -> Result<Self, Self::Error> {
        match item {
            _ if item == PropertyIdentifier::PayloadFormatIndicator as u8 => {
                Ok(PropertyIdentifier::PayloadFormatIndicator)
            }
            _ if item == PropertyIdentifier::MessageExpiryInterval as u8 => {
                Ok(PropertyIdentifier::MessageExpiryInterval)
            }
            _ if item == PropertyIdentifier::ContentType as u8 => {
                Ok(PropertyIdentifier::ContentType)
            }
            _ if item == PropertyIdentifier::ResponseTopic as u8 => {
                Ok(PropertyIdentifier::ResponseTopic)
            }
            _ if item == PropertyIdentifier::CorrelationData as u8 => {
                Ok(PropertyIdentifier::CorrelationData)
            }
            _ if item == PropertyIdentifier::SubscriptionIdentifier as u8 => {
                Ok(PropertyIdentifier::SubscriptionIdentifier)
            }
            _ if item == PropertyIdentifier::SessionExpiryInterval as u8 => {
                Ok(PropertyIdentifier::SessionExpiryInterval)
            }
            _ if item == PropertyIdentifier::AssignedClientIdentifier as u8 => {
                Ok(PropertyIdentifier::AssignedClientIdentifier)
            }
            _ if item == PropertyIdentifier::ServerKeepAlive as u8 => {
                Ok(PropertyIdentifier::ServerKeepAlive)
            }
            _ if item == PropertyIdentifier::AuthenticationMethod as u8 => {
                Ok(PropertyIdentifier::AuthenticationMethod)
            }
            _ if item == PropertyIdentifier::AuthenticationData as u8 => {
                Ok(PropertyIdentifier::AuthenticationData)
            }
            _ if item == PropertyIdentifier::RequestProblemInformation as u8 => {
                Ok(PropertyIdentifier::RequestProblemInformation)
            }
            _ if item == PropertyIdentifier::WillDelayInterval as u8 => {
                Ok(PropertyIdentifier::WillDelayInterval)
            }
            _ if item == PropertyIdentifier::RequestResponseInformation as u8 => {
                Ok(PropertyIdentifier::RequestResponseInformation)
            }

            _ if item == PropertyIdentifier::ResponseInformation as u8 => {
                Ok(PropertyIdentifier::ResponseInformation)
            }

            _ if item == PropertyIdentifier::ServerReference as u8 => {
                Ok(PropertyIdentifier::ServerReference)
            }

            _ if item == PropertyIdentifier::ReasonString as u8 => {
                Ok(PropertyIdentifier::ReasonString)
            }

            _ if item == PropertyIdentifier::ReceiveMaximum as u8 => {
                Ok(PropertyIdentifier::ReceiveMaximum)
            }

            _ if item == PropertyIdentifier::TopicAliasMaximum as u8 => {
                Ok(PropertyIdentifier::TopicAliasMaximum)
            }

            _ if item == PropertyIdentifier::TopicAlias as u8 => Ok(PropertyIdentifier::TopicAlias),
            _ if item == PropertyIdentifier::MaximumQos as u8 => Ok(PropertyIdentifier::MaximumQos),
            _ if item == PropertyIdentifier::RetainAvailable as u8 => {
                Ok(PropertyIdentifier::RetainAvailable)
            }
            _ if item == PropertyIdentifier::UserProperty as u8 => {
                Ok(PropertyIdentifier::UserProperty)
            }
            _ if item == PropertyIdentifier::MaximumPacketSize as u8 => {
                Ok(PropertyIdentifier::MaximumPacketSize)
            }
            _ if item == PropertyIdentifier::WildcardSubscriptionAvailable as u8 => {
                Ok(PropertyIdentifier::WildcardSubscriptionAvailable)
            }
            _ if item == PropertyIdentifier::SubscriptionIdentifierAvailable as u8 => {
                Ok(PropertyIdentifier::SubscriptionIdentifierAvailable)
            }
            _ if item == PropertyIdentifier::SharedSubscriptionAvailable as u8 => {
                Ok(PropertyIdentifier::SharedSubscriptionAvailable)
            }
            _ => Err(()),
        }
    }
}

/// Will return a list of invalid properties
///
fn invalid_property_for_packet_type(
    properties: &[Property],
    validated_properties: Vec<PropertyIdentifier>,
    pack_type: PacketTypes,
) -> Vec<Property> {
    let mut valid_property_identifiers: Vec<PropertyIdentifier> = vec![];
    valid_property_identifiers.extend(validated_properties);

    let property_extension = match pack_type {
        PacketTypes::Connect => valid_properties_for_connect_packet(),
        PacketTypes::Connack => valid_properties_for_connack_packet(),
        PacketTypes::Publish => valid_properties_for_publish_packet(),
        PacketTypes::Puback => valid_properties_for_puback_packet(),
        PacketTypes::Pubrec => valid_properties_for_pubrec_packet(),
        PacketTypes::Pubrel => valid_properties_for_pubrel_packet(),
        PacketTypes::Pubcomp => valid_properties_for_pubcomp_packet(),
        PacketTypes::Subscribe => valid_properties_for_subscribe_packet(),
        PacketTypes::Suback => valid_properties_for_suback_packet(),
        PacketTypes::Unsubscribe => valid_properties_for_unsubscribe_packet(),
        PacketTypes::Unsuback => valid_properties_for_unsuback_packet(),
        PacketTypes::Disconnect => valid_properties_for_disconnect_packet(),
        PacketTypes::Auth => valid_properties_for_auth_packet(),
        _ => vec![],
    };

    valid_property_identifiers.extend(property_extension);

    let mut invalid: Vec<Property> = Vec::with_capacity(13);

    println!("valid properties are {:?}", valid_property_identifiers);

    invalid_property(properties, &valid_property_identifiers, &mut invalid);

    invalid
}

pub fn invalid_property_for_non_connect_packet_type(
    properties: &[Property],
    packet_type: PacketTypes,
) -> Vec<Property> {
    invalid_property_for_packet_type(properties, vec![], packet_type)
}

pub fn invalid_property_for_connect_packet_type(
    properties: &[Property],
    will_flag: bool,
) -> Vec<Property> {
    invalid_property_for_packet_type(
        properties,
        if will_flag {
            valid_properties_for_will()
        } else {
            vec![]
        },
        PacketTypes::Connect,
    )
}

pub fn invalid_property(
    property: &[Property],
    valid_property_identifier: &[PropertyIdentifier],
    differences: &mut Vec<Property>,
) {
    println!("properties are {:?}", property);
    println!(
        "valid_property_identifiers are {:?}",
        valid_property_identifier
    );
    for p in property {
        if !valid_property_identifier.contains(&p.property_identifier()) {
            differences.push(p.clone());
        }
    }
}

pub fn valid_properties_for_will() -> Vec<PropertyIdentifier> {
    vec![
        PropertyIdentifier::WillDelayInterval,
        PropertyIdentifier::PayloadFormatIndicator,
        PropertyIdentifier::MessageExpiryInterval,
        PropertyIdentifier::ContentType,
        PropertyIdentifier::ResponseTopic,
        PropertyIdentifier::CorrelationData,
        PropertyIdentifier::UserProperty,
    ]
}

fn valid_properties_for_connect_packet() -> Vec<PropertyIdentifier> {
    vec![
        PropertyIdentifier::SessionExpiryInterval,
        PropertyIdentifier::AuthenticationMethod,
        PropertyIdentifier::AuthenticationData,
        PropertyIdentifier::RequestProblemInformation,
        PropertyIdentifier::RequestResponseInformation,
        PropertyIdentifier::ReceiveMaximum,
        PropertyIdentifier::TopicAliasMaximum,
        PropertyIdentifier::UserProperty,
        PropertyIdentifier::MaximumPacketSize,
    ]
}

fn valid_properties_for_connack_packet() -> Vec<PropertyIdentifier> {
    vec![
        PropertyIdentifier::SessionExpiryInterval,
        PropertyIdentifier::ReceiveMaximum,
        PropertyIdentifier::MaximumQos,
        PropertyIdentifier::RetainAvailable,
        PropertyIdentifier::MaximumPacketSize,
        PropertyIdentifier::AssignedClientIdentifier,
        PropertyIdentifier::TopicAliasMaximum,
        PropertyIdentifier::ReasonString,
        PropertyIdentifier::UserProperty,
        PropertyIdentifier::WildcardSubscriptionAvailable,
        PropertyIdentifier::SubscriptionIdentifierAvailable,
        PropertyIdentifier::SharedSubscriptionAvailable,
        PropertyIdentifier::ServerKeepAlive,
        PropertyIdentifier::ResponseInformation,
        PropertyIdentifier::ServerReference,
        PropertyIdentifier::AuthenticationMethod,
        PropertyIdentifier::AuthenticationData,
    ]
}

fn valid_properties_for_publish_packet() -> Vec<PropertyIdentifier> {
    vec![
        PropertyIdentifier::PayloadFormatIndicator,
        PropertyIdentifier::MessageExpiryInterval,
        PropertyIdentifier::ContentType,
        PropertyIdentifier::ResponseTopic,
        PropertyIdentifier::CorrelationData,
        PropertyIdentifier::SubscriptionIdentifier,
        PropertyIdentifier::TopicAlias,
        PropertyIdentifier::UserProperty,
    ]
}

fn valid_properties_for_puback_packet() -> Vec<PropertyIdentifier> {
    vec![
        PropertyIdentifier::ReasonString,
        PropertyIdentifier::UserProperty,
    ]
}

fn valid_properties_for_pubrec_packet() -> Vec<PropertyIdentifier> {
    vec![
        PropertyIdentifier::ReasonString,
        PropertyIdentifier::UserProperty,
    ]
}

fn valid_properties_for_pubrel_packet() -> Vec<PropertyIdentifier> {
    vec![
        PropertyIdentifier::ReasonString,
        PropertyIdentifier::UserProperty,
    ]
}

fn valid_properties_for_pubcomp_packet() -> Vec<PropertyIdentifier> {
    vec![
        PropertyIdentifier::ReasonString,
        PropertyIdentifier::UserProperty,
    ]
}

fn valid_properties_for_subscribe_packet() -> Vec<PropertyIdentifier> {
    vec![
        PropertyIdentifier::SubscriptionIdentifier,
        PropertyIdentifier::UserProperty,
    ]
}

fn valid_properties_for_suback_packet() -> Vec<PropertyIdentifier> {
    vec![
        PropertyIdentifier::ReasonString,
        PropertyIdentifier::UserProperty,
    ]
}

fn valid_properties_for_unsubscribe_packet() -> Vec<PropertyIdentifier> {
    vec![PropertyIdentifier::UserProperty]
}

fn valid_properties_for_unsuback_packet() -> Vec<PropertyIdentifier> {
    vec![
        PropertyIdentifier::ReasonString,
        PropertyIdentifier::UserProperty,
    ]
}

fn valid_properties_for_disconnect_packet() -> Vec<PropertyIdentifier> {
    vec![
        PropertyIdentifier::SessionExpiryInterval,
        PropertyIdentifier::ServerReference,
        PropertyIdentifier::ReasonString,
        PropertyIdentifier::UserProperty,
    ]
}

fn valid_properties_for_auth_packet() -> Vec<PropertyIdentifier> {
    vec![
        PropertyIdentifier::AuthenticationMethod,
        PropertyIdentifier::AuthenticationData,
        PropertyIdentifier::ReasonString,
        PropertyIdentifier::UserProperty,
    ]
}

fn packet_identifier_present(mqtt_control_packet: PacketTypes, qos: u8) -> bool {
    match mqtt_control_packet {
        PacketTypes::Connect
        | PacketTypes::Connack
        | PacketTypes::Pingreq
        | PacketTypes::Pingresp
        | PacketTypes::Disconnect
        | PacketTypes::Auth => false,
        PacketTypes::Puback
        | PacketTypes::Pubrec
        | PacketTypes::Pubrel
        | PacketTypes::Pubcomp
        | PacketTypes::Subscribe
        | PacketTypes::Suback
        | PacketTypes::Unsubscribe
        | PacketTypes::Unsuback => true,
        PacketTypes::Publish => qos > 0,
    }
}

// Returns all instances of Property with
pub fn non_unique_properties(props: &Vec<Property>) -> HashMap<PropertyIdentifier, Vec<Property>> {
    let mut shared_properties: HashMap<PropertyIdentifier, Vec<Property>> = HashMap::new();
    for p in props {
        if let Some(v) = shared_properties.get_mut(&p.property_identifier()) {
            v.push(p.clone());
        } else {
            shared_properties.insert(p.property_identifier(), vec![p.clone()]);
        }
    }

    shared_properties.retain(|_, v| v.len() > 1);

    shared_properties
}

///
/// Returns whether the property has been successfully added. If not then it already exists,
/// and the property is a duplicate. Ignores the User Property
///  # Arguments
///
/// * `props` - List of properties
/// * `to_add` - Property to add
///
pub fn add_property_old(props: &mut Vec<Property>, to_add: Property) -> bool {
    // if props.iter().any(|p| {
    //     u8::from(*p) == u8::from(to_add)
    //         && p.as_ref() as u8 != PropertyIdentifier::UserProperty as u8
    // }) {
    //     return false;
    // }
    if !props.contains(&to_add) {
        // add to list if not property not already exists or is userproperty property
        props.push(to_add);
    }

    true
}

///
/// Returns true when the property has been successfully added. If not then it already exists and returns false
/// Ignores the User Property
///  # Arguments
///
/// * `props` - List of properties
/// * `to_add` - Property to add
///
pub fn add_property(props: &mut Vec<Property>, to_add: Property) -> bool {
    if to_add.property_identifier() == PropertyIdentifier::UserProperty {
        props.push(to_add);
        return true;
    }

    for p in props {
        if p.property_identifier() == to_add.property_identifier() {
            return false;
        }
    }

    false
}

#[cfg(test)]
mod test {
    use crate::mqttbroker::primitive_types::{
        BinaryData, Byte, Utf8EncodedString, Utf8StringPair, VariableByteInteger,
    };
    use crate::mqttbroker::properties::{
        add_property, invalid_property, Property, PropertyIdentifier,
    };

    #[test]
    fn test_add_property_with_duplicate_property() {
        // let mut properties = vec![
        //     Property {
        //         element_value: PropertyType::Byte {
        //             value: Byte::new(0x01),
        //         },
        //         property_identifier: PropertyIdentifier::SubscriptionIdentifier as u8,
        //     },
        //     Property {
        //         element_value: PropertyType::Byte {
        //             value: Byte::new(0x01),
        //         },
        //         property_identifier: PropertyIdentifier::SubscriptionIdentifier as u8,
        //     },
        // ];

        let mut properties = vec![
            Property::SubscriptionIdentifier {
                value: VariableByteInteger(0x01),
            },
            Property::SubscriptionIdentifier {
                value: VariableByteInteger(0x01),
            },
        ];
        // let prop_to_add = Property {
        //     element_value: PropertyType::Byte {
        //         value: Byte::new(0x01),
        //     },
        //     property_identifier: PropertyIdentifier::SubscriptionIdentifier as u8,
        // };
        let prop_to_add = Property::SubscriptionIdentifier {
            value: VariableByteInteger(0x01),
        };

        assert!(!add_property(&mut properties, prop_to_add))
    }

    #[test]
    fn test_add_property_with_unique_property() {
        // let mut properties = vec![Property {
        //     element_value: PropertyType::Byte {
        //         value: Byte::new(0x01),
        //     },
        //     property_identifier: PropertyIdentifier::SubscriptionIdentifier as u8,
        // }];
        let mut properties = vec![Property::SubscriptionIdentifier {
            value: VariableByteInteger(0x01),
        }];
        // let prop_to_add = Property {
        //     element_value: PropertyType::Byte {
        //         value: Byte::new(0x01),
        //     },
        //     property_identifier: PropertyIdentifier::UserProperty as u8,
        // };
        let prop_to_add = Property::User {
            value: Utf8StringPair("key".to_string(), "value".to_string()),
        };
        assert!(add_property(&mut properties, prop_to_add))
    }

    #[test]
    fn test_add_property_with_duplicate_userproperty() {
        // let mut properties = vec![
        //     Property {
        //         element_value: PropertyType::Byte {
        //             value: Byte::new(0x01),
        //         },
        //         property_identifier: PropertyIdentifier::UserProperty as u8,
        //     },
        //     Property {
        //         element_value: PropertyType::Byte {
        //             value: Byte::new(0x01),
        //         },
        //         property_identifier: PropertyIdentifier::UserProperty as u8,
        //     },
        // ];
        let mut properties = vec![
            Property::User {
                value: Utf8StringPair("key".to_string(), "value".to_string()),
            },
            Property::User {
                value: Utf8StringPair("key".to_string(), "value".to_string()),
            },
        ];
        // let prop_to_add = Property {
        //     element_value: PropertyType::Byte {
        //         value: Byte::new(0x01),
        //     },
        //     property_identifier: PropertyIdentifier::UserProperty as u8,
        // };
        let prop_to_add = Property::User {
            value: Utf8StringPair("key2".to_string(), "value2".to_string()),
        };
        assert!(add_property(&mut properties, prop_to_add))
    }

    #[test]
    fn test_invalid_property_1() {
        let property = vec![
            Property::AuthenticationData {
                value: BinaryData(vec![1, 2, 3, 4]),
            },
            Property::WildcardSubscriptionAvailable { value: Byte(1) },
        ];

        let valid_property_identifier = vec![PropertyIdentifier::WildcardSubscriptionAvailable];

        let expected = vec![Property::AuthenticationData {
            value: BinaryData(vec![1, 2, 3, 4]),
        }];
        let mut result: Vec<Property> = vec![];
        invalid_property(&property, &valid_property_identifier, &mut result);
        assert_eq!(expected, result)
    }

    #[test]
    fn test_invalid_property_2() {
        let props = vec![
            Property::AuthenticationData {
                value: BinaryData(vec![1, 2, 3, 4]),
            },
            Property::WildcardSubscriptionAvailable { value: Byte(1) },
            Property::ServerReference {
                value: Utf8EncodedString(String::from("1234")),
            },
        ];

        let valid_prop_ids = vec![
            PropertyIdentifier::WildcardSubscriptionAvailable,
            PropertyIdentifier::ServerReference,
        ];

        let expected = vec![Property::AuthenticationData {
            value: BinaryData(vec![1, 2, 3, 4]),
        }];
        let mut result: Vec<Property> = vec![];
        invalid_property(&props, &valid_prop_ids, &mut result);
        assert_eq!(expected, result)
    }

    // #[test]
    // fn test_non_unique_properties() {
    //     let mut properties = vec![Property {
    //         element_value: PropertyType::Byte {
    //             value: Byte::new(0x01),
    //         },
    //         property_identifier: PropertyIdentifier::SubscriptionIdentifier as u8,
    //     }];
    // }
}
