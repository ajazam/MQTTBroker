use crate::decode::DecodeError::UTF8Errors;
use crate::mqttbroker::mqtt_broker::types::{
    BinaryDataT, FourByteIntegerT, TwoByteIntegerT, Utf8stringPairT, VariableByteIntegerT,
};
use crate::mqttbroker::mqtt_broker::Property;
use crate::mqttbroker::mqtt_broker::{packet_types, PropertyElement};
use bytes::{Buf, BytesMut};
use std::collections::HashMap;
use thiserror::Error;
use tracing::debug;

lazy_static! {
    static ref PROPERTYNAME: HashMap<u8, &'static str> = {
        let mut h = HashMap::new();
        h.insert(1, "Payload Format Indicator");
        h.insert(2, "Message Expiry Interval");
        h.insert(3, "Content Type");
        h.insert(8, "Response Topic");
        h.insert(9, "Correlation Data");
        h.insert(11, "Subscription Identifier");
        h.insert(17, "Session Expiry Interval");
        h.insert(18, "Assigned Client Identifier");
        h.insert(19, "Server Keep Alive");
        h.insert(21, "Authentication Method");
        h.insert(22, "Authentication Data");
        h.insert(23, "Request Problem Information");
        h.insert(24, "Will Delay Interval");
        h.insert(25, "Request Response Information");
        h.insert(26, "Response Information");
        h.insert(28, "Server Reference");
        h.insert(31, "Reason String");
        h.insert(33, "Receive Maximum");
        h.insert(34, "Topic Alias Maximum");
        h.insert(35, "Topic Alias");
        h.insert(36, "Maximum QoS");
        h.insert(37, "Retain Available");
        h.insert(38, "User Property");
        h.insert(39, "Maximum Packet Size");
        h.insert(40, "Wildcard Subscription Available");
        h.insert(41, "Subscription Identifier Available");
        h.insert(42, "Shared Subscription Available");
        h
    };
}
#[derive(Eq, PartialEq)]
pub enum PayLoad {
    Required,
    Optional,
    None,
}

lazy_static! {
    static ref PAYLOADREQUIREDSTATUS: HashMap<packet_types, PayLoad> = {
        let mut h: HashMap<packet_types, PayLoad> = HashMap::new();
        h.insert(packet_types::CONNECT, PayLoad::Required);
        h.insert(packet_types::CONNACK, PayLoad::None);
        h.insert(packet_types::PUBLISH, PayLoad::Optional);
        h.insert(packet_types::PUBACK, PayLoad::None);
        h.insert(packet_types::PUBREC, PayLoad::None);
        h.insert(packet_types::PUBREL, PayLoad::None);
        h.insert(packet_types::PUBCOMP, PayLoad::None);
        h.insert(packet_types::SUBSCRIBE, PayLoad::Required);
        h.insert(packet_types::SUBACK, PayLoad::Required);
        h.insert(packet_types::UNSUBSCRIBE, PayLoad::Required);
        h.insert(packet_types::UNSUBACK, PayLoad::Required);
        h.insert(packet_types::PINGREQ, PayLoad::None);
        h.insert(packet_types::PINGRESP, PayLoad::None);
        h.insert(packet_types::DISCONNECT, PayLoad::None);
        h.insert(packet_types::AUTH, PayLoad::None);
        h
    };
}

// lazy_static! {
//     static ref valid_will_property_codes:[u8;6]  = {
//         let valid_properties = [PropertyIdentifiers::WillDelayInterval, PropertyIdentifiers::PayloadFormatIndicator, PropertyIdentifiers::MessageExpiryInterval, PropertyIdentifiers::ContentType, PropertyIdentifiers::ResponseTopic, PropertyIdentifiers::CorrelationData]
//     }
// }

static valid_will_property_codes: [PropertyIdentifiers; 6] = [
    PropertyIdentifiers::WillDelayInterval,
    PropertyIdentifiers::PayloadFormatIndicator,
    PropertyIdentifiers::MessageExpiryInterval,
    PropertyIdentifiers::ContentType,
    PropertyIdentifiers::ResponseTopic,
    PropertyIdentifiers::CorrelationData,
];

