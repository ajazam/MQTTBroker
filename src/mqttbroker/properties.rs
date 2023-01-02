use crate::mqttbroker::packets::PacketTypes;
use crate::mqttbroker::primitive_types::{
    BinaryData, Byte, FourByteInteger, TwoByteInteger, Utf8EncodedString, Utf8StringPair,
    VariableByteInteger,
};
use bytes::BufMut;
use std::collections::HashMap;
use std::ops::Deref;
use tracing::trace;

#[derive(Debug, PartialEq, PartialOrd, Eq, Hash, Copy, Clone)]
#[repr(u8)]
pub enum PropertyIdentifierConstant {
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
    User = 0x26,
    MaximumPacketSize = 0x27,
    WildcardSubscriptionAvailable = 0x28,
    SubscriptionIdentifierAvailable = 0x29,
    SharedSubscriptionAvailable = 0x2a,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Hash)]
pub struct PropertyIdentifier {
    pub value: PropertyIdentifierConstant,
}
impl PropertyIdentifier {
    pub fn new(value: PropertyIdentifierConstant) -> PropertyIdentifier {
        PropertyIdentifier { value }
    }

    pub fn to_u8(&self) -> u8 {
        let v = &self.value.clone();
        *v as u8
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
#[repr(u8)]
pub enum Property {
    PayloadFormatIndicator(Byte) = PropertyIdentifierConstant::PayloadFormatIndicator as u8,
    MessageExpiryInterval(FourByteInteger) =
        PropertyIdentifierConstant::MessageExpiryInterval as u8,
    ContentType(Utf8EncodedString) = PropertyIdentifierConstant::ContentType as u8,
    ResponseTopic(Utf8EncodedString) = PropertyIdentifierConstant::ResponseTopic as u8,
    CorrelationData(BinaryData) = PropertyIdentifierConstant::CorrelationData as u8,
    SubscriptionIdentifier(VariableByteInteger) =
        PropertyIdentifierConstant::SubscriptionIdentifier as u8,
    SessionExpiryInterval(FourByteInteger) =
        PropertyIdentifierConstant::SessionExpiryInterval as u8,
    AssignedClientIdentifier(Utf8EncodedString) =
        PropertyIdentifierConstant::AssignedClientIdentifier as u8,
    ServerKeepAlive(TwoByteInteger) = PropertyIdentifierConstant::ServerKeepAlive as u8,
    AuthenticationMethod(Utf8EncodedString) =
        PropertyIdentifierConstant::AuthenticationMethod as u8,
    AuthenticationData(BinaryData) = PropertyIdentifierConstant::AuthenticationData as u8,
    RequestProblemInformation(Byte) = PropertyIdentifierConstant::RequestProblemInformation as u8,
    WillDelayInterval(FourByteInteger) = PropertyIdentifierConstant::WillDelayInterval as u8,
    RequestResponseInformation(Byte) = PropertyIdentifierConstant::RequestResponseInformation as u8,
    ResponseInformation(Utf8EncodedString) = PropertyIdentifierConstant::ResponseInformation as u8,
    ServerReference(Utf8EncodedString) = PropertyIdentifierConstant::ServerReference as u8,
    ReasonString(Utf8EncodedString) = PropertyIdentifierConstant::ReasonString as u8,
    ReceiveMaximum(TwoByteInteger) = PropertyIdentifierConstant::ReceiveMaximum as u8,
    TopicAliasMaximum(TwoByteInteger) = PropertyIdentifierConstant::TopicAliasMaximum as u8,
    TopicAlias(TwoByteInteger) = PropertyIdentifierConstant::TopicAlias as u8,
    MaximumQos(Byte) = PropertyIdentifierConstant::MaximumQos as u8,
    RetainAvailable(Byte) = PropertyIdentifierConstant::RetainAvailable as u8,
    User(Utf8StringPair) = PropertyIdentifierConstant::User as u8,
    MaximumPacketSize(FourByteInteger) = PropertyIdentifierConstant::MaximumPacketSize as u8,
    WildcardSubscriptionAvailable(Byte) =
        PropertyIdentifierConstant::WildcardSubscriptionAvailable as u8,
    SubscriptionIdentifierAvailable(Byte) =
        PropertyIdentifierConstant::SubscriptionIdentifierAvailable as u8,
    SharedSubscriptionAvailable(Byte) =
        PropertyIdentifierConstant::SharedSubscriptionAvailable as u8,
}

impl From<&Property> for PropertyIdentifier {
    fn from(value: &Property) -> Self {
        match value {
            Property::PayloadFormatIndicator(..) => {
                PropertyIdentifier::new(PropertyIdentifierConstant::PayloadFormatIndicator)
            }

            Property::MessageExpiryInterval(..) => {
                PropertyIdentifier::new(PropertyIdentifierConstant::MessageExpiryInterval)
            }

            Property::ContentType(..) => {
                PropertyIdentifier::new(PropertyIdentifierConstant::ContentType)
            }

            Property::ResponseTopic(..) => {
                PropertyIdentifier::new(PropertyIdentifierConstant::ResponseTopic)
            }

            Property::CorrelationData(..) => {
                PropertyIdentifier::new(PropertyIdentifierConstant::CorrelationData)
            }

            Property::SubscriptionIdentifier(..) => {
                PropertyIdentifier::new(PropertyIdentifierConstant::SubscriptionIdentifier)
            }

            Property::SessionExpiryInterval(..) => {
                PropertyIdentifier::new(PropertyIdentifierConstant::SessionExpiryInterval)
            }

            Property::AssignedClientIdentifier(..) => {
                PropertyIdentifier::new(PropertyIdentifierConstant::AssignedClientIdentifier)
            }

            Property::ServerKeepAlive(..) => {
                PropertyIdentifier::new(PropertyIdentifierConstant::ServerKeepAlive)
            }

            Property::AuthenticationMethod(..) => {
                PropertyIdentifier::new(PropertyIdentifierConstant::AuthenticationMethod)
            }

            Property::AuthenticationData(..) => {
                PropertyIdentifier::new(PropertyIdentifierConstant::AuthenticationData)
            }

            Property::RequestProblemInformation(..) => {
                PropertyIdentifier::new(PropertyIdentifierConstant::RequestProblemInformation)
            }

            Property::WillDelayInterval(..) => {
                PropertyIdentifier::new(PropertyIdentifierConstant::WillDelayInterval)
            }

            Property::RequestResponseInformation(..) => {
                PropertyIdentifier::new(PropertyIdentifierConstant::RequestResponseInformation)
            }

            Property::ResponseInformation(..) => {
                PropertyIdentifier::new(PropertyIdentifierConstant::ResponseInformation)
            }

            Property::ServerReference(..) => {
                PropertyIdentifier::new(PropertyIdentifierConstant::ServerReference)
            }

            Property::ReasonString(..) => {
                PropertyIdentifier::new(PropertyIdentifierConstant::ReasonString)
            }

            Property::ReceiveMaximum(..) => {
                PropertyIdentifier::new(PropertyIdentifierConstant::ReceiveMaximum)
            }

            Property::TopicAliasMaximum(..) => {
                PropertyIdentifier::new(PropertyIdentifierConstant::TopicAliasMaximum)
            }

            Property::TopicAlias(..) => {
                PropertyIdentifier::new(PropertyIdentifierConstant::TopicAlias)
            }

            Property::MaximumQos(..) => {
                PropertyIdentifier::new(PropertyIdentifierConstant::MaximumQos)
            }

            Property::RetainAvailable(..) => {
                PropertyIdentifier::new(PropertyIdentifierConstant::RetainAvailable)
            }

            Property::User(..) => PropertyIdentifier::new(PropertyIdentifierConstant::User),

            Property::MaximumPacketSize(..) => {
                PropertyIdentifier::new(PropertyIdentifierConstant::MaximumPacketSize)
            }

            Property::WildcardSubscriptionAvailable(..) => {
                PropertyIdentifier::new(PropertyIdentifierConstant::WildcardSubscriptionAvailable)
            }

            Property::SubscriptionIdentifierAvailable(..) => {
                PropertyIdentifier::new(PropertyIdentifierConstant::SubscriptionIdentifierAvailable)
            }

            Property::SharedSubscriptionAvailable(..) => {
                PropertyIdentifier::new(PropertyIdentifierConstant::SharedSubscriptionAvailable)
            }
        }
    }
}

impl Property {
    pub fn encode(&self, encoded: &mut Vec<u8>) {
        match self {
            p @ Property::CorrelationData(value) => Self::encode_binary_data(p, value, encoded),

            p @ Property::SubscriptionIdentifier(value) => {
                Self::encode_variable_byte_integer(p, value, encoded);
            }

            p @ (Property::SessionExpiryInterval(value)
            | Property::MaximumPacketSize(value)
            | Property::WillDelayInterval(value)
            | Property::MessageExpiryInterval(value)) => {
                Self::encode_four_byte_integer(p, value, encoded);
            }

            p @ Property::AuthenticationData(value) => {
                Self::encode_binary_data(p, value, encoded);
            }

            p @ (Property::ResponseInformation(value)
            | Property::ServerReference(value)
            | Property::ReasonString(value)
            | Property::ContentType(value)
            | Property::AuthenticationMethod(value)
            | Property::AssignedClientIdentifier(value)
            | Property::ResponseTopic(value)) => {
                Self::encode_utf8_encoded_string(p, value, encoded);
            }

            p @ (Property::ReceiveMaximum(value)
            | Property::TopicAliasMaximum(value)
            | Property::TopicAlias(value)
            | Property::ServerKeepAlive(value)) => {
                Self::encode_two_byte_integer(p, value, encoded);
            }

            p @ Property::User(value) => {
                Self::encode_utf8_string_pair(p, value, encoded);
            }

            p @ (Property::PayloadFormatIndicator(value)
            | Property::RequestProblemInformation(value)
            | Property::RequestResponseInformation(value)
            | Property::MaximumQos(value)
            | Property::RetainAvailable(value)
            | Property::WildcardSubscriptionAvailable(value)
            | Property::SubscriptionIdentifierAvailable(value)
            | Property::SharedSubscriptionAvailable(value)) => Self::encode_byte(p, value, encoded),
        }
    }

    fn encode_byte(property: &Property, value: &Byte, encoded: &mut Vec<u8>) {
        encoded.put_u8(PropertyIdentifier::from(property).to_u8());
        encoded.put_u8(value.0);
    }

    fn encode_two_byte_integer(property: &Property, value: &TwoByteInteger, encoded: &mut Vec<u8>) {
        encoded.put_u8(PropertyIdentifier::from(property).to_u8());
        encoded.put_u16(value.0);
    }

    fn encode_four_byte_integer(
        property: &Property,
        value: &FourByteInteger,
        encoded: &mut Vec<u8>,
    ) {
        encoded.put_u8(PropertyIdentifier::from(property).to_u8());
        encoded.put_u32(value.0);
    }

    fn encode_variable_byte_integer(
        property: &Property,
        value: &VariableByteInteger,
        encoded: &mut Vec<u8>,
    ) {
        encoded.put_u8(PropertyIdentifier::from(property).to_u8());
        encoded.put_u32(value.0);
    }

    fn encode_utf8_encoded_string(
        property: &Property,
        value: &Utf8EncodedString,
        encoded: &mut Vec<u8>,
    ) {
        encoded.put_u8(PropertyIdentifier::from(property).to_u8());
        encoded.put_u16(value.0.len() as u16);
        encoded.put_slice(value.0.as_bytes());
    }

    fn encode_utf8_string_pair(property: &Property, value: &Utf8StringPair, encoded: &mut Vec<u8>) {
        encoded.put_u8(PropertyIdentifier::from(property).to_u8());
        encoded.put_u16(value.0.len() as u16);
        encoded.put_slice(value.0.as_bytes());
        encoded.put_u16(value.1.len() as u16);
        encoded.put_slice(value.1.as_bytes());
    }

    fn encode_binary_data(property: &Property, value: &BinaryData, encoded: &mut Vec<u8>) {
        encoded.put_u8(PropertyIdentifier::from(property).to_u8());
        encoded.put_u16(value.0.len() as u16);
        encoded.put_slice(value.0.as_slice());
    }
}

#[tracing::instrument]
/// Will return a list of invalid properties
///
///
pub fn invalid_property_for_packet_type(
    properties: &[Property],
    pack_type: PacketTypes,
) -> Vec<Property> {
    let mut valid_property_identifiers: Vec<PropertyIdentifier> = vec![];

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

    trace!("valid properties are {valid_property_identifiers:?}");

    invalid_property(properties, &valid_property_identifiers, &mut invalid);

    invalid
}

pub fn invalid_property(
    property: &[Property],
    valid_property_identifier: &[PropertyIdentifier],
    differences: &mut Vec<Property>,
) {
    trace!("properties are {property:?}");
    println!("properties are {:?}", property);
    trace!("valid_property_identifiers are {valid_property_identifier:?}");
    for p in property {
        if !valid_property_identifier
            .contains(&PropertyIdentifier::new(PropertyIdentifier::from(p).value))
        {
            differences.push(p.clone());
        }
    }
}

pub fn valid_properties_for_will() -> Vec<PropertyIdentifier> {
    vec![
        PropertyIdentifier::new(PropertyIdentifierConstant::WillDelayInterval),
        PropertyIdentifier::new(PropertyIdentifierConstant::PayloadFormatIndicator),
        PropertyIdentifier::new(PropertyIdentifierConstant::MessageExpiryInterval),
        PropertyIdentifier::new(PropertyIdentifierConstant::ContentType),
        PropertyIdentifier::new(PropertyIdentifierConstant::ResponseTopic),
        PropertyIdentifier::new(PropertyIdentifierConstant::CorrelationData),
        PropertyIdentifier::new(PropertyIdentifierConstant::User),
    ]
}

fn valid_properties_for_connect_packet() -> Vec<PropertyIdentifier> {
    vec![
        PropertyIdentifier::new(PropertyIdentifierConstant::SessionExpiryInterval),
        PropertyIdentifier::new(PropertyIdentifierConstant::AuthenticationMethod),
        PropertyIdentifier::new(PropertyIdentifierConstant::AuthenticationData),
        PropertyIdentifier::new(PropertyIdentifierConstant::RequestProblemInformation),
        PropertyIdentifier::new(PropertyIdentifierConstant::RequestResponseInformation),
        PropertyIdentifier::new(PropertyIdentifierConstant::ReceiveMaximum),
        PropertyIdentifier::new(PropertyIdentifierConstant::TopicAliasMaximum),
        PropertyIdentifier::new(PropertyIdentifierConstant::User),
        PropertyIdentifier::new(PropertyIdentifierConstant::MaximumPacketSize),
    ]
}

fn valid_properties_for_connack_packet() -> Vec<PropertyIdentifier> {
    vec![
        PropertyIdentifier::new(PropertyIdentifierConstant::SessionExpiryInterval),
        PropertyIdentifier::new(PropertyIdentifierConstant::ReceiveMaximum),
        PropertyIdentifier::new(PropertyIdentifierConstant::MaximumQos),
        PropertyIdentifier::new(PropertyIdentifierConstant::RetainAvailable),
        PropertyIdentifier::new(PropertyIdentifierConstant::MaximumPacketSize),
        PropertyIdentifier::new(PropertyIdentifierConstant::AssignedClientIdentifier),
        PropertyIdentifier::new(PropertyIdentifierConstant::TopicAliasMaximum),
        PropertyIdentifier::new(PropertyIdentifierConstant::ReasonString),
        PropertyIdentifier::new(PropertyIdentifierConstant::User),
        PropertyIdentifier::new(PropertyIdentifierConstant::WildcardSubscriptionAvailable),
        PropertyIdentifier::new(PropertyIdentifierConstant::SubscriptionIdentifierAvailable),
        PropertyIdentifier::new(PropertyIdentifierConstant::SharedSubscriptionAvailable),
        PropertyIdentifier::new(PropertyIdentifierConstant::ServerKeepAlive),
        PropertyIdentifier::new(PropertyIdentifierConstant::ResponseInformation),
        PropertyIdentifier::new(PropertyIdentifierConstant::ServerReference),
        PropertyIdentifier::new(PropertyIdentifierConstant::AuthenticationMethod),
        PropertyIdentifier::new(PropertyIdentifierConstant::AuthenticationData),
    ]
}

fn valid_properties_for_publish_packet() -> Vec<PropertyIdentifier> {
    vec![
        PropertyIdentifier::new(PropertyIdentifierConstant::PayloadFormatIndicator),
        PropertyIdentifier::new(PropertyIdentifierConstant::MessageExpiryInterval),
        PropertyIdentifier::new(PropertyIdentifierConstant::ContentType),
        PropertyIdentifier::new(PropertyIdentifierConstant::ResponseTopic),
        PropertyIdentifier::new(PropertyIdentifierConstant::CorrelationData),
        PropertyIdentifier::new(PropertyIdentifierConstant::SubscriptionIdentifier),
        PropertyIdentifier::new(PropertyIdentifierConstant::TopicAlias),
        PropertyIdentifier::new(PropertyIdentifierConstant::User),
    ]
}

fn valid_properties_for_puback_packet() -> Vec<PropertyIdentifier> {
    vec![
        PropertyIdentifier::new(PropertyIdentifierConstant::ReasonString),
        PropertyIdentifier::new(PropertyIdentifierConstant::User),
    ]
}

fn valid_properties_for_pubrec_packet() -> Vec<PropertyIdentifier> {
    vec![
        PropertyIdentifier::new(PropertyIdentifierConstant::ReasonString),
        PropertyIdentifier::new(PropertyIdentifierConstant::User),
    ]
}

fn valid_properties_for_pubrel_packet() -> Vec<PropertyIdentifier> {
    vec![
        PropertyIdentifier::new(PropertyIdentifierConstant::ReasonString),
        PropertyIdentifier::new(PropertyIdentifierConstant::User),
    ]
}

fn valid_properties_for_pubcomp_packet() -> Vec<PropertyIdentifier> {
    vec![
        PropertyIdentifier::new(PropertyIdentifierConstant::ReasonString),
        PropertyIdentifier::new(PropertyIdentifierConstant::User),
    ]
}

fn valid_properties_for_subscribe_packet() -> Vec<PropertyIdentifier> {
    vec![
        PropertyIdentifier::new(PropertyIdentifierConstant::SubscriptionIdentifier),
        PropertyIdentifier::new(PropertyIdentifierConstant::User),
    ]
}

fn valid_properties_for_suback_packet() -> Vec<PropertyIdentifier> {
    vec![
        PropertyIdentifier::new(PropertyIdentifierConstant::ReasonString),
        PropertyIdentifier::new(PropertyIdentifierConstant::User),
    ]
}

fn valid_properties_for_unsubscribe_packet() -> Vec<PropertyIdentifier> {
    vec![PropertyIdentifier::new(PropertyIdentifierConstant::User)]
}

fn valid_properties_for_unsuback_packet() -> Vec<PropertyIdentifier> {
    vec![
        PropertyIdentifier::new(PropertyIdentifierConstant::ReasonString),
        PropertyIdentifier::new(PropertyIdentifierConstant::User),
    ]
}

fn valid_properties_for_disconnect_packet() -> Vec<PropertyIdentifier> {
    vec![
        PropertyIdentifier::new(PropertyIdentifierConstant::SessionExpiryInterval),
        PropertyIdentifier::new(PropertyIdentifierConstant::ServerReference),
        PropertyIdentifier::new(PropertyIdentifierConstant::ReasonString),
        PropertyIdentifier::new(PropertyIdentifierConstant::User),
    ]
}

fn valid_properties_for_auth_packet() -> Vec<PropertyIdentifier> {
    vec![
        PropertyIdentifier::new(PropertyIdentifierConstant::AuthenticationMethod),
        PropertyIdentifier::new(PropertyIdentifierConstant::AuthenticationData),
        PropertyIdentifier::new(PropertyIdentifierConstant::ReasonString),
        PropertyIdentifier::new(PropertyIdentifierConstant::User),
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
pub fn non_unique(props: &Vec<Property>) -> HashMap<PropertyIdentifier, Vec<Property>> {
    let mut shared_properties: HashMap<PropertyIdentifier, Vec<Property>> = HashMap::new();
    for p in props {
        if let Some(v) =
            shared_properties.get_mut(&PropertyIdentifier::new(PropertyIdentifier::from(p).value))
        {
            v.push(p.clone());
        } else {
            shared_properties.insert(
                PropertyIdentifier::new(PropertyIdentifier::from(p).value),
                vec![p.clone()],
            );
        }
    }

    shared_properties.retain(|_, v| v.len() > 1);

    shared_properties
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
    if PropertyIdentifier::from(&to_add)
        == (PropertyIdentifier::new(PropertyIdentifierConstant::User))
    {
        props.push(to_add);
        return true;
    }

    for p in props {
        if PropertyIdentifier::from(p.deref()).to_u8() == PropertyIdentifier::from(&to_add).to_u8()
        {
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
        add_property, invalid_property, Property, PropertyIdentifier, PropertyIdentifierConstant,
    };

    #[test]
    fn test_add_property_with_duplicate_property() {
        let mut properties = vec![
            Property::SubscriptionIdentifier(VariableByteInteger(0x01)),
            Property::SubscriptionIdentifier(VariableByteInteger(0x01)),
        ];

        let prop_to_add = Property::SubscriptionIdentifier(VariableByteInteger(0x01));

        assert!(!add_property(&mut properties, prop_to_add));
    }

    #[test]
    fn test_add_property_with_unique_property() {
        let mut properties = vec![Property::SubscriptionIdentifier(VariableByteInteger(0x01))];

        let prop_to_add = Property::User(Utf8StringPair("key".to_string(), "value".to_string()));
        assert!(add_property(&mut properties, prop_to_add));
    }

    #[test]
    fn test_add_property_with_duplicate_userproperty() {
        let mut properties = vec![
            Property::User(Utf8StringPair("key".to_string(), "value".to_string())),
            Property::User(Utf8StringPair("key".to_string(), "value".to_string())),
        ];

        let prop_to_add = Property::User(Utf8StringPair("key2".to_string(), "value2".to_string()));

        assert!(add_property(&mut properties, prop_to_add));
    }

    #[test]
    fn test_invalid_property_1() {
        let property = vec![
            Property::AuthenticationData(BinaryData(vec![1, 2, 3, 4])),
            Property::WildcardSubscriptionAvailable(Byte(1)),
        ];

        let valid_property_identifier: Vec<PropertyIdentifier> = vec![PropertyIdentifier::new(
            PropertyIdentifierConstant::WildcardSubscriptionAvailable,
        )];

        let expected = vec![Property::AuthenticationData(BinaryData(vec![1, 2, 3, 4]))];
        let mut result: Vec<Property> = vec![];
        invalid_property(&property, &valid_property_identifier, &mut result);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_invalid_property_2() {
        use crate::mqttbroker::properties::PropertyIdentifier;
        let props = vec![
            Property::AuthenticationData(BinaryData(vec![1, 2, 3, 4])),
            Property::WildcardSubscriptionAvailable(Byte(1)),
            Property::ServerReference(Utf8EncodedString(String::from("1234"))),
        ];

        let valid_prop_ids = vec![
            PropertyIdentifier::new(PropertyIdentifierConstant::WildcardSubscriptionAvailable),
            PropertyIdentifier::new(PropertyIdentifierConstant::ServerReference),
        ];

        let expected = vec![Property::AuthenticationData(BinaryData(vec![1, 2, 3, 4]))];
        let mut result: Vec<Property> = vec![];
        invalid_property(&props, &valid_prop_ids, &mut result);
        assert_eq!(expected, result);
    }
}
