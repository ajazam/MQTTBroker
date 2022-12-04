use crate::encode;
use crate::mqttbroker::packets::PacketTypes;
use crate::mqttbroker::primitive_types::{
    BinaryData, Byte, FourByteInteger, TwoByteInteger, Utf8EncodedString, Utf8StringPair,
    VariableByteInteger,
};
use bytes::{BufMut, BytesMut};
use std::collections::HashMap;
use std::convert::TryFrom;
use tracing::debug;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum PropertyType {
    Byte { value: Byte },
    FourByteInteger { value: FourByteInteger },
    UTF8EncodedString { value: Utf8EncodedString },
    BinaryData { value: BinaryData },
    TwoByteInteger { value: TwoByteInteger },
    UTF8StringPair { value: Utf8StringPair },
    VariableByteInteger { value: VariableByteInteger },
}

impl PropertyType {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Property {
    pub element_value: PropertyType,
    pub property_identifier: u8,
}

impl Property {
    pub fn new(ev: PropertyType, pi: u8) -> Self {
        Property {
            element_value: ev,
            property_identifier: pi,
        }
    }
}

impl Property {
    pub fn encode(&self, encoded: &mut Vec<u8>) {
        encoded.put_u8(self.property_identifier as u8);
        match self.element_value {
            PropertyType::Byte { ref value } => {
                encoded.put_u8(*value.as_ref());
            }

            PropertyType::FourByteInteger { ref value } => {
                encoded.put_u32(*value.as_ref());
            }

            PropertyType::UTF8EncodedString { ref value } => {
                let mut encoded_bytes = BytesMut::with_capacity(200);
                encode::encode_utf8_encoded_string(value.as_ref(), &mut encoded_bytes);
                encoded.put(encoded_bytes);
            }

            PropertyType::BinaryData { ref value } => {
                let mut encoded_bytes = BytesMut::with_capacity(200);
                let src = BytesMut::from(value.as_ref().as_slice());
                encode::encode_binary_data(&src, &mut encoded_bytes);
                encoded.put(encoded_bytes);
            }

            PropertyType::TwoByteInteger { ref value } => encoded.put_u16(*value.as_ref()),

            PropertyType::UTF8StringPair { ref value } => {
                let mut encoded_bytes = BytesMut::with_capacity(200);
                encode::utf8_string_pair(&value.0, &value.1, &mut encoded_bytes);
                encoded.put(encoded_bytes);
            }

            PropertyType::VariableByteInteger { ref value } => {
                let mut encoded_bytes = BytesMut::with_capacity(200);
                encode::encode_variable_byte_integer(&value, &mut encoded_bytes);
                encoded.put(encoded_bytes);
            }
        }
    }
}

impl From<Property> for u8 {
    fn from(prop: Property) -> Self {
        prop.property_identifier as u8
    }
}

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

impl From<Property> for PropertyIdentifier {
    fn from(p: Property) -> Self {
        PropertyIdentifier::try_from(p.property_identifier as u8).unwrap()
    }
}

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

    debug!("valid properties are {:?}", valid_property_identifiers);