use crate::mqttbroker::mqtt_broker::PropertyIdentifiers;
lazy_static! {
    static ref VALIDPROPERTYCODES: HashMap<packet_types, Vec<PropertyIdentifiers>> = {
        let mut vpc: HashMap<packet_types, Vec<PropertyIdentifiers>> = HashMap::new();

        vpc.insert(
            packet_types::CONNECT,
            vec![
                PropertyIdentifiers::SessionExpiryInterval,
                PropertyIdentifiers::AuthenticationMethod,
                PropertyIdentifiers::AuthenticationData,
                PropertyIdentifiers::RequestProblemInformation,
                PropertyIdentifiers::RequestResponseInformation,
                PropertyIdentifiers::ReceiveMaximum,
                PropertyIdentifiers::TopicAliasMaximum,
                PropertyIdentifiers::UserProperty,
                PropertyIdentifiers::MaximumPacketSize,
            ],
        );

        vpc.insert(
            packet_types::CONNACK,
            vec![
                PropertyIdentifiers::SessionExpiryInterval,
                PropertyIdentifiers::AssignedClientIdentifier,
                PropertyIdentifiers::ServerKeepAlive,
                PropertyIdentifiers::AuthenticationMethod,
                PropertyIdentifiers::AuthenticationData,
                PropertyIdentifiers::ResponseInformation,
                PropertyIdentifiers::ServerReference,
                PropertyIdentifiers::ReasonString,
                PropertyIdentifiers::ReceiveMaximum,
                PropertyIdentifiers::TopicAliasMaximum,
                PropertyIdentifiers::MaximumQos,
                PropertyIdentifiers::RetainAvailable,
                PropertyIdentifiers::UserProperty,
                PropertyIdentifiers::MaximumPacketSize,
                PropertyIdentifiers::WildcardSubscriptionAvailable,
                PropertyIdentifiers::SubscriptionIdentifierAvailable,
                PropertyIdentifiers::SharedSubscriptionAvailable,
            ],
        );

        vpc.insert(
            packet_types::PUBLISH,
            vec![
                PropertyIdentifiers::PayloadFormatIndicator,
                PropertyIdentifiers::MessageExpiryInterval,
                PropertyIdentifiers::ContentType,
                PropertyIdentifiers::ResponseTopic,
                PropertyIdentifiers::CorrelationData,
                PropertyIdentifiers::SubscriptionIdentifier,
                PropertyIdentifiers::TopicAlias,
                PropertyIdentifiers::UserProperty,
            ],
        );

        vpc.insert(
            packet_types::PUBACK,
            vec![
                PropertyIdentifiers::ReasonString,
                PropertyIdentifiers::UserProperty,
            ],
        );

        vpc.insert(
            packet_types::PUBREC,
            vec![
                PropertyIdentifiers::ReasonString,
                PropertyIdentifiers::UserProperty,
            ],
        );

        vpc.insert(
            packet_types::PUBREL,
            vec![
                PropertyIdentifiers::ReasonString,
                PropertyIdentifiers::UserProperty,
            ],
        );

        vpc.insert(
            packet_types::PUBCOMP,
            vec![
                PropertyIdentifiers::ReasonString,
                PropertyIdentifiers::UserProperty,
            ],
        );

        vpc.insert(
            packet_types::SUBSCRIBE,
            vec![
                PropertyIdentifiers::SubscriptionIdentifier,
                PropertyIdentifiers::UserProperty,
            ],
        );

        vpc.insert(
            packet_types::SUBACK,
            vec![
                PropertyIdentifiers::ReasonString,
                PropertyIdentifiers::UserProperty,
            ],
        );

        vpc.insert(
            packet_types::UNSUBSCRIBE,
            vec![PropertyIdentifiers::UserProperty],
        );

        vpc.insert(
            packet_types::UNSUBACK,
            vec![
                PropertyIdentifiers::ReasonString,
                PropertyIdentifiers::UserProperty,
            ],
        );

        vpc.insert(
            packet_types::DISCONNECT,
            vec![
                PropertyIdentifiers::SessionExpiryInterval,
                PropertyIdentifiers::ServerReference,
                PropertyIdentifiers::ReasonString,
                PropertyIdentifiers::UserProperty,
            ],
        );

        vpc.insert(
            packet_types::AUTH,
            vec![
                PropertyIdentifiers::AuthenticationMethod,
                PropertyIdentifiers::AuthenticationData,
                PropertyIdentifiers::ReasonString,
                PropertyIdentifiers::UserProperty,
            ],
        );

        vpc
    };
}
use crate::mqttbroker::mqtt_broker::reason_codes;
lazy_static! {
    static ref VALIDREASONCODES: HashMap<packet_types, Vec<u8>> = {
        let mut vrc: HashMap<packet_types, Vec<u8>> = HashMap::new();
        vrc.insert(
            packet_types::CONNACK,
            vec![
                reason_codes::SUCCESS,
                reason_codes::UNSPECIFIED_ERROR,
                reason_codes::MALFORMED_PACKET,
                reason_codes::PROTOCOL_ERROR,
                reason_codes::IMPLEMENTATION_SPECIFIC_ERROR,
                reason_codes::UNSUPPORTED_PROTOCOL_VERSION,
                reason_codes::CLIENT_IDENTIFIER_NOT_VALID,
                reason_codes::BAD_USER_NAME_OR_PASSWORD,
                reason_codes::NOT_AUTHORIZED,
                reason_codes::SERVER_UNAVAILABLE,
                reason_codes::SERVER_BUSY,
                reason_codes::BANNED,
                reason_codes::BAD_AUTHENTICATION_METHOD,
                reason_codes::TOPIC_NAME_INVALID,
                reason_codes::PACKET_TOO_LARGE,
                reason_codes::QUOTA_EXCEEDED,
                reason_codes::PAYLOAD_FORMAT_INVALID,
                reason_codes::RETAIN_NOT_SUPPORTED,
                reason_codes::QOS_NOT_SUPPORTED,
                reason_codes::USE_ANOTHER_SERVER,
                reason_codes::SERVER_MOVED,
                reason_codes::CONNECTION_RATE_EXCEEDED,
            ],
        );

        vrc.insert(
            packet_types::PUBACK,
            vec![
                reason_codes::SUCCESS,
                reason_codes::NO_MATCHING_SUBSCRIBERS,
                reason_codes::UNSPECIFIED_ERROR,
                reason_codes::IMPLEMENTATION_SPECIFIC_ERROR,
                reason_codes::NOT_AUTHORIZED,
                reason_codes::TOPIC_NAME_INVALID,
                reason_codes::PACKET_IDENTIFIER_IN_USE,
                reason_codes::QUOTA_EXCEEDED,
                reason_codes::PAYLOAD_FORMAT_INVALID,
            ],
        );

        vrc.insert(
            packet_types::PUBREL,
            vec![
                reason_codes::SUCCESS,
                reason_codes::PACKET_IDENTIFIER_NOT_FOUND,
            ],
        );

        vrc.insert(
            packet_types::PUBCOMP,
            vec![
                reason_codes::SUCCESS,
                reason_codes::PACKET_IDENTIFIER_NOT_FOUND,
            ],
        );

        vrc.insert(
            packet_types::SUBACK,
            vec![
                reason_codes::GRANTED_QOS_0,
                reason_codes::GRANTED_QOS_1,
                reason_codes::GRANTED_QOS_2,
                reason_codes::UNSPECIFIED_ERROR,
                reason_codes::IMPLEMENTATION_SPECIFIC_ERROR,
                reason_codes::NOT_AUTHORIZED,
                reason_codes::TOPIC_FILTER_INVALID,
                reason_codes::PACKET_IDENTIFIER_IN_USE,
                reason_codes::QUOTA_EXCEEDED,
                reason_codes::SHARED_SUBSCRIPTIONS_NOT_SUPPORTED,
                reason_codes::SUBSCRIPTION_IDENTIFIERS_NOT_SUPPORTED,
                reason_codes::WILDCARD_SUBSCRIPTIONS_NOT_SUPPORTED,
            ],
        );

        vrc.insert(
            packet_types::UNSUBACK,
            vec![
                reason_codes::SUCCESS,
                reason_codes::NO_SUBSCRIPTION_EXISTED,
                reason_codes::UNSPECIFIED_ERROR,
                reason_codes::IMPLEMENTATION_SPECIFIC_ERROR,
                reason_codes::NOT_AUTHORIZED,
                reason_codes::TOPIC_FILTER_INVALID,
                reason_codes::PACKET_IDENTIFIER_IN_USE,
            ],
        );

        vrc.insert(
            packet_types::DISCONNECT,
            vec![
                reason_codes::NORMAL_DISCONNECTION,
                reason_codes::DISCONNECT_WITH_WILL_MESSAGE,
                reason_codes::UNSPECIFIED_ERROR,
                reason_codes::MALFORMED_PACKET,
                reason_codes::PROTOCOL_ERROR,
                reason_codes::IMPLEMENTATION_SPECIFIC_ERROR,
                reason_codes::NOT_AUTHORIZED,
                reason_codes::SERVER_BUSY,
                reason_codes::SERVER_SHUTTING_DOWN,
                reason_codes::BAD_AUTHENTICATION_METHOD,
                reason_codes::KEEP_ALIVE_TIMEOUT,
                reason_codes::SESSION_TAKEN_OVER,
                reason_codes::TOPIC_FILTER_INVALID,
                reason_codes::TOPIC_NAME_INVALID,
                reason_codes::RECEIVE_MAXIMUM_EXCEEDED,
                reason_codes::TOPIC_ALIAS_INVALID,
                reason_codes::PACKET_TOO_LARGE,
                reason_codes::MESSAGE_RATE_TOO_HIGH,
                reason_codes::QUOTA_EXCEEDED,
                reason_codes::ADMINISTRATIVE_ACTION,
                reason_codes::PAYLOAD_FORMAT_INVALID,
                reason_codes::RETAIN_NOT_SUPPORTED,
                reason_codes::QOS_NOT_SUPPORTED,
                reason_codes::USE_ANOTHER_SERVER,
                reason_codes::SERVER_MOVED,
                reason_codes::SHARED_SUBSCRIPTIONS_NOT_SUPPORTED,
                reason_codes::CONNECTION_RATE_EXCEEDED,
                reason_codes::MAXIMUM_CONNECT_TIME,
                reason_codes::SUBSCRIPTION_IDENTIFIERS_NOT_SUPPORTED,
                reason_codes::WILDCARD_SUBSCRIPTIONS_NOT_SUPPORTED,
            ],
        );

        vrc.insert(
            packet_types::AUTH,
            vec![
                reason_codes::SUCCESS,
                reason_codes::CONTINUE_AUTHENTICATION,
                reason_codes::RE_AUTHENTICATE,
            ],
        );

        vrc
    };
}

