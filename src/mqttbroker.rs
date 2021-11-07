mod mqtt_broker {
    use crate::mqttbroker::mqtt_broker::types::{
        BinaryDataT, ByteT, FourByteIntegerT, TwoByteIntegerT, Utf8stringPairT,
        VariableByteIntegerT,
    };

    pub mod types {
        #[derive(Debug, PartialEq)]
        pub struct Utf8stringPairT {
            pub key: String,
            pub value: String,
        }
        pub type ByteT = u8;
        pub type FourByteIntegerT = u32;
        pub type BinaryDataT = Vec<u8>;
        pub type TwoByteIntegerT = u16;
        pub type VariableByteIntegerT = u32;
    }

    #[derive(Debug, PartialEq)]
    pub enum Property {
        Byte {
            value: ByteT,
            property_identifier: u8,
        },
        FourByteInteger {
            value: FourByteIntegerT,
            property_identifier: u8,
        },
        UTF8EncodedString {
            value: String,
            property_identifier: u8,
        },
        BinaryData {
            value: BinaryDataT,
            property_identifier: u8,
        },
        TwoByteInteger {
            value: TwoByteIntegerT,
            property_identifier: u8,
        },
        UTF8StringPair {
            value: Utf8stringPairT,
            property_identifier: u8,
        },
        VariableByteInteger {
            value: VariableByteIntegerT,
            property_identifier: u8,
        },
    }
    pub mod property_identifiers {
        pub const PAYLOAD_FORMAT_INDICATOR: u8 = 0x01;
        pub const MESSAGE_EXPIRY_INTERVAL: u8 = 0x02;
        pub const CONTENT_TYPE: u8 = 0x03;
        pub const RESPONSE_TOPIC: u8 = 0x08;
        pub const CORRELATION_DATA: u8 = 0x09;
        pub const SUBSCRIPTION_IDENTIFIER: u8 = 0x0b;
        pub const SESSION_EXPIRY_INTERVAL: u8 = 0x11;
        pub const ASSIGNED_CLIENT_IDENTIFIER: u8 = 0x12;
        pub const SERVER_KEEP_ALIVE: u8 = 0x13;
        pub const AUTHENTICATION_METHOD: u8 = 0x15;
        pub const AUTHENTICATION_DATA: u8 = 0x16;
        pub const REQUEST_PROBLEM_INFORMATION: u8 = 0x17;
        pub const WILL_DELAY_INTERVAL: u8 = 0x18;
        pub const REQUEST_RESPONSE_INFORMATION: u8 = 0x19;
        pub const RESPONSE_INFORMATION: u8 = 0x1a;
        pub const SERVER_REFERENCE: u8 = 0x1c;
        pub const REASON_STRING: u8 = 0x1f;
        pub const RECEIVE_MAXIMUM: u8 = 0x21;
        pub const TOPIC_ALIAS_MAXIMUM: u8 = 0x22;
        pub const TOPIC_ALIAS: u8 = 0x23;
        pub const MAXIMUM_QOS: u8 = 0x24;
        pub const RETAIN_AVAILABLE: u8 = 0x25;
        pub const USER_PROPERTY: u8 = 0x26;
        pub const MAXIMUM_PACKET_SIZE: u8 = 0x27;
        pub const WILDCARD_SUBSCRIPTION_AVAILABLE: u8 = 0x28;
        pub const SUBSCRIPTION_IDENTIFIER_AVAILABLE: u8 = 0x29;
        pub const SHARED_SUBSCRIPTION_AVAILABLE: u8 = 0x2a;
    }

    pub mod packet_types {
        pub const CONNECT: u8 = 0x01;
        pub const CONNACK: u8 = 0x02;
        pub const PUBLISH: u8 = 0x03;
        pub const PUBACK: u8 = 0x04;
        pub const PUBREC: u8 = 0x05;
        pub const PUBREL: u8 = 0x06;
        pub const PUBCOMP: u8 = 0x07;
        pub const SUBSCRIBE: u8 = 0x08;
        pub const SUBACK: u8 = 0x09;
        pub const UNSUBSCRIBE: u8 = 0x0a;
        pub const UNSUBACK: u8 = 0x0b;
        pub const PINGREQ: u8 = 0x0c;
        pub const PINGRESP: u8 = 0x0d;
        pub const DISCONNECT: u8 = 0x0e;
        pub const AUTH: u8 = 0x0f;
    }

    pub mod reason_codes {
        pub const SUCCESS: u8 = 0x00;
        pub const NORMAL_DISCONNECTION: u8 = 0x00;
        pub const GRANTED_QOS_0: u8 = 0x00;
        pub const GRANTED_QOS_1: u8 = 0x01;
        pub const GRANTED_QOS_2: u8 = 0x02;
        pub const DISCONNECT_WITH_WILL_MESSAGE: u8 = 0x04;
        pub const NO_MATCHING_SUBSCRIBERS: u8 = 0x10;
        pub const NO_SUBSCRIPTION_EXISTED: u8 = 0x11;
        pub const CONTINUE_AUTHENTICATION: u8 = 0x18;
        pub const RE_AUTHENTICATE: u8 = 0x19;
        pub const UNSPECIFIED_ERROR: u8 = 0x80;
        pub const MALFORMED_PACKET: u8 = 0x81;
        pub const PROTOCOL_ERROR: u8 = 0x82;
        pub const IMPLEMENTATION_SPECIFIC_ERROR: u8 = 0x83;
        pub const UNSUPPORTED_PROTOCOL_VERSION: u8 = 0x84;
        pub const CLIENT_IDENTIFIER_NOT_VALID: u8 = 0x85;
        pub const BAD_USER_NAME_OR_PASSWORD: u8 = 0x86;
        pub const NOT_AUTHORISED: u8 = 0x87;
        pub const SERVER_UNAVAILABLE: u8 = 0x88;
        pub const SERVER_BUSY: u8 = 0x89;
        pub const BANNED: u8 = 0x8a;
        pub const SERVER_SHUTTING_DOWN: u8 = 0x8B;
        pub const BAD_AUTHENTICATION_METHOD: u8 = 0x8c;
        pub const KEEP_ALIVE_TIMEOUT: u8 = 0x8d;
        pub const SESSION_TAKEN_OVER: u8 = 0x8e;
        pub const TOPIC_FILTER_INVALID: u8 = 0x8f;
        pub const TOPIC_NAME_INVALID: u8 = 0x90;
        pub const PACKET_IDENTIFIER_IN_USE: u8 = 0x91;
        pub const PACKET_IDENTIFIER_NOT_FOUND: u8 = 0x92;
        pub const RECEIVE_MAXIMUM_EXCEEDED: u8 = 0x93;
        pub const TOPIC_ALIAS_INVALID: u8 = 0x94;
        pub const PACKET_TOO_LARGE: u8 = 0x95;
        pub const MESSAGE_RATE_TOO_HIGH: u8 = 0x96;
        pub const QUOTA_EXCEEDED: u8 = 0x97;
        pub const ADMINISTRATIVE_ACTION: u8 = 0x98;
        pub const PAYLOAD_FORMAT_INVALID: u8 = 0x99;
        pub const RETAIN_NOT_SUPPORTED: u8 = 0x9a;
        pub const QOS_NOT_SUPPORTED: u8 = 0x9b;
        pub const USE_ANOTHER_SERVER: u8 = 0x9c;
        pub const SERVER_MOVED: u8 = 0x9d;
        pub const SHARED_SUBSCRIPTIONS_NOT_SUPPORTED: u8 = 0x9e;
        pub const CONNECTION_RATE_EXCEEDED: u8 = 0x9f;
        pub const MAXIMUM_CONECT_TIME: u8 = 0xa0;
        pub const SUBSCRIPTION_IDENTIFIERS_NOT_SUPPORTED: u8 = 0xa1;
        pub const WILDCARD_SUBSCRIPTIONS_NOT_SUPPORTED: u8 = 0xa2;
    }

    pub mod encode {
        use crate::mqttbroker::mqtt_broker::property_identifiers::{
            CONTENT_TYPE, CORRELATION_DATA, MESSAGE_EXPIRY_INTERVAL, PAYLOAD_FORMAT_INDICATOR,
            RESPONSE_TOPIC, SUBSCRIPTION_IDENTIFIER,
        };
        use crate::mqttbroker::mqtt_broker::types::{
            FourByteIntegerT, TwoByteIntegerT, VariableByteIntegerT,
        };
        use bytes::{BufMut, BytesMut};
        use std::ffi::OsString;
        use thiserror::Error;

        #[derive(Error, Debug, PartialEq)]
        pub enum EncodeError {
            #[error(
                "Number is too large, greater than 268,435,455, to convert to a variable integer"
            )]
            NumberTooLarge,
        }

        fn two_byte_integer(i: TwoByteIntegerT, b: &mut BytesMut) {
            b.put_u16(i);
        }

        fn four_byte_integer(i: FourByteIntegerT, b: &mut BytesMut) {
            b.put_u32(i);
        }

        pub fn utf8_encoded_string(s: &str, b: &mut BytesMut) {
            b.put_u16(s.len() as u16);
            b.put_slice(s.as_bytes());
        }

        pub fn variable_byte_integer(
            i: VariableByteIntegerT,
            b: &mut BytesMut,
        ) -> Result<(), EncodeError> {
            if i > 268_435_455 {
                return Err(EncodeError::NumberTooLarge);
            }
            let mut encoded_byte: u8;
            let mut to_encode = i;
            loop {
                encoded_byte = to_encode.rem_euclid(128u32) as u8;
                to_encode = to_encode.div_euclid(128u32);
                if to_encode > 0 {
                    encoded_byte |= 128
                }
                b.put_u8(encoded_byte);
                if to_encode == 0 {
                    break;
                }
            }
            Ok(())
        }

        pub fn binary_data(binary_data: &BytesMut, buffer: &mut BytesMut) {
            let size: u16 = binary_data.len() as u16;
            buffer.put_u16(size);
            buffer.put_slice(binary_data);
        }

        pub fn utf8_string_pair(key: &str, value: &str, buf: &mut BytesMut) {
            utf8_encoded_string(key, buf);
            utf8_encoded_string(value, buf);
        }

        #[cfg(test)]
        mod test {
            use crate::mqttbroker::mqtt_broker::encode;
            use crate::mqttbroker::mqtt_broker::encode::EncodeError;
            use bytes::{Buf, BufMut, BytesMut};

            #[test]
            fn test_encode_128() {
                let mut b = BytesMut::with_capacity(2);
                assert_eq!(Ok(()), encode::variable_byte_integer(128, &mut b));
                assert_eq!(b.to_vec(), vec![0x80, 1]);
            }

            #[test]
            fn test_encode_256() {
                let mut b = BytesMut::with_capacity(2);
                assert_eq!(Ok(()), encode::variable_byte_integer(256, &mut b));
                assert_eq!(b.to_vec(), vec![0x80, 2]);
            }

            #[test]
            fn test_encode_32767() {
                let mut b = BytesMut::with_capacity(2);
                assert_eq!(Ok(()), encode::variable_byte_integer(32767, &mut b));
                assert_eq!(b.to_vec(), vec![0xff, 0xff, 1]);
            }

            #[test]
            fn test_encode_number_too_large() {
                let mut b = BytesMut::with_capacity(4);
                let result = encode::variable_byte_integer(300_000_000, &mut b);
                assert_eq!(EncodeError::NumberTooLarge, result.err().unwrap());
            }

            #[test]
            fn test_string() {
                let mut b = BytesMut::with_capacity(20);
                let s = "hello world";
                encode::utf8_encoded_string(s, &mut b);
                let length = b.get_u16();
                assert_eq!(s.as_bytes(), b.to_vec());
                assert_eq!(length as usize, s.len());
            }
        }
    }

    mod decode {
        use crate::mqttbroker::mqtt_broker::decode::DecodeError::UTF8Errors;
        use crate::mqttbroker::mqtt_broker::types::{
            BinaryDataT, FourByteIntegerT, TwoByteIntegerT, Utf8stringPairT, VariableByteIntegerT,
        };
        use crate::mqttbroker::mqtt_broker::Property;
        use crate::mqttbroker::mqtt_broker::*;
        use bytes::{Buf, BytesMut};
        use std::collections::HashMap;
        use thiserror::Error;

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

        pub enum PayLoad {
            Required,
            Optional,
            None,
        }

        lazy_static! {
            static ref PAYLOADREQUIREDSTATUS: HashMap<u8, PayLoad> = {
                let mut h = HashMap::new();
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

        #[derive(Error, Debug, PartialEq)]
        pub enum DecodeError {
            #[error("Not enough bytes decoding {0}")]
            NotEnoughBytes(String),
            #[error("The variable int does not have the MSB clear on the fourth byte.")]
            NotValidVarInt,
            #[error(
                "Not enough bytes found for decoding {2}. Require {0} bytes, found {1} bytes."
            )]
            MoreBytesRequired(u16, u16, String),
            #[error("Converting bytes to utf-8 string for {0}")]
            UTF8Errors(String),
        }

        fn two_byte_integer(
            name: String,
            b: &mut BytesMut,
        ) -> anyhow::Result<TwoByteIntegerT, DecodeError> {
            if b.len() < 2 {
                Err(DecodeError::MoreBytesRequired(2, b.len() as u16, name))
            } else {
                Ok(b.get_u16())
            }
        }

        fn four_byte_integer(
            name: String,
            b: &mut BytesMut,
        ) -> anyhow::Result<FourByteIntegerT, DecodeError> {
            if b.len() < 4 {
                Err(DecodeError::MoreBytesRequired(4, b.len() as u16, name))
            } else {
                Ok(b.get_u32())
            }
        }

        fn utf8_string(name: String, b: &mut BytesMut) -> anyhow::Result<String, DecodeError> {
            if b.len() < 2 {
                Err(DecodeError::NotEnoughBytes(name))
            } else {
                let mut i = b.iter();
                let string_length = *(i.next().unwrap()) as u16 * 256 + *(i.next().unwrap()) as u16;
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

        fn binary(name: String, b: &mut BytesMut) -> anyhow::Result<BinaryDataT, DecodeError> {
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
            match utf8_string(name_of_key.clone(), b) {
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
            match utf8_string(name_of_value.clone(), b) {
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

        fn varint(b: &mut bytes::BytesMut) -> anyhow::Result<VariableByteIntegerT, DecodeError> {
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

        fn property(b: &mut bytes::BytesMut) -> anyhow::Result<Vec<Property>, DecodeError> {
            println!("pre varint length is {}", b.len());
            let length = varint(b)?;
            println!("post varint length is {}", b.len());
            let mut sub_b = b.split_to((length) as usize);
            println!("post sub_b is {}", sub_b.len());

            let mut p_vec: Vec<Property> = vec![];
            println!("property length {}", length);

            while !sub_b.is_empty() {
                let property_identifier = sub_b.get_u8();
                let p = match property_identifier {
                    1 | 23 | 25 | 36 | 37 | 40 | 41 | 42 => Property::Byte {
                        value: sub_b.get_u8(),
                        property_identifier,
                    },
                    2 | 17 | 24 | 39 => {
                        let four_byte_integer = four_byte_integer(
                            String::from(
                                *PROPERTYNAME.get(&property_identifier).to_owned().unwrap(),
                            ),
                            &mut sub_b,
                        )?;
                        Property::FourByteInteger {
                            value: four_byte_integer,
                            property_identifier,
                        }
                    }
                    3 | 8 | 18 | 21 | 26 | 28 | 31 => {
                        let str = utf8_string(
                            String::from(
                                *PROPERTYNAME.get(&property_identifier).to_owned().unwrap(),
                            ),
                            &mut sub_b,
                        )?;

                        Property::UTF8EncodedString {
                            value: str,
                            property_identifier,
                        }
                    }
                    9 | 22 => {
                        let binary_data = binary(
                            String::from(
                                *PROPERTYNAME.get(&property_identifier).to_owned().unwrap(),
                            ),
                            &mut sub_b,
                        )?;

                        Property::BinaryData {
                            value: binary_data,
                            property_identifier,
                        }
                    }

                    19 | 33 | 34 | 35 => {
                        let two_byte_integer = two_byte_integer(
                            String::from(
                                *PROPERTYNAME.get(&property_identifier).to_owned().unwrap(),
                            ),
                            &mut sub_b,
                        )?;

                        Property::TwoByteInteger {
                            value: two_byte_integer,
                            property_identifier,
                        }
                    }

                    38 => {
                        let utf8_string_pair = utf8_string_pair(
                            String::from(
                                *PROPERTYNAME.get(&property_identifier).to_owned().unwrap(),
                            ),
                            &mut sub_b,
                        )?;
                        Property::UTF8StringPair {
                            value: utf8_string_pair,
                            property_identifier,
                        }
                    }

                    11 => {
                        println!("pre variable_byte_integer sub_p len is {}", sub_b.len());
                        let variable_byte_integer = varint(&mut sub_b)?;
                        println!("post variable_byte_integer sub_p len is {}", sub_b.len());
                        Property::VariableByteInteger {
                            value: variable_byte_integer,
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
            use crate::mqttbroker::mqtt_broker::decode::{utf8_string_pair, varint, DecodeError};
            use crate::mqttbroker::mqtt_broker::encode::{
                binary_data, utf8_encoded_string, variable_byte_integer, EncodeError,
            };
            use crate::mqttbroker::mqtt_broker::packet_types::{
                AUTH, CONNACK, CONNECT, DISCONNECT, PINGREQ, PUBACK, PUBCOMP, PUBLISH, PUBREC,
                PUBREL, SUBACK, SUBSCRIBE, UNSUBACK, UNSUBSCRIBE,
            };
            use crate::mqttbroker::mqtt_broker::property_identifiers::{
                ASSIGNED_CLIENT_IDENTIFIER, CORRELATION_DATA, REASON_STRING, RESPONSE_TOPIC,
                SESSION_EXPIRY_INTERVAL, SUBSCRIPTION_IDENTIFIER, USER_PROPERTY,
            };
            use crate::mqttbroker::mqtt_broker::types::{FourByteIntegerT, Utf8stringPairT};
            use crate::mqttbroker::mqtt_broker::utility::invalid_property_for_packet_type;
            use crate::mqttbroker::mqtt_broker::ReasonCode::ProtocolError;
            use crate::mqttbroker::mqtt_broker::{decode, encode, property_identifiers, Property};
            use bytes::BufMut;
            use bytes::BytesMut;
            use std::collections::HashSet;
            use std::iter::FromIterator;

            fn payload_format_indicator(val: u8, buf: &mut BytesMut) {
                buf.put_u8(property_identifiers::PAYLOAD_FORMAT_INDICATOR);
                buf.put_u8(val);
            }

            fn message_expiry_interval(value: u32, buf: &mut BytesMut) {
                buf.put_u8(property_identifiers::MESSAGE_EXPIRY_INTERVAL);
                buf.put_u32(value);
            }

            fn content_type(value: &str, buf: &mut BytesMut) {
                buf.put_u8(property_identifiers::CONTENT_TYPE);
                utf8_encoded_string(value, buf);
            }

            fn correlation_data(value: &BytesMut, buf: &mut BytesMut) {
                buf.put_u8(property_identifiers::CORRELATION_DATA);
                binary_data(&value, buf);
            }

            fn subscription_identifier(value: u32, buf: &mut BytesMut) -> Result<(), EncodeError> {
                buf.put_u8(property_identifiers::SUBSCRIPTION_IDENTIFIER);
                variable_byte_integer(value, buf)
            }

            fn session_expiry_interval(value: u32, buf: &mut BytesMut) {
                buf.put_u8(property_identifiers::SESSION_EXPIRY_INTERVAL);
                buf.put_u32(value);
            }

            fn assigned_client_identifier(value: &str, buf: &mut BytesMut) {
                buf.put_u8(property_identifiers::ASSIGNED_CLIENT_IDENTIFIER);
                utf8_encoded_string(value, buf);
            }

            fn user_property(key: &str, value: &str, buf: &mut BytesMut) {
                buf.put_u8(property_identifiers::USER_PROPERTY);
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

                b_prop.put_u8(property_identifiers::PAYLOAD_FORMAT_INDICATOR); // property identifier
                b_prop.put_u8(0x0); //
                if let Ok(..) = encode::variable_byte_integer(b_prop.len() as u32, &mut b) {
                    b.put(b_prop);

                    assert_eq!(2, *b.get(0).unwrap());
                }
            }

            #[test]
            fn test_property_payload_with_zero_property_length() {
                let mut b_prop = BytesMut::with_capacity(2);
                let mut b = BytesMut::with_capacity(100);

                variable_byte_integer(b_prop.len() as u32, &mut b);
                b.put(b_prop.to_vec().as_slice()); //insert payload format indicator property
                let blank_property: Vec<Property> = vec![];
                if let Ok(p) = decode::property(&mut b) {
                    assert_eq!(blank_property, p)
                }
            }

            #[test]
            fn test_property_payload_with_7_properties_of_different_types() {
                let mut b_prop = BytesMut::with_capacity(2);
                let mut b = BytesMut::with_capacity(100);
            }

            #[test]
            fn test_property_type_byte_using_payload_format_indicator() {
                let mut b_prop = BytesMut::with_capacity(0);
                let mut b = BytesMut::with_capacity(100);

                b_prop.put_u8(property_identifiers::PAYLOAD_FORMAT_INDICATOR); // property identifier, Payload format indicator
                b_prop.put_u8(0x02); // value

                variable_byte_integer(b_prop.len() as u32, &mut b);
                b.put(b_prop); //insert payload format indicator property

                if let Ok(p) = decode::property(&mut b) {
                    assert_eq!(
                        vec![Property::Byte {
                            value: 2,
                            property_identifier: property_identifiers::PAYLOAD_FORMAT_INDICATOR
                        }],
                        p
                    )
                }
            }

            #[test]
            fn test_property_type_using_double_payload_format_indicator() {
                let mut b_prop = BytesMut::with_capacity(0);
                let mut b = BytesMut::with_capacity(100);

                b_prop.put_u8(property_identifiers::PAYLOAD_FORMAT_INDICATOR); // property identifier, Payload format indicator
                b_prop.put_u8(0x02); // value

                b_prop.put_u8(property_identifiers::PAYLOAD_FORMAT_INDICATOR); // property identifier, Payload format indicator
                b_prop.put_u8(0x03); // value

                variable_byte_integer(b_prop.len() as u32, &mut b);
                b.put(b_prop); //insert payload format indicator property

                if let Ok(p) = decode::property(&mut b) {
                    assert_eq!(
                        vec![
                            Property::Byte {
                                value: 2,
                                property_identifier: property_identifiers::PAYLOAD_FORMAT_INDICATOR
                            },
                            Property::Byte {
                                value: 3,
                                property_identifier: property_identifiers::PAYLOAD_FORMAT_INDICATOR
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
                            Property::Byte {
                                value: 99,
                                property_identifier: property_identifiers::PAYLOAD_FORMAT_INDICATOR,
                            },
                            Property::FourByteInteger {
                                value: 123456,
                                property_identifier: property_identifiers::MESSAGE_EXPIRY_INTERVAL,
                            },
                            Property::UTF8EncodedString {
                                value: "hello".to_string(),
                                property_identifier: property_identifiers::CONTENT_TYPE,
                            },
                            Property::BinaryData {
                                value: vec![1u8, 2, 3, 4, 5, 6],
                                property_identifier: property_identifiers::CORRELATION_DATA
                            },
                            Property::VariableByteInteger {
                                value: 12345,
                                property_identifier: property_identifiers::SUBSCRIPTION_IDENTIFIER
                            }
                        ],
                        p
                    )
                }
            }

            #[test]
            fn test_for_invalid_properties_for_packet_type_connect_without_will_flag_not_set() {
                let assigned_property: HashSet<u8> = HashSet::from_iter(vec![
                    ASSIGNED_CLIENT_IDENTIFIER,
                    SESSION_EXPIRY_INTERVAL,
                    CORRELATION_DATA,
                    RESPONSE_TOPIC,
                ]);
                let invalid_property_set: HashSet<u8> = HashSet::from_iter(vec![
                    ASSIGNED_CLIENT_IDENTIFIER,
                    CORRELATION_DATA,
                    RESPONSE_TOPIC,
                ]);

                assert_eq!(
                    invalid_property_set,
                    invalid_property_for_packet_type(assigned_property, CONNECT, false)
                );
            }

            #[test]
            fn test_for_invalid_properties_for_packet_type_connect_with_will_flag_set() {
                let assigned_property: HashSet<u8> = HashSet::from_iter(vec![
                    ASSIGNED_CLIENT_IDENTIFIER,
                    SESSION_EXPIRY_INTERVAL,
                    CORRELATION_DATA,
                    RESPONSE_TOPIC,
                ]);
                let invalid_property_set: HashSet<u8> =
                    HashSet::from_iter(vec![ASSIGNED_CLIENT_IDENTIFIER]);

                assert_eq!(
                    invalid_property_set,
                    invalid_property_for_packet_type(assigned_property, CONNECT, true)
                );
            }

            #[test]
            fn should_return_invalid_properties_for_packet_type_connack_with_will_flag_not_set() {
                let assigned_property: HashSet<u8> = HashSet::from_iter(vec![
                    SUBSCRIPTION_IDENTIFIER,
                    SESSION_EXPIRY_INTERVAL,
                    CORRELATION_DATA,
                    RESPONSE_TOPIC,
                ]);
                let invalid_property_set: HashSet<u8> = HashSet::from_iter(vec![
                    SUBSCRIPTION_IDENTIFIER,
                    CORRELATION_DATA,
                    RESPONSE_TOPIC,
                ]);

                assert_eq!(
                    invalid_property_set,
                    invalid_property_for_packet_type(assigned_property, CONNACK, false)
                );
            }

            #[test]
            fn should_return_invalid_properties_for_packet_type_publish_with_will_flag_not_set() {
                let assigned_property: HashSet<u8> = HashSet::from_iter(vec![
                    SESSION_EXPIRY_INTERVAL,
                    CORRELATION_DATA,
                    RESPONSE_TOPIC,
                ]);
                let invalid_property_set: HashSet<u8> =
                    HashSet::from_iter(vec![SESSION_EXPIRY_INTERVAL]);

                assert_eq!(
                    invalid_property_set,
                    invalid_property_for_packet_type(assigned_property, PUBLISH, false)
                );
            }

            // puback

            #[test]
            fn should_return_invalid_properties_for_packet_type_puback_with_will_flag_not_set() {
                let assigned_property: HashSet<u8> =
                    HashSet::from_iter(vec![RESPONSE_TOPIC, CORRELATION_DATA, REASON_STRING]);
                let invalid_property_set: HashSet<u8> =
                    HashSet::from_iter(vec![RESPONSE_TOPIC, CORRELATION_DATA]);

                assert_eq!(
                    invalid_property_set,
                    invalid_property_for_packet_type(assigned_property, PUBACK, false)
                );
            }

            // pubrec
            #[test]
            fn should_return_invalid_properties_for_packet_type_pubrec_with_will_flag_not_set() {
                let assigned_property: HashSet<u8> =
                    HashSet::from_iter(vec![RESPONSE_TOPIC, CORRELATION_DATA, REASON_STRING]);
                let invalid_property_set: HashSet<u8> =
                    HashSet::from_iter(vec![RESPONSE_TOPIC, CORRELATION_DATA]);

                assert_eq!(
                    invalid_property_set,
                    invalid_property_for_packet_type(assigned_property, PUBREC, false)
                );
            }

            // pubrel
            #[test]
            fn should_return_invalid_properties_for_packet_type_pubrel_with_will_flag_not_set() {
                let assigned_property: HashSet<u8> =
                    HashSet::from_iter(vec![RESPONSE_TOPIC, CORRELATION_DATA, REASON_STRING]);
                let invalid_property_set: HashSet<u8> =
                    HashSet::from_iter(vec![RESPONSE_TOPIC, CORRELATION_DATA]);

                assert_eq!(
                    invalid_property_set,
                    invalid_property_for_packet_type(assigned_property, PUBREL, false)
                );
            }

            // pubcomp
            #[test]
            fn should_return_invalid_properties_for_packet_type_pubcomp_with_will_flag_not_set() {
                let assigned_property: HashSet<u8> =
                    HashSet::from_iter(vec![RESPONSE_TOPIC, CORRELATION_DATA, REASON_STRING]);
                let invalid_property_set: HashSet<u8> =
                    HashSet::from_iter(vec![RESPONSE_TOPIC, CORRELATION_DATA]);

                assert_eq!(
                    invalid_property_set,
                    invalid_property_for_packet_type(assigned_property, PUBCOMP, false)
                );
            }

            // subscribe
            #[test]
            fn should_return_invalid_properties_for_packet_type_subscribe_with_will_flag_not_set() {
                let assigned_property: HashSet<u8> = HashSet::from_iter(vec![
                    RESPONSE_TOPIC,
                    CORRELATION_DATA,
                    REASON_STRING,
                    SUBSCRIPTION_IDENTIFIER,
                ]);
                let invalid_property_set: HashSet<u8> =
                    HashSet::from_iter(vec![RESPONSE_TOPIC, CORRELATION_DATA, REASON_STRING]);

                assert_eq!(
                    invalid_property_set,
                    invalid_property_for_packet_type(assigned_property, SUBSCRIBE, false)
                );
            }

            // suback
            #[test]
            fn should_return_invalid_properties_for_packet_type_suback_with_will_flag_not_set() {
                let assigned_property: HashSet<u8> = HashSet::from_iter(vec![
                    RESPONSE_TOPIC,
                    CORRELATION_DATA,
                    REASON_STRING,
                    SUBSCRIPTION_IDENTIFIER,
                ]);
                let invalid_property_set: HashSet<u8> = HashSet::from_iter(vec![
                    RESPONSE_TOPIC,
                    CORRELATION_DATA,
                    SUBSCRIPTION_IDENTIFIER,
                ]);

                assert_eq!(
                    invalid_property_set,
                    invalid_property_for_packet_type(assigned_property, SUBACK, false)
                );
            }

            // unsubscribe
            #[test]
            fn should_return_invalid_properties_for_packet_type_unsubscribe_with_will_flag_not_set()
            {
                let assigned_property: HashSet<u8> = HashSet::from_iter(vec![
                    RESPONSE_TOPIC,
                    CORRELATION_DATA,
                    REASON_STRING,
                    SUBSCRIPTION_IDENTIFIER,
                ]);
                let invalid_property_set: HashSet<u8> = HashSet::from_iter(vec![
                    RESPONSE_TOPIC,
                    CORRELATION_DATA,
                    REASON_STRING,
                    SUBSCRIPTION_IDENTIFIER,
                ]);

                assert_eq!(
                    invalid_property_set,
                    invalid_property_for_packet_type(assigned_property, UNSUBSCRIBE, false)
                );
            }
            // unsuback
            #[test]
            fn should_return_invalid_properties_for_packet_type_unsuback_with_will_flag_not_set() {
                let assigned_property: HashSet<u8> = HashSet::from_iter(vec![
                    RESPONSE_TOPIC,
                    CORRELATION_DATA,
                    REASON_STRING,
                    SUBSCRIPTION_IDENTIFIER,
                    USER_PROPERTY,
                ]);
                let invalid_property_set: HashSet<u8> = HashSet::from_iter(vec![
                    RESPONSE_TOPIC,
                    CORRELATION_DATA,
                    SUBSCRIPTION_IDENTIFIER,
                ]);

                assert_eq!(
                    invalid_property_set,
                    invalid_property_for_packet_type(assigned_property, UNSUBACK, false)
                );
            }
            // pingreq
            #[test]
            fn should_return_invalid_properties_for_packet_type_pingreq_with_will_flag_not_set() {
                let assigned_property: HashSet<u8> = HashSet::from_iter(vec![
                    RESPONSE_TOPIC,
                    CORRELATION_DATA,
                    REASON_STRING,
                    SUBSCRIPTION_IDENTIFIER,
                    USER_PROPERTY,
                ]);
                let invalid_property_set: HashSet<u8> = HashSet::from_iter(vec![
                    RESPONSE_TOPIC,
                    CORRELATION_DATA,
                    REASON_STRING,
                    SUBSCRIPTION_IDENTIFIER,
                    USER_PROPERTY,
                ]);

                assert_eq!(
                    invalid_property_set,
                    invalid_property_for_packet_type(assigned_property, PINGREQ, false)
                );
            }

            // disconnect
            #[test]
            fn should_return_invalid_properties_for_packet_type_disconnect_with_will_flag_not_set()
            {
                let assigned_property: HashSet<u8> = HashSet::from_iter(vec![
                    RESPONSE_TOPIC,
                    CORRELATION_DATA,
                    REASON_STRING,
                    SUBSCRIPTION_IDENTIFIER,
                    USER_PROPERTY,
                ]);
                let invalid_property_set: HashSet<u8> = HashSet::from_iter(vec![
                    RESPONSE_TOPIC,
                    CORRELATION_DATA,
                    SUBSCRIPTION_IDENTIFIER,
                ]);

                assert_eq!(
                    invalid_property_set,
                    invalid_property_for_packet_type(assigned_property, DISCONNECT, false)
                );
            }
            // auth
            #[test]
            fn should_return_invalid_properties_for_packet_type_auth_with_will_flag_not_set() {
                let assigned_property: HashSet<u8> = HashSet::from_iter(vec![
                    RESPONSE_TOPIC,
                    CORRELATION_DATA,
                    REASON_STRING,
                    SUBSCRIPTION_IDENTIFIER,
                    USER_PROPERTY,
                ]);
                let invalid_property_set: HashSet<u8> = HashSet::from_iter(vec![
                    RESPONSE_TOPIC,
                    CORRELATION_DATA,
                    SUBSCRIPTION_IDENTIFIER,
                ]);

                assert_eq!(
                    invalid_property_set,
                    invalid_property_for_packet_type(assigned_property, AUTH, false)
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
                        vec![Property::FourByteInteger {
                            value: 1,
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
                b_prop.put_u8(property_identifiers::CONTENT_TYPE); // property identifier, UTF-8 Encoded String

                b_prop.put_u16(b_string.len() as u16);
                b_prop.put(b_string.as_bytes());

                variable_byte_integer(b_prop.len() as u32, &mut b);
                b.put(b_prop);

                if let Ok(p) = decode::property(&mut b) {
                    assert_eq!(
                        vec![Property::UTF8EncodedString {
                            value: String::from("hello world"),
                            property_identifier: property_identifiers::CONTENT_TYPE,
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

                b_prop.put_u8(property_identifiers::CORRELATION_DATA);
                b_prop.put_u16(b_binarydata.len() as u16);
                b_prop.put(b_binarydata.as_slice());
                variable_byte_integer(b_prop.len() as u32, &mut b);
                b.put(b_prop);

                if let Ok(p) = decode::property(&mut b) {
                    assert_eq!(
                        vec![Property::BinaryData {
                            value: vec![1u8, 2, 3, 4, 5],
                            property_identifier: property_identifiers::CORRELATION_DATA,
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

                b_prop.put_u8(property_identifiers::SUBSCRIPTION_IDENTIFIER);
                variable_byte_integer(b_integer, &mut b_prop);

                variable_byte_integer(b_prop.len() as u32, &mut b); // size of property

                b.put(b_prop);

                if let Ok(p) = decode::property(&mut b) {
                    assert_eq!(
                        vec![Property::VariableByteInteger {
                            value: b_integer,
                            property_identifier: property_identifiers::SUBSCRIPTION_IDENTIFIER,
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

                b_prop.put_u8(property_identifiers::SESSION_EXPIRY_INTERVAL);
                b_prop.put_u32(b_integer);
                variable_byte_integer(b_prop.len() as u32, &mut b);
                b.put(b_prop);

                if let Ok(p) = decode::property(&mut b) {
                    assert_eq!(
                        vec![Property::FourByteInteger {
                            value: b_integer,
                            property_identifier: property_identifiers::SESSION_EXPIRY_INTERVAL
                        }],
                        p
                    )
                }
            }

            #[test]
            fn test_property_type_two_integer_using_server_keep_alive() {
                let mut b_prop = BytesMut::with_capacity(2);
                let mut b = BytesMut::with_capacity(100);

                b_prop.put_u8(property_identifiers::SERVER_KEEP_ALIVE);
                b_prop.put_u16(0x1001);

                variable_byte_integer(b_prop.len() as u32, &mut b);
                b.put(b_prop);

                if let Ok(p) = decode::property(&mut b) {
                    assert_eq!(
                        vec![Property::TwoByteInteger {
                            value: 0x1001,
                            property_identifier: property_identifiers::SERVER_KEEP_ALIVE
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

                b_prop.put_u8(property_identifiers::USER_PROPERTY);
                encode::utf8_string_pair("Hello", "World", &mut b_prop);
                variable_byte_integer(b_prop.len() as u32, &mut b);
                b.put(b_prop);

                if let Ok(p) = decode::property(&mut b) {
                    assert_eq!(
                        vec![Property::UTF8StringPair {
                            value: Utf8stringPairT {
                                key: String::from_utf8(key.to_vec()).unwrap(),
                                value: String::from_utf8(value.to_vec()).unwrap(),
                            },
                            property_identifier: property_identifiers::USER_PROPERTY
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
        }
    }

    pub mod utility {
        use crate::mqttbroker::mqtt_broker::packet_types;
        use crate::mqttbroker::mqtt_broker::property_identifiers::{
            ASSIGNED_CLIENT_IDENTIFIER, AUTHENTICATION_DATA, AUTHENTICATION_METHOD, CONTENT_TYPE,
            CORRELATION_DATA, MAXIMUM_PACKET_SIZE, MAXIMUM_QOS, MESSAGE_EXPIRY_INTERVAL,
            PAYLOAD_FORMAT_INDICATOR, REASON_STRING, RECEIVE_MAXIMUM, REQUEST_PROBLEM_INFORMATION,
            REQUEST_RESPONSE_INFORMATION, RESPONSE_INFORMATION, RESPONSE_TOPIC, RETAIN_AVAILABLE,
            SERVER_KEEP_ALIVE, SERVER_REFERENCE, SESSION_EXPIRY_INTERVAL,
            SHARED_SUBSCRIPTION_AVAILABLE, SUBSCRIPTION_IDENTIFIER,
            SUBSCRIPTION_IDENTIFIER_AVAILABLE, TOPIC_ALIAS, TOPIC_ALIAS_MAXIMUM, USER_PROPERTY,
            WILDCARD_SUBSCRIPTION_AVAILABLE, WILL_DELAY_INTERVAL,
        };
        use std::collections::HashSet;
        use std::iter::FromIterator;

        fn concat(mut set: HashSet<u8>, subset: &HashSet<u8>) -> HashSet<u8> {
            for x in subset {
                set.insert(*x);
            }

            set
        }

        /// Will return a list of invalid properties
        ///
        pub fn invalid_property_for_packet_type(
            property: HashSet<u8>,
            packet_type: u8,
            will_flag_set: bool,
        ) -> HashSet<u8> {
            let mut valid_properties: HashSet<u8> = HashSet::from_iter(vec![]);

            if will_flag_set {
                valid_properties.extend(&valid_properties_for_will());
            }

            let property_extension = match packet_type {
                packet_types::CONNECT => valid_properties_for_connect_packet(),

                packet_types::CONNACK => valid_properties_for_connack_packet(),
                packet_types::PUBLISH => valid_properties_for_publish_packet(),
                packet_types::PUBACK => valid_properties_for_puback_packet(),
                packet_types::PUBREC => valid_properties_for_pubrec_packet(),
                packet_types::PUBREL => valid_properties_for_pubrel_packet(),
                packet_types::PUBCOMP => valid_properties_for_pubcomp_packet(),
                packet_types::SUBSCRIBE => valid_properties_for_subscribe_packet(),
                packet_types::SUBACK => valid_properties_for_suback_packet(),
                packet_types::UNSUBSCRIBE => valid_properties_for_unsubscribe_packet(),
                packet_types::UNSUBACK => valid_properties_for_unsuback_packet(),
                packet_types::DISCONNECT => valid_properties_for_disconnect_packet(),
                packet_types::AUTH => valid_properties_for_auth_packet(),
                _ => HashSet::from_iter::<Vec<u8>>(vec![]),
            };

            valid_properties.extend(property_extension);

            let mut diff = HashSet::<u8>::new();
            diff.extend(property.difference(&valid_properties));

            diff
        }

        fn valid_properties_for_will() -> HashSet<u8> {
            let p = HashSet::from_iter(vec![
                PAYLOAD_FORMAT_INDICATOR,
                MESSAGE_EXPIRY_INTERVAL,
                CONTENT_TYPE,
                RESPONSE_TOPIC,
                CORRELATION_DATA,
                WILL_DELAY_INTERVAL,
                USER_PROPERTY,
            ]);
            p
        }

        fn valid_properties_for_connect_packet() -> HashSet<u8> {
            HashSet::from_iter(vec![
                SESSION_EXPIRY_INTERVAL,
                AUTHENTICATION_METHOD,
                AUTHENTICATION_DATA,
                REQUEST_PROBLEM_INFORMATION,
                REQUEST_RESPONSE_INFORMATION,
                RECEIVE_MAXIMUM,
                TOPIC_ALIAS_MAXIMUM,
                USER_PROPERTY,
                MAXIMUM_PACKET_SIZE,
            ])
        }

        fn valid_properties_for_connack_packet() -> HashSet<u8> {
            HashSet::from_iter(vec![
                SESSION_EXPIRY_INTERVAL,
                ASSIGNED_CLIENT_IDENTIFIER,
                SERVER_KEEP_ALIVE,
                AUTHENTICATION_METHOD,
                AUTHENTICATION_DATA,
                RESPONSE_INFORMATION,
                SERVER_REFERENCE,
                REASON_STRING,
                RECEIVE_MAXIMUM,
                TOPIC_ALIAS_MAXIMUM,
                MAXIMUM_QOS,
                RETAIN_AVAILABLE,
                USER_PROPERTY,
                MAXIMUM_PACKET_SIZE,
                WILDCARD_SUBSCRIPTION_AVAILABLE,
                SUBSCRIPTION_IDENTIFIER_AVAILABLE,
                SHARED_SUBSCRIPTION_AVAILABLE,
            ])
        }

        fn valid_properties_for_publish_packet() -> HashSet<u8> {
            HashSet::from_iter(vec![
                PAYLOAD_FORMAT_INDICATOR,
                MESSAGE_EXPIRY_INTERVAL,
                CONTENT_TYPE,
                RESPONSE_TOPIC,
                CORRELATION_DATA,
                SUBSCRIPTION_IDENTIFIER,
                TOPIC_ALIAS,
                USER_PROPERTY,
            ])
        }

        fn valid_properties_for_puback_packet() -> HashSet<u8> {
            HashSet::from_iter(vec![REASON_STRING, USER_PROPERTY])
        }

        fn valid_properties_for_pubrec_packet() -> HashSet<u8> {
            HashSet::from_iter(vec![REASON_STRING, USER_PROPERTY])
        }

        fn valid_properties_for_pubrel_packet() -> HashSet<u8> {
            HashSet::from_iter(vec![REASON_STRING, USER_PROPERTY])
        }

        fn valid_properties_for_pubcomp_packet() -> HashSet<u8> {
            HashSet::from_iter(vec![REASON_STRING, USER_PROPERTY])
        }

        fn valid_properties_for_subscribe_packet() -> HashSet<u8> {
            HashSet::from_iter(vec![SUBSCRIPTION_IDENTIFIER, USER_PROPERTY])
        }

        fn valid_properties_for_suback_packet() -> HashSet<u8> {
            HashSet::from_iter(vec![REASON_STRING, USER_PROPERTY])
        }

        fn valid_properties_for_unsubscribe_packet() -> HashSet<u8> {
            HashSet::from_iter(vec![USER_PROPERTY])
        }

        fn valid_properties_for_unsuback_packet() -> HashSet<u8> {
            HashSet::from_iter(vec![REASON_STRING, USER_PROPERTY])
        }

        fn valid_properties_for_disconnect_packet() -> HashSet<u8> {
            HashSet::from_iter(vec![
                SESSION_EXPIRY_INTERVAL,
                SERVER_REFERENCE,
                REASON_STRING,
                USER_PROPERTY,
            ])
        }

        fn valid_properties_for_auth_packet() -> HashSet<u8> {
            HashSet::from_iter(vec![
                AUTHENTICATION_METHOD,
                AUTHENTICATION_DATA,
                REASON_STRING,
                USER_PROPERTY,
            ])
        }

        fn packet_identifier_present(mqtt_control_packet: u8, qos: u8) -> bool {
            match mqtt_control_packet {
                packet_types::CONNECT
                | packet_types::CONNACK
                | packet_types::PINGREQ
                | packet_types::PINGRESP
                | packet_types::DISCONNECT
                | packet_types::AUTH => false,
                packet_types::PUBACK
                | packet_types::PUBREC
                | packet_types::PUBREL
                | packet_types::PUBCOMP
                | packet_types::SUBSCRIBE
                | packet_types::SUBACK
                | packet_types::UNSUBSCRIBE
                | packet_types::UNSUBACK => true,
                packet_types::PUBLISH => (qos > 0),
                _ => false,
            }
        }
    }

    pub enum ReasonCode {
        Success,
        NormalDisconnection,
        GrantedQoS0,
        GrantedQos1,
        GrantedQos2,
        DisconnectWithWillMessage,
        NoMatchingSubscribers,
        NoSubscriptionExisted,
        ContinueAuthentication,
        Reauthenticate,
        UnSpecifiedError,
        MalformedPacket,
        ProtocolError,
        ImplementationSpecificError,
        UnsupportedProtocolVersion,
        ClientIdentifierNotValid,
        BadUserNameOrPassword,
        NotAuthorised,
        ServerUnavailable,
        ServerBusy,
        Banned,
        SererShuttingDown,
        BadAuthentication,
        KeepAliveTimeout,
        SessionTakenOver,
        TopicFilterInvalid,
        TopicNameInvalid,
        PacketIdentifierInUse,
        PacketIdentifierNotFound,
        ReceiveMaximumExceeded,
        TopicAliasInvalid,
        PacketTooLarge,
        MessageRateTooHigh,
        QuotaExceeded,
        AdministrativeAction,
        PayloadFormatInvalid,
        RetainNotSupported,
        QoSNotSupported,
        UseAnotherServer,
        ServerMoved,
        SharedSubscriptionsNotSupported,
        ConnectionRateExceeded,
        MaximumConnectTime,
        SubscriptionIdentifiersNotSupported,
        WildcardSubscriptionsNotSupported,
    }

    mod packets {
        use crate::mqttbroker::mqtt_broker::{Property, ReasonCode};
        use std::collections::HashSet;

        pub struct Connect {
            // fixed header
            packet_type: u8,

            // variable header
            protocol_name: String,
            protocol_version: u8,
            connect_flags: u8,
            keep_alive: u16,
            property: HashSet<Property>,

            //payload
            client_id: String,
            will_property: Option<Vec<Property>>,
            will_topic: Option<String>,
            will_payload: Vec<u8>,
            username: Option<String>,
            password: Option<String>,
        }

        pub struct ConnAck {
            //fixed header
            packet_type: u8,

            // variable header
            connect_ack_flags: u8,
            reason_code: ReasonCode,
            property: HashSet<Property>,
            //payload
            // no payload
        }

        pub struct Publish {
            //fixed header
            packet_type: u8,

            //variable_header
            topic_name: String,
            packet_id: u16,
            property: HashSet<Property>,

            //payload
            application_message: Vec<u8>,
        }

        pub struct PubAck {
            //fixed header
            packet_type: u8,

            //variable header
            packet_id: u16,
            reason_code: Option<ReasonCode>, // not available if remaining length in fixed header
            // is 2, which means there is only a packet_id in variable header. If there is no Reason code then 0x00(Success) used by the client.
            property: Option<HashSet<Property>>,
            //payload
            //no payload
        }

        pub struct PubRec {
            //fixed header
            packet_type: u8,

            //variable header
            packet_id: u16,
            reason_code: ReasonCode,
            property: HashSet<Property>, // if the remaining length is 4 then property length is zero

                                         //payload
                                         //no payload
        }

        pub struct PubRel {
            //fixed header
            packet_type: u8,

            //variable header
            packet_id: u16,
            reason_code: ReasonCode,
            property: HashSet<Property>, // if the remaining length is 4 then property length is zero

                                         //payload
                                         //no payload
        }

        pub struct PubComp {
            //fixed header
            packet_type: u8,

            //variable header
            packet_id: u16,
            reason_code: ReasonCode,
            property: HashSet<Property>, // if the remaining length is 4 then property length is zero

                                         //payload
                                         //no payload
        }

        pub struct Subscribe {
            //fixed header
            packet_type: u8,

            //variable header
            packet_id: u16,
            property: HashSet<Property>,

            //payload
            topic_filter: Vec<(String, u8)>,
        }

        pub struct SubAck {
            //fixed header
            packet_type: u8,

            //variable header
            packet_id: u16,
            property: HashSet<Property>,

            //payload
            reason_code: Vec<ReasonCode>,
        }
    }

    mod server {

        #[cfg(test)]
        mod test {

            #[test]
            fn test_connect() {}
        }
    }

    // async fn process<T>(socket: T) {
    //     loop {}
    // }
    //
    // struct MqttBroker<'a> {
    //     addr: &'a str,
    // }
    //
    // impl MqttBroker<'_> {
    //     async fn run(self) -> Result<(), Error> {
    //         let mut listener = TcpListener::bind(self.addr).await.unwrap();
    //
    //         loop {
    //             let (mut socket, _) = listener.accept().await?;
    //             tokio::spawn(async move {
    //                 // do something here
    //                 process(socket).await;
    //             });
    //         }
    //     }
    // }
}