    diff(properties, &valid_property_identifiers, &mut invalid);

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

pub fn diff(left: &[Property], right: &[PropertyIdentifier], differences: &mut Vec<Property>) {
    for property in left {
        if !right
            .contains(&PropertyIdentifier::try_from(property.property_identifier as u8).unwrap())
        {
            differences.push(property.clone());
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
        PropertyIdentifier::AssignedClientIdentifier,
        PropertyIdentifier::ServerKeepAlive,
        PropertyIdentifier::AuthenticationMethod,
        PropertyIdentifier::AuthenticationData,
        PropertyIdentifier::ResponseInformation,
        PropertyIdentifier::ServerReference,
        PropertyIdentifier::ReasonString,
        PropertyIdentifier::ReceiveMaximum,
        PropertyIdentifier::TopicAliasMaximum,
        PropertyIdentifier::MaximumQos,
        PropertyIdentifier::RetainAvailable,
        PropertyIdentifier::UserProperty,
        PropertyIdentifier::MaximumPacketSize,
        PropertyIdentifier::WildcardSubscriptionAvailable,
        PropertyIdentifier::SubscriptionIdentifierAvailable,
        PropertyIdentifier::SharedSubscriptionAvailable,
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

///
/// Returns non-unique properties, excluding User Property
pub fn non_unique_properties(props: &Vec<Property>) -> Vec<Property> {
    let mut prop_count: HashMap<Property, u8> = HashMap::new();
    for p in props {
        prop_count.insert(p.clone(), 1);
        let count = prop_count.entry(p.clone()).or_insert(0);
        *count += 1;
    }

    prop_count.retain(|k, v| k.property_identifier != PropertyIdentifier::UserProperty as u8);

    prop_count.retain(|_, v| *v > 1);
    prop_count.into_keys().collect()
}

///
/// Returns whether the property has been successfully added. If not then it already exists,
/// and the property is a duplicate. Ignores the User Property
///  # Arguments
///
/// * `props` - List of properties
/// * `to_add` - Property to add
///
pub fn add_property(props: &mut Vec<Property>, mut to_add: Property) -> bool {
    if props.iter().any(|p| {
        p.property_identifier == to_add.property_identifier
            && p.property_identifier != PropertyIdentifier::UserProperty as u8
    }) {
        return false;
    }

    // add to list if not property not already exists or is userproperty property
    props.push(to_add);

    true
}

#[cfg(test)]
mod test {
    use crate::mqttbroker::primitive_types::Byte;
    use crate::mqttbroker::properties::{add_property, Property, PropertyIdentifier, PropertyType};

    #[test]
    fn test_add_property_with_duplicate_property() {
        let mut properties = vec![
            Property {
                element_value: PropertyType::Byte {
                    value: Byte::new(0x01),
                },
                property_identifier: PropertyIdentifier::SubscriptionIdentifier as u8,
            },
            Property {
                element_value: PropertyType::Byte {
                    value: Byte::new(0x01),
                },
                property_identifier: PropertyIdentifier::SubscriptionIdentifier as u8,
            },
        ];
        let prop_to_add = Property {
            element_value: PropertyType::Byte {
                value: Byte::new(0x01),
            },
            property_identifier: PropertyIdentifier::SubscriptionIdentifier as u8,
        };
        assert!(!add_property(&mut properties, prop_to_add))
    }

    #[test]
    fn test_add_property_with_unique_property() {
        let mut properties = vec![Property {
            element_value: PropertyType::Byte {
                value: Byte::new(0x01),
            },
            property_identifier: PropertyIdentifier::SubscriptionIdentifier as u8,
        }];
        let prop_to_add = Property {
            element_value: PropertyType::Byte {
                value: Byte::new(0x01),
            },
            property_identifier: PropertyIdentifier::UserProperty as u8,
        };
        assert!(add_property(&mut properties, prop_to_add))
    }

    #[test]
    fn test_add_property_with_duplicate_userproperty() {
        let mut properties = vec![
            Property {
                element_value: PropertyType::Byte {
                    value: Byte::new(0x01),
                },
                property_identifier: PropertyIdentifier::UserProperty as u8,
            },
            Property {
                element_value: PropertyType::Byte {
                    value: Byte::new(0x01),
                },
                property_identifier: PropertyIdentifier::UserProperty as u8,
            },
        ];
        let prop_to_add = Property {
            element_value: PropertyType::Byte {
                value: Byte::new(0x01),
            },
            property_identifier: PropertyIdentifier::UserProperty as u8,
        };
        assert!(add_property(&mut properties, prop_to_add))
    }

    #[test]
    fn test_non_unique_properties() {
        let mut properties = vec![Property {
            element_value: PropertyType::Byte {
                value: Byte::new(0x01),
            },
            property_identifier: PropertyIdentifier::SubscriptionIdentifier as u8,
        }];
    }
}