#[derive(Error, Debug, PartialEq)]
pub enum DecodeError {
    #[error("Not enough bytes decoding {0}")]
    NotEnoughBytes(String),
    #[error("The variable int does not have the MSB clear on the fourth byte.")]
    NotValidVarInt,
    #[error("Not enough bytes found for decoding {2}. Require {0} bytes, found {1} bytes.")]
    MoreBytesRequired(u16, u16, String),
    #[error("Converting bytes to utf-8 string for {0}")]
    UTF8Errors(String),
}

pub fn two_byte_integer(
    name: String,
    b: &mut BytesMut,
) -> anyhow::Result<TwoByteIntegerT, DecodeError> {
    if b.len() < 2 {
        Err(DecodeError::MoreBytesRequired(2, b.len() as u16, name))
    } else {
        Ok(b.get_u16())
    }
}

pub fn four_byte_integer(
    name: String,
    b: &mut BytesMut,
) -> anyhow::Result<FourByteIntegerT, DecodeError> {
    if b.len() < 4 {
        Err(DecodeError::MoreBytesRequired(4, b.len() as u16, name))
    } else {
        Ok(b.get_u32())
    }
}

pub fn utf8_string(name: String, b: &mut BytesMut) -> anyhow::Result<String, DecodeError> {
    if b.len() < 2 {
        Err(DecodeError::NotEnoughBytes(name))
    } else {
        let mut i = b.iter();
        let string_length = *(i.next().unwrap()) as u16 * 256 + *(i.next().unwrap()) as u16;
        debug!("String {} length is {} ****", name, string_length);
        if (b.len() as u16) < (string_length + 2) {
            Err(DecodeError::MoreBytesRequired(
                string_length,
                b.len() as u16 - 2,
                name,
            ))
        } else {
            b.advance(2);
            let s = b.split_to(string_length as usize);
            match String::from_utf8(s.chunk().to_vec()) {
                Ok(s) => Ok(s),
                Err(_) => Err(UTF8Errors(name)),
            }
        }
    }
}

pub fn binary(name: String, b: &mut BytesMut) -> anyhow::Result<BinaryDataT, DecodeError> {
    if b.len() < 2 {
        Err(DecodeError::NotEnoughBytes(name))
    } else {
        let mut i = b.iter();
        let string_length: u16 =
            (*(i.next().unwrap()) as u16 * 256 + *(i.next().unwrap()) as u16) as u16;
        if (b.len() as u16) < (string_length + 2) {
            Err(DecodeError::MoreBytesRequired(
                string_length,
                b.len() as u16 - 2,
                name,
            ))
        } else {
            b.advance(2);
            let binary = b.split_to(string_length as usize);

            Ok(binary.to_vec())
        }
    }
}

fn utf8_string_pair(
    name: String,
    b: &mut BytesMut,
) -> anyhow::Result<Utf8stringPairT, DecodeError> {
    let mut key: String = String::from("empty");
    let mut value: String = String::from("empty");
    let name_of_key = format!("key of {0}", name);
    let name_of_value = format!("value of {0}", name);
    match utf8_string(name_of_key, b) {
        Ok(k) => {
            key = k;
        }
        Err(DecodeError::NotEnoughBytes(name_of_key)) => {
            return Err(DecodeError::NotEnoughBytes(name_of_key))
        }
        Err(DecodeError::MoreBytesRequired(required, found, name_of_key)) => {
            return Err(DecodeError::MoreBytesRequired(required, found, name_of_key))
        }
        Err(DecodeError::UTF8Errors(name)) => return Err(DecodeError::UTF8Errors(name)),
        _ => {}
    }
    match utf8_string(name_of_value, b) {
        Ok(v) => {
            value = v;
        }
        Err(DecodeError::NotEnoughBytes(name_of_value)) => {
            return Err(DecodeError::NotEnoughBytes(name_of_value))
        }
        Err(DecodeError::MoreBytesRequired(required, found, name_of_value)) => {
            return Err(DecodeError::MoreBytesRequired(
                required,
                found,
                name_of_value,
            ))
        }
        Err(DecodeError::UTF8Errors(name)) => return Err(DecodeError::UTF8Errors(name)),
        _ => {}
    }

    Ok(Utf8stringPairT { key, value })
}

pub fn varint(b: &mut bytes::BytesMut) -> anyhow::Result<VariableByteIntegerT, DecodeError> {
    let mut pos: usize = 0;
    let mut multiplier = 1u32;
    let mut value = 0u32;
    let mut encoded_byte: u8;
    let mut bytes = b.iter();

    loop {
        encoded_byte = *bytes.next().unwrap();
        value += (encoded_byte & 127) as u32 * multiplier;
        multiplier *= 128;

        if encoded_byte & 128 == 0 {
            break;
        }

        if pos == 3 {
            return Err(DecodeError::NotValidVarInt);
        }
        pos += 1;
    }
    b.advance(pos + 1);
    Ok(value)
}

pub fn property(b: &mut bytes::BytesMut) -> anyhow::Result<Vec<Property>, DecodeError> {
    debug!("pre varint length is {}", b.len());
    let length = varint(b)?;
    debug!("post varint length is {}", b.len());
    let mut sub_b = b.split_to((length) as usize);
    debug!("post sub_b is {}", sub_b.len());

    let mut p_vec: Vec<Property> = vec![];
    debug!("property length {}", length);

    while !sub_b.is_empty() {
        let property_identifier = sub_b.get_u8();
        let p = match property_identifier {
            1 | 23 | 25 | 36 | 37 | 40 | 41 | 42 => Property {
                element_value: PropertyElement::Byte {
                    value: sub_b.get_u8(),
                },
                property_identifier,
            },
            2 | 17 | 24 | 39 => {
                let four_byte_integer = four_byte_integer(
                    String::from(*PROPERTYNAME.get(&property_identifier).to_owned().unwrap()),
                    &mut sub_b,
                )?;
                Property {
                    element_value: PropertyElement::FourByteInteger {
                        value: four_byte_integer,
                    },
                    property_identifier,
                }
            }
            3 | 8 | 18 | 21 | 26 | 28 | 31 => {
                let str = utf8_string(
                    String::from(*PROPERTYNAME.get(&property_identifier).to_owned().unwrap()),
                    &mut sub_b,
                )?;

                Property {
                    element_value: PropertyElement::UTF8EncodedString { value: str },
                    property_identifier,
                }
            }
            9 | 22 => {
                let binary_data = binary(
                    String::from(*PROPERTYNAME.get(&property_identifier).to_owned().unwrap()),
                    &mut sub_b,
                )?;

                Property {
                    element_value: PropertyElement::BinaryData { value: binary_data },
                    property_identifier,
                }
            }

            19 | 33 | 34 | 35 => {
                let two_byte_integer = two_byte_integer(
                    String::from(*PROPERTYNAME.get(&property_identifier).to_owned().unwrap()),
                    &mut sub_b,
                )?;

                Property {
                    element_value: PropertyElement::TwoByteInteger {
                        value: two_byte_integer,
                    },
                    property_identifier,
                }
            }

            38 => {
                let utf8_string_pair = utf8_string_pair(
                    String::from(*PROPERTYNAME.get(&property_identifier).to_owned().unwrap()),
                    &mut sub_b,
                )?;
                Property {
                    element_value: PropertyElement::UTF8StringPair {
                        value: utf8_string_pair,
                    },
                    property_identifier,
                }
            }

            11 => {
                debug!("pre variable_byte_integer sub_p len is {}", sub_b.len());
                let variable_byte_integer = varint(&mut sub_b)?;
                debug!("post variable_byte_integer sub_p len is {}", sub_b.len());
                Property {
                    element_value: PropertyElement::VariableByteInteger {
                        value: variable_byte_integer,
                    },
                    property_identifier,
                }
            }

            _ => {
                //FIXME should return a malformed packet error
                panic!() // should return a malformed packet Error
            }
        };
        p_vec.push(p);
    }
    Ok(p_vec)
}

#[cfg(test)]
mod test {
    use crate::mqttbroker::mqtt_broker::packet_types::{
        AUTH, CONNACK, CONNECT, DISCONNECT, PINGREQ, PUBACK, PUBCOMP, PUBLISH, PUBREC, PUBREL,
        SUBACK, SUBSCRIBE, UNSUBACK, UNSUBSCRIBE,
    };
    use crate::mqttbroker::mqtt_broker::types::{FourByteIntegerT, Utf8stringPairT};
    use crate::mqttbroker::mqtt_broker::utility::{
        check_for_non_unique_properties, invalid_property_for_packet_type,
        valid_properties_for_will,
    };
    use crate::mqttbroker::mqtt_broker::PropertyIdentifiers::{
        AssignedClientIdentifier, CorrelationData, ResponseTopic, SessionExpiryInterval,
        SubscriptionIdentifier,
    };

    use crate::decode::{utf8_string_pair, varint, DecodeError};
    use crate::encode::{binary_data, utf8_encoded_string, variable_byte_integer, EncodeError};
    use crate::mqttbroker::mqtt_broker::{Property, PropertyElement, PropertyIdentifiers};
    use crate::{decode, encode};
    use bytes::BufMut;
    use bytes::BytesMut;
    use test_log::test;
    use tracing::{debug, event, info, span, trace, Level};
    use tracing_subscriber::fmt;

    fn payload_format_indicator(val: u8, buf: &mut BytesMut) {
        buf.put_u8(PropertyIdentifiers::PayloadFormatIndicator as u8);
        buf.put_u8(val);
    }

    fn message_expiry_interval(value: u32, buf: &mut BytesMut) {
        buf.put_u8(PropertyIdentifiers::MessageExpiryInterval as u8);
        buf.put_u32(value);
    }

    fn content_type(value: &str, buf: &mut BytesMut) {
        buf.put_u8(PropertyIdentifiers::ContentType as u8);
        utf8_encoded_string(value, buf);
    }

    fn correlation_data(value: &BytesMut, buf: &mut BytesMut) {
        buf.put_u8(PropertyIdentifiers::CorrelationData as u8);
        binary_data(value, buf);
    }

    fn subscription_identifier(value: u32, buf: &mut BytesMut) -> Result<(), EncodeError> {
        buf.put_u8(PropertyIdentifiers::SubscriptionIdentifier as u8);
        variable_byte_integer(value, buf)
    }

    fn session_expiry_interval(value: u32, buf: &mut BytesMut) {
        buf.put_u8(PropertyIdentifiers::SessionExpiryInterval as u8);
        buf.put_u32(value);
    }

    fn assigned_client_identifier(value: &str, buf: &mut BytesMut) {
        buf.put_u8(PropertyIdentifiers::AssignedClientIdentifier as u8);
        utf8_encoded_string(value, buf);
    }

    fn user_property(key: &str, value: &str, buf: &mut BytesMut) {
        buf.put_u8(PropertyIdentifiers::UserProperty as u8);
        encode::utf8_string_pair(key, value, buf);
    }

    #[test]
    fn test_2_byte_integer() {
        let b = &mut BytesMut::with_capacity(2);
        b.put_u16(257);
        assert_eq!(Ok(257), decode::two_byte_integer(String::from("name"), b));
    }

    #[test]
    fn test_2_byte_integer_not_enough_bytes() {
        let b = &mut BytesMut::with_capacity(2);
        b.put_u8(2);
        let name = String::from("name");
        assert_eq!(
            Err(DecodeError::MoreBytesRequired(
                2,
                b.len() as u16,
                name.clone()
            )),
            decode::two_byte_integer(name, b)
        );
    }

    #[test]
    fn test_4_byte_integer() {
        let b = &mut BytesMut::with_capacity(4);
        b.put_u32(257);
        assert_eq!(Ok(257), decode::four_byte_integer(String::from("name"), b));
    }

    #[test]
    fn test_4_byte_integer_not_enough_bytes() {
        let b = &mut BytesMut::with_capacity(2);
        b.put_u16(257);
        let name = String::from("name");
        assert_eq!(
            Err(DecodeError::MoreBytesRequired(
                4,
                b.len() as u16,
                name.clone()
            )),
            decode::four_byte_integer(name, b)
        );
    }

    #[test]
    fn test_utf8_string() {
        let str = b"hello world";
        let b = &mut BytesMut::with_capacity(str.len() + 2);
        b.put_u16(str.len() as u16);
        b.put_slice(str);
        let name = String::from("name");
        assert_eq!("hello world", decode::utf8_string(name, b).unwrap());
    }

    #[test]
    fn test_utf8_string_with_extra_byte() {
        let str = b"hello world";
        let b = &mut BytesMut::with_capacity(str.len() + 2);
        b.put_u16(str.len() as u16);
        b.put_slice(str);
        b.put_u8(0);
        b.put_u8(1);
        b.put_u8(2);
        let name = String::from("name");
        //assert_eq!("hello world", decode::utf8_string(name, b).unwrap());
        let decoded = decode::utf8_string(name, b).unwrap();
        assert_eq!(3, b.len());
        let vec = b.to_vec();
        assert_eq!(vec![0, 1, 2], vec);
    }

    #[test]
    fn test_utf8_string_not_enough_bytes() {
        let b = &mut BytesMut::with_capacity(1);
        b.put_u8(1);
        let name = String::from("name");
        assert_eq!(
            Err(decode::DecodeError::NotEnoughBytes(name.clone())),
            decode::utf8_string(name, b)
        );
    }

    #[test]
    fn test_utf8_string_invalid_utf8() {
        let b = &mut BytesMut::with_capacity(3);
        let str = vec![0, 159];
        let name = String::from("name");
        b.put_u16(str.len() as u16);
        b.put_slice(str.as_slice());
        assert_eq!(
            Err(decode::DecodeError::UTF8Errors(name.clone())),
            decode::utf8_string(name, b)
        )
    }

    #[test]
    fn test_binary() {
        let b = &mut BytesMut::with_capacity(8);
        let name = String::from("name");
        let binary = vec![0, 1, 2, 3, 4, 5];
        b.put_u16(binary.len() as u16);
        b.put_slice(binary.as_slice());
        assert_eq!(binary, decode::binary(name, b).unwrap());
    }

    #[test]
    fn test_binary_with_extra_byte() {
        let b = &mut BytesMut::with_capacity(8);
        let name = String::from("name");
        let binary = vec![0, 1, 2, 3, 4, 5];
        b.put_u16(binary.len() as u16);
        b.put_slice(binary.as_slice());
        b.put_u8(0); // dummy byte value
        assert_eq!(binary, decode::binary(name, b).unwrap());
        assert_eq!(b.len(), 1);
    }

    #[test]
    fn test_binary_not_enough_bytes() {
        let b = &mut BytesMut::with_capacity(1);
        b.put_u8(1);
        let name = String::from("name");
        assert_eq!(
            Err(DecodeError::NotEnoughBytes(name.clone())),
            decode::binary(name, b)
        );
    }
    #[test]
    fn test_utf8_string_pair() {
        let key = b"key";
        let b_key = &mut BytesMut::with_capacity(key.len() + 2);
        b_key.put_u16(key.len() as u16);
        b_key.put(key.to_vec().as_slice());

        let value = b"value";
        let b_value = &mut BytesMut::with_capacity(value.len() + 2);
        b_value.put_u16(value.len() as u16);
        b_value.put(value.to_vec().as_slice());

        let b = &mut BytesMut::with_capacity(100);
        b.put(b_key);
        b.put(b_value);

        let string_pair = Utf8stringPairT {
            key: String::from("key"),
            value: String::from("value"),
        };

        assert_eq!(
            string_pair,
            utf8_string_pair(String::from("property name"), b).unwrap()
        );
    }

    #[test]
    fn varint_1_test() {
        let mut b = BytesMut::with_capacity(1);
        b.put_u8(1);

        if let Ok(i) = varint(&mut b) {
            assert_eq!(i, 1);
        }
    }

    #[test]
    fn varint_127_test() {
        let mut b = BytesMut::with_capacity(1);
        b.put_u8(127);

        if let Ok(i) = varint(&mut b) {
            assert_eq!(i, 127);
        }
    }

    #[test]
    fn varint_127_test_with_extra_byte() {
        let mut b = BytesMut::with_capacity(1);
        b.put_u8(127);
        b.put_u8(0);

        if let Ok(i) = varint(&mut b) {
            assert_eq!(i, 127);
            assert_eq!(1, b.len());
        }
    }
    #[test]
    fn varint_128_test() {
        let mut b = BytesMut::with_capacity(2);
        b.put_u8(0x80);
        b.put_u8(0x01);

        if let Ok(i) = varint(&mut b) {
            assert_eq!(i, 128);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn varint_128_test_with_extra_byte() {
        let mut b = BytesMut::with_capacity(2);
        b.put_u8(0x80);
        b.put_u8(0x01);
        b.put_u8(0);
        if let Ok(i) = varint(&mut b) {
            assert_eq!(i, 128);
            assert_eq!(1, b.len());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn varint_32767_test() {
        let mut b = BytesMut::with_capacity(3);
        b.put_u8(0xFF);
        b.put_u8(0xFF);
        b.put_u8(0x01);
        b.put_u8(0);
        b.put_u8(0);
        if let Ok(i) = varint(&mut b) {
            assert_eq!(i, 32767);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_decode_varint_32767_with_1_dummy_byte_appended() {
        let mut b = BytesMut::with_capacity(1);
        b.put_u8(0xFF);
        b.put_u8(0xFF);
        b.put_u8(0x01);
        b.put_u8(0);
        if let Ok(_) = varint(&mut b) {
            assert_eq!(b.len(), 1);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_decode_varint_32767_with_3_dummy_byte_appended() {
        let mut b = BytesMut::with_capacity(1);
        b.put_u8(0xFF);
        b.put_u8(0xFF);
        b.put_u8(0x01);
        b.put_u8(0);
        b.put_u8(0);
        b.put_u8(0);
        if let Ok(_) = varint(&mut b) {
            assert_eq!(b.len(), 3);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn varint_32768_test() {
        let mut b = BytesMut::with_capacity(3);
        b.put_u8(0x80);
        b.put_u8(0x80);
        b.put_u8(0x02);
        if let Ok(i) = varint(&mut b) {
            assert_eq!(i, 32768);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn varint_268_435_455_test() {
        let mut b = BytesMut::with_capacity(10);
        b.put_u8(0xFF);
        b.put_u8(0xFF);
        b.put_u8(0xFF);
        b.put_u8(0x7f);

        if let Ok(i) = varint(&mut b) {
            assert_eq!(i, 268_435_455);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_property_payload_format_indicator_with_will_message_is_unspecified_bytes_is_correct_size(
    ) {
        let mut b_prop = BytesMut::with_capacity(2);
        let mut b = BytesMut::with_capacity(100);

        b_prop.put_u8(PropertyIdentifiers::PayloadFormatIndicator as u8); // property identifier
        b_prop.put_u8(0x0); //
        if let Ok(..) = encode::variable_byte_integer(b_prop.len() as u32, &mut b) {
            b.put(b_prop);

            assert_eq!(2, *b.get(0).unwrap());
        }
    }

    #[test]
    fn test_property_payload_with_zero_property_length() {
        let b_prop = BytesMut::with_capacity(2);
        let mut b = BytesMut::with_capacity(100);

        variable_byte_integer(b_prop.len() as u32, &mut b);
        b.put(b_prop.to_vec().as_slice()); //insert payload format indicator property
        let blank_property: Vec<Property> = vec![];
        if let Ok(p) = decode::property(&mut b) {
            assert_eq!(blank_property, p)
        }
    }

    #[test]
    fn test_property_type_byte_using_payload_format_indicator() {
        let mut b_prop = BytesMut::with_capacity(0);
        let mut b = BytesMut::with_capacity(100);

        b_prop.put_u8(PropertyIdentifiers::PayloadFormatIndicator as u8); // property identifier, Payload format indicator
        b_prop.put_u8(0x02); // value

        variable_byte_integer(b_prop.len() as u32, &mut b);
        b.put(b_prop); //insert payload format indicator property

        if let Ok(p) = decode::property(&mut b) {
            assert_eq!(
                vec![Property {
                    element_value: PropertyElement::Byte { value: 2 },
                    property_identifier: PropertyIdentifiers::PayloadFormatIndicator as u8
                }],
                p
            )
        }
    }

    #[test]
    fn test_property_type_using_double_payload_format_indicator() {
        let mut b_prop = BytesMut::with_capacity(0);
        let mut b = BytesMut::with_capacity(100);

        b_prop.put_u8(PropertyIdentifiers::PayloadFormatIndicator as u8); // property identifier, Payload format indicator
        b_prop.put_u8(0x02); // value

        b_prop.put_u8(PropertyIdentifiers::PayloadFormatIndicator as u8); // property identifier, Payload format indicator
        b_prop.put_u8(0x03); // value

        variable_byte_integer(b_prop.len() as u32, &mut b);
        b.put(b_prop); //insert payload format indicator property

        if let Ok(p) = decode::property(&mut b) {
            assert_eq!(
                vec![
                    Property {
                        element_value: PropertyElement::Byte { value: 2 },
                        property_identifier: PropertyIdentifiers::PayloadFormatIndicator as u8
                    },
                    Property {
                        element_value: PropertyElement::Byte { value: 3 },
                        property_identifier: PropertyIdentifiers::PayloadFormatIndicator as u8
                    },
                ],
                p
            )
        }
    }

    #[test]
    fn test_property_type_using_all_data_types() {
        let mut b_prop = BytesMut::with_capacity(0);
        let mut b = BytesMut::with_capacity(100);

        payload_format_indicator(99, &mut b_prop);
        message_expiry_interval(123456, &mut b_prop);
        content_type("hello", &mut b_prop);
        let mut binary_data = BytesMut::with_capacity(0);
        binary_data.put(vec![1u8, 2, 3, 4, 5, 6].as_slice());
        correlation_data(&binary_data, &mut b_prop);
        subscription_identifier(12345, &mut b_prop);

        variable_byte_integer(b_prop.len() as u32, &mut b);
        b.put(b_prop);
        if let Ok(p) = decode::property(&mut b) {
            assert_eq!(
                vec![
                    Property {
                        element_value: PropertyElement::Byte { value: 99 },
                        property_identifier: PropertyIdentifiers::PayloadFormatIndicator as u8,
                    },
                    Property {
                        element_value: PropertyElement::FourByteInteger { value: 123456 },
                        property_identifier: PropertyIdentifiers::MessageExpiryInterval as u8,
                    },
                    Property {
                        element_value: PropertyElement::UTF8EncodedString {
                            value: "hello".to_string()
                        },
                        property_identifier: PropertyIdentifiers::ContentType as u8,
                    },
                    Property {
                        element_value: PropertyElement::BinaryData {
                            value: vec![1u8, 2, 3, 4, 5, 6]
                        },
                        property_identifier: PropertyIdentifiers::CorrelationData as u8
                    },
                    Property {
                        element_value: PropertyElement::VariableByteInteger { value: 12345 },
                        property_identifier: PropertyIdentifiers::SubscriptionIdentifier as u8
                    }
                ],
                p
            )
        }
    }

    #[test]
    fn test_property_type_using_all_data_types_permutation2() {
        let mut b_prop = BytesMut::with_capacity(0);
        let mut b = BytesMut::with_capacity(100);

        subscription_identifier(12345, &mut b_prop);
        message_expiry_interval(123456, &mut b_prop);
        content_type("hello", &mut b_prop);
        let mut binary_data = BytesMut::with_capacity(0);
        binary_data.put(vec![1u8, 2, 3, 4, 5, 6].as_slice());
        correlation_data(&binary_data, &mut b_prop);
        payload_format_indicator(99, &mut b_prop);

        variable_byte_integer(b_prop.len() as u32, &mut b);
        b.put(b_prop);
        if let Ok(p) = decode::property(&mut b) {
            assert_eq!(
                vec![
                    Property {
                        element_value: PropertyElement::VariableByteInteger { value: 12345 },
                        property_identifier: PropertyIdentifiers::SubscriptionIdentifier as u8
                    },
                    Property {
                        element_value: PropertyElement::FourByteInteger { value: 123456 },
                        property_identifier: PropertyIdentifiers::MessageExpiryInterval as u8,
                    },
                    Property {
                        element_value: PropertyElement::UTF8EncodedString {
                            value: "hello".to_string()
                        },
                        property_identifier: PropertyIdentifiers::ContentType as u8,
                    },
                    Property {
                        element_value: PropertyElement::BinaryData {
                            value: vec![1u8, 2, 3, 4, 5, 6]
                        },
                        property_identifier: PropertyIdentifiers::CorrelationData as u8
                    },
                    Property {
                        element_value: PropertyElement::Byte { value: 99 },
                        property_identifier: PropertyIdentifiers::PayloadFormatIndicator as u8,
                    }
                ],
                p
            )
        }
    }

    #[test]
    fn test_for_invalid_properties_for_packet_type_connect_without_will_flag_not_set() {
        let assigned_property: Vec<Property> = vec![
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                AssignedClientIdentifier as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                SessionExpiryInterval as u8,
            ),
            Property::new(PropertyElement::Byte { value: 0u8 }, CorrelationData as u8),
            Property::new(PropertyElement::Byte { value: 0u8 }, ResponseTopic as u8),
        ];

        let invalid_property_set: Vec<PropertyIdentifiers> =
            vec![AssignedClientIdentifier, CorrelationData, ResponseTopic];
        let invalid_property_set: Vec<Property> = vec![
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                AssignedClientIdentifier as u8,
            ),
            Property::new(PropertyElement::Byte { value: 0u8 }, CorrelationData as u8),
            Property::new(PropertyElement::Byte { value: 0u8 }, ResponseTopic as u8),
        ];

        assert_eq!(
            invalid_property_set,
            invalid_property_for_packet_type(&assigned_property, vec![], CONNECT)
        );
    }

    #[test]
    fn test_for_invalid_properties_for_packet_type_connect_with_will_flag_set() {
        let assigned_property: Vec<Property> = vec![
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                AssignedClientIdentifier as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                SessionExpiryInterval as u8,
            ),
            Property::new(PropertyElement::Byte { value: 0u8 }, CorrelationData as u8),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                AssignedClientIdentifier as u8,
            ),
        ];

        let invalid_property: Vec<Property> = vec![
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                AssignedClientIdentifier as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                AssignedClientIdentifier as u8,
            ),
        ];

        assert_eq!(
            invalid_property,
            invalid_property_for_packet_type(
                &assigned_property,
                valid_properties_for_will(),
                CONNECT
            )
        );
    }

    #[test]
    fn should_return_invalid_properties_for_packet_type_connack_with_will_flag_not_set() {
        let assigned_property: Vec<Property> = vec![
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                SubscriptionIdentifier as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                SessionExpiryInterval as u8,
            ),
            Property::new(PropertyElement::Byte { value: 0u8 }, CorrelationData as u8),
            Property::new(PropertyElement::Byte { value: 0u8 }, ResponseTopic as u8),
        ];

        let invalid_property_set: Vec<Property> = vec![
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                SubscriptionIdentifier as u8,
            ),
            Property::new(PropertyElement::Byte { value: 0u8 }, CorrelationData as u8),
            Property::new(PropertyElement::Byte { value: 0u8 }, ResponseTopic as u8),
        ];

        assert_eq!(
            invalid_property_set,
            invalid_property_for_packet_type(&assigned_property, vec![], CONNACK)
        );
    }

    #[test]
    fn should_return_invalid_properties_for_packet_type_publish_with_will_flag_not_set() {
        let assigned_property: Vec<Property> = vec![
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::SessionExpiryInterval as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::CorrelationData as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::ResponseTopic as u8,
            ),
        ];

        let invalid_property: Vec<Property> = vec![Property::new(
            PropertyElement::Byte { value: 0u8 },
            PropertyIdentifiers::SessionExpiryInterval as u8,
        )];

        assert_eq!(
            invalid_property,
            invalid_property_for_packet_type(&assigned_property, vec![], PUBLISH)
        );
    }

    // puback

    #[test]
    fn should_return_invalid_properties_for_packet_type_puback_with_will_flag_not_set() {
        let assigned_property: Vec<Property> = vec![
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::ResponseTopic as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::CorrelationData as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::ReasonString as u8,
            ),
        ];

        let invalid_property: Vec<Property> = vec![
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::ResponseTopic as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::CorrelationData as u8,
            ),
        ];

        assert_eq!(
            invalid_property,
            invalid_property_for_packet_type(&assigned_property, vec![], PUBACK)
        );
    }

    // pubrec
    #[test]
    fn should_return_invalid_properties_for_packet_type_pubrec_with_will_flag_not_set() {
        let assigned_property: Vec<Property> = vec![
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::ResponseTopic as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::CorrelationData as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::ReasonString as u8,
            ),
        ];

        let invalid_property: Vec<Property> = vec![
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::ResponseTopic as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::CorrelationData as u8,
            ),
        ];

        assert_eq!(
            invalid_property,
            invalid_property_for_packet_type(&assigned_property, vec![], PUBREC)
        );
    }

    // pubrel
    #[test]
    fn should_return_invalid_properties_for_packet_type_pubrel_with_will_flag_not_set() {
        let assigned_property: Vec<Property> = vec![
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::ResponseTopic as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::CorrelationData as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::ReasonString as u8,
            ),
        ];

        let invalid_property: Vec<Property> = vec![
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::ResponseTopic as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::CorrelationData as u8,
            ),
        ];

        assert_eq!(
            invalid_property,
            invalid_property_for_packet_type(&assigned_property, vec![], PUBREL)
        );
    }

    // pubcomp
    #[test]
    fn should_return_invalid_properties_for_packet_type_pubcomp_with_will_flag_not_set() {
        let assigned_property: Vec<Property> = vec![
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::ResponseTopic as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::CorrelationData as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::ReasonString as u8,
            ),
        ];

        let invalid_property: Vec<Property> = vec![
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::ResponseTopic as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::CorrelationData as u8,
            ),
        ];

        assert_eq!(
            invalid_property,
            invalid_property_for_packet_type(&assigned_property, vec![], PUBCOMP)
        );
    }

    // subscribe
    #[test]
    fn should_return_invalid_properties_for_packet_type_subscribe_with_will_flag_not_set() {
        let assigned_property: Vec<Property> = vec![
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::ResponseTopic as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::CorrelationData as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::ReasonString as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::SubscriptionIdentifier as u8,
            ),
        ];

        let invalid_property: Vec<Property> = vec![
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::ResponseTopic as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::CorrelationData as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::ReasonString as u8,
            ),
        ];
        assert_eq!(
            invalid_property,
            invalid_property_for_packet_type(&assigned_property, vec![], SUBSCRIBE)
        );
    }

    // suback
    #[test]
    fn should_return_invalid_properties_for_packet_type_suback_with_will_flag_not_set() {
        let assigned_property: Vec<Property> = vec![
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::ResponseTopic as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::CorrelationData as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::ReasonString as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::SubscriptionIdentifier as u8,
            ),
        ];

        let invalid_property: Vec<Property> = vec![
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::ResponseTopic as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::CorrelationData as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::SubscriptionIdentifier as u8,
            ),
        ];

        assert_eq!(
            invalid_property,
            invalid_property_for_packet_type(&assigned_property, vec![], SUBACK)
        );
    }

    // unsubscribe
    #[test]
    fn should_return_invalid_properties_for_packet_type_unsubscribe_with_will_flag_not_set() {
        let assigned_property: Vec<Property> = vec![
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::ResponseTopic as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::CorrelationData as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::ReasonString as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::SubscriptionIdentifier as u8,
            ),
        ];

        let invalid_property: Vec<Property> = vec![
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::ResponseTopic as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::CorrelationData as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::ReasonString as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::SubscriptionIdentifier as u8,
            ),
        ];

        assert_eq!(
            invalid_property,
            invalid_property_for_packet_type(&assigned_property, vec![], UNSUBSCRIBE)
        );
    }
    // unsuback
    #[test]
    fn should_return_invalid_properties_for_packet_type_unsuback_with_will_flag_not_set() {
        let assigned_property: Vec<Property> = vec![
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::ResponseTopic as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::CorrelationData as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::ReasonString as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::SubscriptionIdentifier as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::UserProperty as u8,
            ),
        ];

        let invalid_property: Vec<Property> = vec![
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::ResponseTopic as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::CorrelationData as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::SubscriptionIdentifier as u8,
            ),
        ];

        assert_eq!(
            invalid_property,
            invalid_property_for_packet_type(&assigned_property, vec![], UNSUBACK),
        );
    }
    // pingreq
    #[test]
    fn should_return_invalid_properties_for_packet_type_pingreq_with_will_flag_not_set() {
        let assigned_property: Vec<Property> = vec![
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::ResponseTopic as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::CorrelationData as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::ReasonString as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::SubscriptionIdentifier as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::UserProperty as u8,
            ),
        ];

        let invalid_property: Vec<Property> = vec![
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::ResponseTopic as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::CorrelationData as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::ReasonString as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::SubscriptionIdentifier as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::UserProperty as u8,
            ),
        ];

        assert_eq!(
            invalid_property,
            invalid_property_for_packet_type(&assigned_property, vec![], PINGREQ)
        );
    }

    // disconnect
    #[test]
    fn should_return_invalid_properties_for_packet_type_disconnect_with_will_flag_not_set() {
        let assigned_property: Vec<Property> = vec![
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::ResponseTopic as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::CorrelationData as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::ReasonString as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::SubscriptionIdentifier as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::UserProperty as u8,
            ),
        ];

        let invalid_property: Vec<Property> = vec![
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::ResponseTopic as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::CorrelationData as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::SubscriptionIdentifier as u8,
            ),
        ];

        assert_eq!(
            invalid_property,
            invalid_property_for_packet_type(&assigned_property, vec![], DISCONNECT)
        );
    }
    // auth
    #[test]
    fn should_return_invalid_properties_for_packet_type_auth_with_will_flag_not_set() {
        let assigned_property: Vec<Property> = vec![
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::ResponseTopic as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::CorrelationData as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::ReasonString as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::SubscriptionIdentifier as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::UserProperty as u8,
            ),
        ];

        let invalid_property_set: Vec<PropertyIdentifiers> =
            vec![ResponseTopic, CorrelationData, SubscriptionIdentifier];

        let invalid_property: Vec<Property> = vec![
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::ResponseTopic as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::CorrelationData as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::SubscriptionIdentifier as u8,
            ),
        ];

        assert_eq!(
            invalid_property,
            invalid_property_for_packet_type(&assigned_property, vec![], AUTH)
        );
    }
    #[test]
    fn test_property_type_four_byte_integer_using_message_expiry_interval_type() {
        let mut b_prop = BytesMut::with_capacity(2);
        let mut b = BytesMut::with_capacity(100);
        b_prop.put_u8(0x02); // property identifier, message expiry interval
        b_prop.put_u32(0x01); // message expiry interval

        variable_byte_integer(b_prop.len() as u32, &mut b);
        b.put(b_prop);

        if let Ok(p) = decode::property(&mut b) {
            assert_eq!(
                vec![Property {
                    element_value: PropertyElement::FourByteInteger { value: 1 },
                    property_identifier: 0x02,
                }],
                p
            )
        }
    }

    #[test]
    fn test_property_type_utf_8_encode_string_using_content_type() {
        let mut b_prop = BytesMut::with_capacity(2);
        let mut b = BytesMut::with_capacity(100);
        let b_string = String::from("hello world");
        b_prop.put_u8(PropertyIdentifiers::ContentType as u8); // property identifier, UTF-8 Encoded String

        b_prop.put_u16(b_string.len() as u16);
        b_prop.put(b_string.as_bytes());

        variable_byte_integer(b_prop.len() as u32, &mut b);
        b.put(b_prop);

        if let Ok(p) = decode::property(&mut b) {
            assert_eq!(
                vec![Property {
                    element_value: PropertyElement::UTF8EncodedString {
                        value: String::from("hello world")
                    },
                    property_identifier: PropertyIdentifiers::ContentType as u8,
                }],
                p
            )
        }
    }

    #[test]
    fn test_property_type_binary_data_using_correlation_data() {
        let mut b_prop = BytesMut::with_capacity(2);
        let mut b = BytesMut::with_capacity(100);
        let b_binarydata = vec![1u8, 2u8, 3u8, 4u8, 5u8];

        b_prop.put_u8(PropertyIdentifiers::CorrelationData as u8);
        b_prop.put_u16(b_binarydata.len() as u16);
        b_prop.put(b_binarydata.as_slice());
        variable_byte_integer(b_prop.len() as u32, &mut b);
        b.put(b_prop);

        if let Ok(p) = decode::property(&mut b) {
            assert_eq!(
                vec![Property {
                    element_value: PropertyElement::BinaryData {
                        value: vec![1u8, 2, 3, 4, 5]
                    },
                    property_identifier: PropertyIdentifiers::CorrelationData as u8,
                }],
                p
            )
        }
    }

    #[test]
    fn test_property_type_variable_byte_integer() {
        let mut b_prop = BytesMut::with_capacity(20);
        let mut b = BytesMut::with_capacity(100);
        let b_integer: u32 = 268_435_455;

        b_prop.put_u8(PropertyIdentifiers::SubscriptionIdentifier as u8);
        variable_byte_integer(b_integer, &mut b_prop);

        variable_byte_integer(b_prop.len() as u32, &mut b); // size of property

        b.put(b_prop);

        if let Ok(p) = decode::property(&mut b) {
            assert_eq!(
                vec![Property {
                    element_value: PropertyElement::VariableByteInteger { value: b_integer },
                    property_identifier: PropertyIdentifiers::SubscriptionIdentifier as u8,
                }],
                p
            )
        }
    }

    #[test]
    fn test_property_type_four_byte_integer() {
        let mut b_prop = BytesMut::with_capacity(2);
        let mut b = BytesMut::with_capacity(100);
        let b_integer: FourByteIntegerT = 269_435_455;

        b_prop.put_u8(PropertyIdentifiers::SessionExpiryInterval as u8);
        b_prop.put_u32(b_integer);
        variable_byte_integer(b_prop.len() as u32, &mut b);
        b.put(b_prop);

        if let Ok(p) = decode::property(&mut b) {
            assert_eq!(
                vec![Property {
                    element_value: PropertyElement::FourByteInteger { value: b_integer },
                    property_identifier: PropertyIdentifiers::SessionExpiryInterval as u8
                }],
                p
            )
        }
    }

    #[test]
    fn test_property_type_two_integer_using_server_keep_alive() {
        let mut b_prop = BytesMut::with_capacity(2);
        let mut b = BytesMut::with_capacity(100);

        b_prop.put_u8(PropertyIdentifiers::ServerKeepAlive as u8);
        b_prop.put_u16(0x1001);

        variable_byte_integer(b_prop.len() as u32, &mut b);
        b.put(b_prop);

        if let Ok(p) = decode::property(&mut b) {
            assert_eq!(
                vec![Property {
                    element_value: PropertyElement::TwoByteInteger { value: 0x1001 },
                    property_identifier: PropertyIdentifiers::ServerKeepAlive as u8
                }],
                p
            )
        }
    }

    #[test]
    fn test_property_type_utf8_string_pair_using_user_property() {
        let mut b_prop = BytesMut::with_capacity(2);
        let mut b = BytesMut::with_capacity(100);

        let key = b"Hello";
        let value = b"World";

        b_prop.put_u8(PropertyIdentifiers::UserProperty as u8);
        encode::utf8_string_pair("Hello", "World", &mut b_prop);
        variable_byte_integer(b_prop.len() as u32, &mut b);
        b.put(b_prop);

        if let Ok(p) = decode::property(&mut b) {
            assert_eq!(
                vec![Property {
                    element_value: PropertyElement::UTF8StringPair {
                        value: Utf8stringPairT {
                            key: String::from_utf8(key.to_vec()).unwrap(),
                            value: String::from_utf8(value.to_vec()).unwrap(),
                        }
                    },
                    property_identifier: PropertyIdentifiers::UserProperty as u8
                }],
                p
            )
        }
    }

    #[test]
    fn test_decoding_all_property_types() {
        // insert Payload format indicator, byte
        // insert Message expiry interval
    }

    #[test]
    fn test_two_user_properties_are_valid() {
        // let properties = vec![
        //     PropertyIdentifiers::UserProperty,
        //     PropertyIdentifiers::UserProperty,
        // ];
        let properties: Vec<Property> = vec![Property::new(
            PropertyElement::Byte { value: 0u8 },
            PropertyIdentifiers::UserProperty as u8,
        )];

        let returned_properties: Vec<Property> = vec![];
        assert_eq!(
            check_for_non_unique_properties(&properties),
            returned_properties
        );
    }
    #[test]
    fn test_zero_user_properties_are_valid() {
        // let format = fmt::format()
        //     .with_level(false) // don't include levels in formatted output
        //     .with_target(false) // don't include targets
        //     .with_thread_ids(true) // include the thread ID of the current thread
        //     .with_thread_names(true) // include the name of the current thread
        //     .compact(); // use the `Compact` formatting style.
        // let _ = tracing_subscriber::fmt().init();
        let span = span!(Level::TRACE, "my first span");
        debug!("////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////");
        event!(Level::TRACE, "event is cool....");
        trace!("//////////////////////////");
        info!("hello from tracing::info");
        let properties = vec![];
        let returned_properties: Vec<Property> = vec![];
        assert_eq!(
            check_for_non_unique_properties(&properties),
            returned_properties
        );
    }

    #[test]
    fn test_two_assigned_client_id_dentifier_are_invalid() {
        let properties: Vec<Property> = vec![
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::AssignedClientIdentifier as u8,
            ),
            Property::new(
                PropertyElement::Byte { value: 0u8 },
                PropertyIdentifiers::AssignedClientIdentifier as u8,
            ),
        ];

        let mut returned_properties: Vec<Property>;

        // let returned_properties: Vec<PropertyIdentifiers> =
        //     vec![PropertyIdentifiers::AssignedClientIdentifier];

        let returned_properties: Vec<Property> = vec![Property::new(
            PropertyElement::Byte { value: 0u8 },
            PropertyIdentifiers::AssignedClientIdentifier as u8,
        )];
        assert_eq!(
            check_for_non_unique_properties(&properties),
            returned_properties
        );
    }

    #[test]
    fn test_parse_connect_control_packet() {
        // let mut packet = BytesMut::with_capacity(200);
        // packet.put_u8(0b0001_0000);
        // //packet.put variable byte integer

        // select control packet type

        //
    }
}
