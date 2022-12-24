use crate::decode::DecodeError::UTF8Errors;
use crate::mqttbroker::packets::PacketTypes;
use bytes::{Buf, BytesMut};
use std::collections::HashMap;
use std::convert::Into;
use thiserror::Error;
use tracing::debug;

lazy_static! {
    static ref PROPERTYNAME: HashMap<u8, &'static str> = {
        let mut h = HashMap::new();
        h.insert(
            Property::PayloadFormatIndicator as u8,
            "Payload Format Indicator",
        );
        h.insert(
            Property::MessageExpiryInterval as u8,
            "Message Expiry Interval",
        );
        h.insert(Property::ContentType as u8, "Content Type");
        h.insert(Property::ResponseTopic as u8, "Response Topic");
        h.insert(Property::CorrelationData as u8, "Correlation Data");
        h.insert(
            Property::SubscriptionIdentifier as u8,
            "Subscription Identifier",
        );
        h.insert(
            Property::SessionExpiryInterval as u8,
            "Session Expiry Interval",
        );
        h.insert(
            Property::AssignedClientIdentifier as u8,
            "Assigned Client Identifier",
        );
        h.insert(Property::ServerKeepAlive as u8, "Server Keep Alive");
        h.insert(
            Property::AuthenticationMethod as u8,
            "Authentication Method",
        );
        h.insert(Property::AuthenticationData as u8, "Authentication Data");
        h.insert(
            Property::RequestProblemInformation as u8,
            "Request Problem Information",
        );
        h.insert(Property::WillDelayInterval as u8, "Will Delay Interval");
        h.insert(
            Property::RequestResponseInformation as u8,
            "Request Response Information",
        );
        h.insert(Property::ResponseInformation as u8, "Response Information");
        h.insert(Property::ServerReference as u8, "Server Reference");
        h.insert(Property::ReasonString as u8, "Reason String");
        h.insert(Property::ReceiveMaximum as u8, "Receive Maximum");
        h.insert(Property::TopicAliasMaximum as u8, "Topic Alias Maximum");
        h.insert(Property::TopicAlias as u8, "Topic Alias");
        h.insert(Property::MaximumQos as u8, "Maximum QoS");
        h.insert(Property::RetainAvailable as u8, "Retain Available");
        h.insert(Property::User as u8, "User Property");
        h.insert(Property::MaximumPacketSize as u8, "Maximum Packet Size");
        h.insert(
            Property::WildcardSubscriptionAvailable as u8,
            "Wildcard Subscription Available",
        );
        h.insert(
            Property::SubscriptionIdentifierAvailable as u8,
            "Subscription Identifier Available",
        );
        h.insert(
            Property::SharedSubscriptionAvailable as u8,
            "Shared Subscription Available",
        );
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
    static ref PAYLOADREQUIREDSTATUS: HashMap<PacketTypes, PayLoad> = {
        let mut h: HashMap<PacketTypes, PayLoad> = HashMap::new();
        h.insert(PacketTypes::Connect, PayLoad::Required);
        h.insert(PacketTypes::Connack, PayLoad::None);
        h.insert(PacketTypes::Publish, PayLoad::Optional);
        h.insert(PacketTypes::Puback, PayLoad::None);
        h.insert(PacketTypes::Pubrec, PayLoad::None);
        h.insert(PacketTypes::Pubrel, PayLoad::None);
        h.insert(PacketTypes::Pubcomp, PayLoad::None);
        h.insert(PacketTypes::Subscribe, PayLoad::Required);
        h.insert(PacketTypes::Suback, PayLoad::Required);
        h.insert(PacketTypes::Unsubscribe, PayLoad::Required);
        h.insert(PacketTypes::Unsuback, PayLoad::Required);
        h.insert(PacketTypes::Pingreq, PayLoad::None);
        h.insert(PacketTypes::Pingresp, PayLoad::None);
        h.insert(PacketTypes::Disconnect, PayLoad::None);
        h.insert(PacketTypes::Auth, PayLoad::None);
        h
    };
    static ref VALID_WILL_PROPERTY_CODES: [PropertyIdentifier; 1] = [PropertyIdentifier::new(
        PropertyIdentifierConstant::WillDelayInterval,
    )];
}

// Property::PayloadFormatIndicator,
// Property::MessageExpiryInterval,
// Property::ContentType,
// Property::ResponseTopic,
// Property::CorrelationData,

lazy_static! {
    static ref VALIDPROPERTYCODES: HashMap<PacketTypes, Vec<PropertyIdentifier>> = {
        let mut vpc: HashMap<PacketTypes, Vec<PropertyIdentifier>> = HashMap::new();

        vpc.insert(
            PacketTypes::Connect,
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
            ],
        );

        vpc.insert(
            PacketTypes::Connack,
            vec![
                PropertyIdentifier::new(PropertyIdentifierConstant::SessionExpiryInterval),
                PropertyIdentifier::new(PropertyIdentifierConstant::AssignedClientIdentifier),
                PropertyIdentifier::new(PropertyIdentifierConstant::ServerKeepAlive),
                PropertyIdentifier::new(PropertyIdentifierConstant::AuthenticationMethod),
                PropertyIdentifier::new(PropertyIdentifierConstant::AuthenticationData),
                PropertyIdentifier::new(PropertyIdentifierConstant::ResponseInformation),
                PropertyIdentifier::new(PropertyIdentifierConstant::ServerReference),
                PropertyIdentifier::new(PropertyIdentifierConstant::ReasonString),
                PropertyIdentifier::new(PropertyIdentifierConstant::ReceiveMaximum),
                PropertyIdentifier::new(PropertyIdentifierConstant::TopicAliasMaximum),
                PropertyIdentifier::new(PropertyIdentifierConstant::MaximumQos),
                PropertyIdentifier::new(PropertyIdentifierConstant::RetainAvailable),
                PropertyIdentifier::new(PropertyIdentifierConstant::User),
                PropertyIdentifier::new(PropertyIdentifierConstant::MaximumPacketSize),
                PropertyIdentifier::new(PropertyIdentifierConstant::WildcardSubscriptionAvailable),
                PropertyIdentifier::new(
                    PropertyIdentifierConstant::SubscriptionIdentifierAvailable,
                ),
                PropertyIdentifier::new(PropertyIdentifierConstant::SharedSubscriptionAvailable),
            ],
        );

        vpc.insert(
            PacketTypes::Publish,
            vec![
                PropertyIdentifier::new(PropertyIdentifierConstant::PayloadFormatIndicator),
                PropertyIdentifier::new(PropertyIdentifierConstant::MessageExpiryInterval),
                PropertyIdentifier::new(PropertyIdentifierConstant::ContentType),
                PropertyIdentifier::new(PropertyIdentifierConstant::ResponseTopic),
                PropertyIdentifier::new(PropertyIdentifierConstant::CorrelationData),
                PropertyIdentifier::new(PropertyIdentifierConstant::SubscriptionIdentifier),
                PropertyIdentifier::new(PropertyIdentifierConstant::TopicAlias),
                PropertyIdentifier::new(PropertyIdentifierConstant::User),
            ],
        );

        vpc.insert(
            PacketTypes::Puback,
            vec![
                PropertyIdentifier::new(PropertyIdentifierConstant::ReasonString),
                PropertyIdentifier::new(PropertyIdentifierConstant::User),
            ],
        );

        vpc.insert(
            PacketTypes::Pubrec,
            vec![
                PropertyIdentifier::new(PropertyIdentifierConstant::ReasonString),
                PropertyIdentifier::new(PropertyIdentifierConstant::User),
            ],
        );

        vpc.insert(
            PacketTypes::Pubrel,
            vec![
                PropertyIdentifier::new(PropertyIdentifierConstant::ReasonString),
                PropertyIdentifier::new(PropertyIdentifierConstant::User),
            ],
        );

        vpc.insert(
            PacketTypes::Pubcomp,
            vec![
                PropertyIdentifier::new(PropertyIdentifierConstant::ReasonString),
                PropertyIdentifier::new(PropertyIdentifierConstant::User),
            ],
        );

        vpc.insert(
            PacketTypes::Subscribe,
            vec![
                PropertyIdentifier::new(PropertyIdentifierConstant::SubscriptionIdentifier),
                PropertyIdentifier::new(PropertyIdentifierConstant::User),
            ],
        );

        vpc.insert(
            PacketTypes::Suback,
            vec![
                PropertyIdentifier::new(PropertyIdentifierConstant::ReasonString),
                PropertyIdentifier::new(PropertyIdentifierConstant::User),
            ],
        );

        vpc.insert(
            PacketTypes::Unsubscribe,
            vec![PropertyIdentifier::new(PropertyIdentifierConstant::User)],
        );

        vpc.insert(
            PacketTypes::Unsuback,
            vec![
                PropertyIdentifier::new(PropertyIdentifierConstant::ReasonString),
                PropertyIdentifier::new(PropertyIdentifierConstant::User),
            ],
        );

        vpc.insert(
            PacketTypes::Disconnect,
            vec![
                PropertyIdentifier::new(PropertyIdentifierConstant::SessionExpiryInterval),
                PropertyIdentifier::new(PropertyIdentifierConstant::ServerReference),
                PropertyIdentifier::new(PropertyIdentifierConstant::ReasonString),
                PropertyIdentifier::new(PropertyIdentifierConstant::User),
            ],
        );

        vpc.insert(
            PacketTypes::Auth,
            vec![
                PropertyIdentifier::new(PropertyIdentifierConstant::AuthenticationMethod),
                PropertyIdentifier::new(PropertyIdentifierConstant::AuthenticationData),
                PropertyIdentifier::new(PropertyIdentifierConstant::ReasonString),
                PropertyIdentifier::new(PropertyIdentifierConstant::User),
            ],
        );

        vpc
    };
}
use crate::mqttbroker::primitive_types::{
    BinaryData, Byte, FourByteInteger, TwoByteInteger, Utf8EncodedString, Utf8StringPair,
    VariableByteInteger,
};
use crate::mqttbroker::properties::PropertyIdentifier;
use crate::mqttbroker::properties::{Property, PropertyIdentifierConstant};

#[derive(Error, Debug, PartialEq, Eq)]
pub enum DecodeError {
    #[error("Not enough bytes decoding {0}")]
    NotEnoughBytes(String),
    #[error("The variable int does not have the MSB clear on the fourth byte.")]
    NotValidVarInt,
    #[error("Not enough bytes found for decoding {2}. Require {0} bytes, found {1} bytes.")]
    MoreBytesRequired(u16, u16, String),
    #[error("Converting bytes to utf-8 string for {0}")]
    UTF8Errors(String),
    #[error("Invalid property identifier. Value is {0}")]
    UnknownProperty(u8),
}

pub fn decode_two_byte_integer(
    name: String,
    b: &mut BytesMut,
) -> anyhow::Result<TwoByteInteger, DecodeError> {
    if b.len() < 2 {
        Err(DecodeError::MoreBytesRequired(2, b.len() as u16, name))
    } else {
        Ok(TwoByteInteger::new(b.get_u16()))
    }
}

pub fn decode_four_byte_integer(
    name: String,
    b: &mut BytesMut,
) -> anyhow::Result<FourByteInteger, DecodeError> {
    if b.len() < 4 {
        Err(DecodeError::MoreBytesRequired(4, b.len() as u16, name))
    } else {
        Ok(FourByteInteger::new(b.get_u32()))
    }
}

pub fn decode_utf8_string(name: String, b: &mut BytesMut) -> anyhow::Result<String, DecodeError> {
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

pub fn decode_binary(name: String, b: &mut BytesMut) -> anyhow::Result<BinaryData, DecodeError> {
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

            Ok(BinaryData::new(binary.to_vec()))
        }
    }
}

fn decode_utf8_string_pair(
    name: String,
    b: &mut BytesMut,
) -> anyhow::Result<Utf8StringPair, DecodeError> {
    let mut key: String = String::from("empty");
    let mut value: String = String::from("empty");
    let name_of_key = format!("key of {0}", name);
    let name_of_value = format!("value of {0}", name);
    match decode_utf8_string(name_of_key, b) {
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
    match decode_utf8_string(name_of_value, b) {
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

    // Ok(Utf8stringPair { key, value })
    Ok(Utf8StringPair(key, value))
}

pub fn decode_varint(b: &mut BytesMut) -> anyhow::Result<VariableByteInteger, DecodeError> {
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
    Ok(VariableByteInteger::new(value))
}

// pub fn decode_property(b: &mut BytesMut) -> anyhow::Result<Vec<Property>, DecodeError> {
//     debug!("pre varint length is {}", b.len());
//     let length = decode_varint(b)?;
//     debug!("post varint length is {}", b.len());
//     let mut sub_b = b.split_to(*length.as_ref() as usize);
//     debug!("post sub_b is {}", sub_b.len());
//
//     let mut p_vec: Vec<Property> = vec![];
//     debug!("property length {}", length.as_ref());
//
//     while !sub_b.is_empty() {
//         let property_identifier = sub_b.get_u8();
//         let p = match property_identifier {
//             prop if [
//                 Property::PayloadFormatIndicator as u8,
//                 Property::RequestProblemInformation as u8,
//                 Property::RequestResponseInformation as u8,
//                 Property::MaximumQos as u8,
//                 Property::RetainAvailable as u8,
//                 Property::WildcardSubscriptionAvailable as u8,
//                 Property::SharedSubscriptionAvailable as u8,
//             ]
//             .contains(&prop) =>
//             {
//                 Property {
//                     element_value: PropertyType::Byte {
//                         value: Byte::new(sub_b.get_u8()),
//                     },
//                     property_identifier,
//                 }
//             }
//             prop if [
//                 Property::MessageExpiryInterval as u8,
//                 Property::SessionExpiryInterval as u8,
//                 Property::WillDelayInterval as u8,
//                 Property::MaximumPacketSize as u8,
//             ]
//             .contains(&prop) =>
//             {
//                 let four_byte_integer = decode_four_byte_integer(
//                     String::from(*PROPERTYNAME.get(&property_identifier).to_owned().unwrap()),
//                     &mut sub_b,
//                 )?;
//                 Property {
//                     element_value: PropertyType::FourByteInteger {
//                         value: four_byte_integer,
//                     },
//                     property_identifier,
//                 }
//             }
//             prop if [
//                 Property::ContentType as u8,
//                 Property::ResponseTopic as u8,
//                 Property::AssignedClientIdentifier as u8,
//                 Property::AuthenticationMethod as u8,
//                 Property::ResponseInformation as u8,
//                 Property::ServerReference as u8,
//                 Property::ReasonString as u8,
//             ]
//             .contains(&prop) =>
//             {
//                 let str = decode_utf8_string(
//                     String::from(*PROPERTYNAME.get(&property_identifier).to_owned().unwrap()),
//                     &mut sub_b,
//                 )?;
//
//                 Property {
//                     element_value: PropertyType::UTF8EncodedString {
//                         value: Utf8EncodedString::new(str),
//                     },
//                     property_identifier,
//                 }
//             }
//             prop if [
//                 Property::CorrelationData as u8,
//                 Property::AuthenticationData as u8,
//             ]
//             .contains(&prop) =>
//             {
//                 let binary_data = decode_binary(
//                     String::from(*PROPERTYNAME.get(&property_identifier).to_owned().unwrap()),
//                     &mut sub_b,
//                 )?;
//
//                 Property {
//                     element_value: PropertyType::BinaryData { value: binary_data },
//                     property_identifier,
//                 }
//             }
//             prop if [
//                 Property::ServerKeepAlive as u8,
//                 Property::ReceiveMaximum as u8,
//                 Property::TopicAliasMaximum as u8,
//                 TopicAlias as u8,
//             ]
//             .contains(&prop) =>
//             {
//                 let two_byte_integer = decode_two_byte_integer(
//                     String::from(*PROPERTYNAME.get(&property_identifier).to_owned().unwrap()),
//                     &mut sub_b,
//                 )?;
//
//                 Property {
//                     element_value: PropertyType::TwoByteInteger {
//                         value: two_byte_integer,
//                     },
//                     property_identifier,
//                 }
//             }
//             prop if [Property::UserProperty as u8].contains(&prop) => {
//                 let utf8_string_pair = decode_utf8_string_pair(
//                     String::from(*PROPERTYNAME.get(&property_identifier).to_owned().unwrap()),
//                     &mut sub_b,
//                 )?;
//                 Property {
//                     element_value: PropertyType::UTF8StringPair {
//                         value: utf8_string_pair,
//                     },
//                     property_identifier,
//                 }
//             }
//             prop if [Property::SubscriptionIdentifier as u8].contains(&prop) => {
//                 debug!("pre variable_byte_integer sub_p len is {}", sub_b.len());
//                 let variable_byte_integer = decode_varint(&mut sub_b)?;
//                 debug!("post variable_byte_integer sub_p len is {}", sub_b.len());
//                 Property {
//                     element_value: PropertyType::VariableByteInteger {
//                         value: variable_byte_integer,
//                     },
//                     property_identifier,
//                 }
//             }
//
//             _ => {
//                 //FIXME should return a malformed packets error
//                 panic!() // should return a malformed packets Error
//             }
//         };
//         p_vec.push(p);
//     }
//     Ok(p_vec)
// }
pub fn decode_property(b: &mut BytesMut) -> anyhow::Result<Vec<Property>, DecodeError> {
    println!("decode property is {:?}", b);
    println!("pre varint length is {}", b.len());
    let length = decode_varint(b)?;
    println!("post varint length is {}", b.len());
    let mut sub_b = b.split_to(*length.as_ref() as usize);
    println!("post sub_b is {:?}", sub_b);

    let mut p_vec: Vec<Property> = vec![];
    println!("property length {}", length.as_ref());

    while !sub_b.is_empty() {
        let property_identifier = sub_b.get_u8();

        let p = match property_identifier {
            prop if Property::PayloadFormatIndicator as u8 == prop => {
                Property::PayloadFormatIndicator(Byte(sub_b.get_u8()))
            }

            prop if Property::MessageExpiryInterval as u8 == prop => {
                Property::MessageExpiryInterval(FourByteInteger(sub_b.get_u32()))
            }

            prop if Property::ContentType as u8 == prop => {
                let str = decode_utf8_string(
                    String::from(*PROPERTYNAME.get(&property_identifier).to_owned().unwrap()),
                    &mut sub_b,
                )?;
                Property::ContentType(Utf8EncodedString(str))
            }

            prop if Property::ResponseTopic as u8 == prop => {
                let str = decode_utf8_string(
                    String::from(*PROPERTYNAME.get(&property_identifier).to_owned().unwrap()),
                    &mut sub_b,
                )?;
                Property::ContentType(Utf8EncodedString(str))
            }

            prop if Property::CorrelationData as u8 == prop => {
                let binary_data = decode_binary(
                    String::from(*PROPERTYNAME.get(&property_identifier).to_owned().unwrap()),
                    &mut sub_b,
                )?;

                Property::CorrelationData(binary_data)
            }

            prop if Property::SubscriptionIdentifier as u8 == prop => {
                let vint = decode_varint(&mut sub_b)?;
                Property::SubscriptionIdentifier(VariableByteInteger(vint.0))
            }

            prop if Property::SessionExpiryInterval as u8 == prop => {
                Property::SessionExpiryInterval(FourByteInteger(sub_b.get_u32()))
            }

            prop if Property::AssignedClientIdentifier as u8 == prop => {
                let str = decode_utf8_string(
                    String::from(*PROPERTYNAME.get(&property_identifier).to_owned().unwrap()),
                    &mut sub_b,
                )?;

                Property::AssignedClientIdentifier(Utf8EncodedString(str))
            }

            prop if Property::ServerKeepAlive as u8 == prop => {
                Property::ServerKeepAlive(TwoByteInteger(sub_b.get_u16()))
            }

            prop if Property::AuthenticationMethod as u8 == prop => {
                let str = decode_utf8_string(
                    String::from(*PROPERTYNAME.get(&property_identifier).to_owned().unwrap()),
                    &mut sub_b,
                )?;
                Property::AuthenticationMethod(Utf8EncodedString(str))
            }

            prop if Property::AuthenticationData as u8 == prop => {
                let binary_data = decode_binary(
                    String::from(*PROPERTYNAME.get(&property_identifier).to_owned().unwrap()),
                    &mut sub_b,
                )?;

                Property::AuthenticationData(binary_data)
            }

            prop if Property::RequestProblemInformation as u8 == prop => {
                Property::RequestProblemInformation(Byte(sub_b.get_u8()))
            }

            prop if Property::WillDelayInterval as u8 == prop => {
                Property::WillDelayInterval(FourByteInteger(sub_b.get_u32()))
            }

            prop if Property::RequestResponseInformation as u8 == prop => {
                Property::RequestResponseInformation(Byte(sub_b.get_u8()))
            }

            prop if Property::ResponseInformation as u8 == prop => {
                let str = decode_utf8_string(
                    String::from(*PROPERTYNAME.get(&property_identifier).to_owned().unwrap()),
                    &mut sub_b,
                )?;
                Property::ResponseInformation(Utf8EncodedString(str))
            }

            prop if Property::ServerReference as u8 == prop => {
                let str = decode_utf8_string(
                    String::from(*PROPERTYNAME.get(&property_identifier).to_owned().unwrap()),
                    &mut sub_b,
                )?;
                Property::ServerReference(Utf8EncodedString(str))
            }

            prop if Property::ReasonString as u8 == prop => {
                let str = decode_utf8_string(
                    String::from(*PROPERTYNAME.get(&property_identifier).to_owned().unwrap()),
                    &mut sub_b,
                )?;
                Property::ReasonString(Utf8EncodedString(str))
            }

            prop if Property::ReceiveMaximum as u8 == prop => {
                Property::ReceiveMaximum(TwoByteInteger(sub_b.get_u16()))
            }

            prop if Property::TopicAliasMaximum as u8 == prop => {
                Property::TopicAliasMaximum(TwoByteInteger(sub_b.get_u16()))
            }

            prop if Property::TopicAlias as u8 == prop => {
                Property::TopicAlias(TwoByteInteger(sub_b.get_u16()))
            }

            prop if Property::MaximumQos as u8 == prop => {
                Property::MaximumQos(Byte(sub_b.get_u8()))
            }

            prop if Property::RetainAvailable as u8 == prop => {
                Property::RetainAvailable(Byte(sub_b.get_u8()))
            }

            prop if Property::User as u8 == prop => {
                let utf8_string_pair = decode_utf8_string_pair(
                    String::from(*PROPERTYNAME.get(&property_identifier).to_owned().unwrap()),
                    &mut sub_b,
                )?;
                Property::User(utf8_string_pair)
            }

            prop if Property::MaximumPacketSize as u8 == prop => {
                Property::MaximumPacketSize(FourByteInteger(sub_b.get_u32()))
            }

            prop if Property::WildcardSubscriptionAvailable as u8 == prop => {
                Property::WildcardSubscriptionAvailable(Byte(sub_b.get_u8()))
            }

            prop if Property::SubscriptionIdentifierAvailable as u8 == prop => {
                Property::SubscriptionIdentifierAvailable(Byte(sub_b.get_u8()))
            }

            prop if Property::SharedSubscriptionAvailable as u8 == prop => {
                Property::SharedSubscriptionAvailable(Byte(sub_b.get_u8()))
            }
            _ => return Err(DecodeError::UnknownProperty(property_identifier)),
        };

        p_vec.push(p);
    }

    Ok(p_vec)
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    // use crate::mqttbroker::mqtt_broker::primitive_types::{
    //     BinaryData, Byte, FourByteInteger, TwoByteInteger, Utf8EncodedString, Utf8StringPair,
    //     VariableByteInteger,
    // };
    use crate::mqttbroker::packets::PacketTypes;

    use crate::decode::{decode_binary, decode_utf8_string_pair, decode_varint, DecodeError};
    use crate::encode::{
        encode_binary_data, encode_utf8_encoded_string, encode_variable_byte_integer, EncodeError,
    };

    use crate::mqttbroker::packets::properties::PropertyContainer::UTF8EncodeString;
    use crate::mqttbroker::packets::reason_codes::DISCONNECT::ProtocolError;
    use crate::mqttbroker::primitive_types::{
        BinaryData, Byte, FourByteInteger, TwoByteInteger, Utf8EncodedString, Utf8StringPair,
        VariableByteInteger,
    };
    use crate::mqttbroker::properties::{
        invalid_property_for_connect_packet_type, invalid_property_for_non_connect_packet_type,
        non_unique_properties, Property, PropertyIdentifier, PropertyIdentifierConstant,
    };
    use crate::{decode, encode};
    use bytes::BufMut;
    use bytes::BytesMut;
    use test_log::test;
    use tracing::{debug, event, info, span, trace, Level};

    fn payload_format_indicator(val: u8, buf: &mut BytesMut) {
        buf.put_u8(Property::PayloadFormatIndicator as u8);
        buf.put_u8(val);
    }

    fn message_expiry_interval(value: u32, buf: &mut BytesMut) {
        buf.put_u8(Property::MessageExpiryInterval as u8);
        buf.put_u32(value);
    }

    fn content_type(value: &str, buf: &mut BytesMut) {
        buf.put_u8(Property::ContentType as u8);
        encode_utf8_encoded_string(value, buf);
    }

    fn correlation_data(value: &BytesMut, buf: &mut BytesMut) {
        buf.put_u8(Property::CorrelationData as u8);
        encode_binary_data(value, buf);
    }

    fn subscription_identifier(value: u32, buf: &mut BytesMut) -> Result<(), EncodeError> {
        buf.put_u8(Property::SubscriptionIdentifier as u8);
        encode_variable_byte_integer(&VariableByteInteger::new(value), buf)
    }

    fn session_expiry_interval(value: u32, buf: &mut BytesMut) {
        buf.put_u8(Property::SessionExpiryInterval as u8);
        buf.put_u32(value);
    }

    fn assigned_client_identifier(value: &str, buf: &mut BytesMut) {
        buf.put_u8(Property::AssignedClientIdentifier as u8);
        encode_utf8_encoded_string(value, buf);
    }

    fn user_property(key: &str, value: &str, buf: &mut BytesMut) {
        buf.put_u8(Property::User as u8);
        encode::utf8_string_pair(key, value, buf);
    }

    #[test]
    fn test_2_byte_integer() {
        let b = &mut BytesMut::with_capacity(2);
        b.put_u16(257);
        assert_eq!(
            Ok(TwoByteInteger::new(257)),
            decode::decode_two_byte_integer(String::from("name"), b)
        );
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
            decode::decode_two_byte_integer(name, b)
        );
    }

    #[test]
    fn test_4_byte_integer() {
        let b = &mut BytesMut::with_capacity(4);
        b.put_u32(257);
        assert_eq!(
            Ok(FourByteInteger::new(257)),
            decode::decode_four_byte_integer(String::from("name"), b)
        );
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
            decode::decode_four_byte_integer(name, b)
        );
    }

    #[test]
    fn test_utf8_string() {
        let str = b"hello world";
        let b = &mut BytesMut::with_capacity(str.len() + 2);
        b.put_u16(str.len() as u16);
        b.put_slice(str);
        let name = String::from("name");
        assert_eq!("hello world", decode::decode_utf8_string(name, b).unwrap());
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
        let decoded = decode::decode_utf8_string(name, b).unwrap();
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
            decode::decode_utf8_string(name, b)
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
            Err(DecodeError::UTF8Errors(name.clone())),
            decode::decode_utf8_string(name, b)
        )
    }

    #[test]
    fn test_binary() {
        let b = &mut BytesMut::with_capacity(8);
        let name = String::from("name");
        let binary = vec![0, 1, 2, 3, 4, 5];
        b.put_u16(binary.len() as u16);
        b.put_slice(binary.as_slice());
        assert_eq!(BinaryData::new(binary), decode_binary(name, b).unwrap());
    }

    #[test]
    fn test_binary_with_extra_byte() {
        let b = &mut BytesMut::with_capacity(8);
        let name = String::from("name");
        let binary = vec![0, 1, 2, 3, 4, 5];
        b.put_u16(binary.len() as u16);
        b.put_slice(binary.as_slice());
        b.put_u8(0); // dummy byte value
        assert_eq!(&binary, decode_binary(name, b).unwrap().as_ref());
        assert_eq!(b.len(), 1);
    }

    #[test]
    fn test_binary_not_enough_bytes() {
        let b = &mut BytesMut::with_capacity(1);
        b.put_u8(1);
        let name = String::from("name");
        assert_eq!(
            Err(DecodeError::NotEnoughBytes(name.clone())),
            decode_binary(name, b)
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

        let string_pair = Utf8StringPair(String::from("key"), String::from("value"));

        assert_eq!(
            string_pair,
            decode_utf8_string_pair(String::from("property name"), b).unwrap()
        );
    }

    #[test]
    fn varint_1_test() {
        let mut b = BytesMut::with_capacity(1);
        b.put_u8(1);

        if let Ok(i) = decode_varint(&mut b) {
            assert_eq!(*i.as_ref(), 1);
        }
    }

    #[test]
    fn varint_127_test() {
        let mut b = BytesMut::with_capacity(1);
        b.put_u8(127);

        if let Ok(i) = decode_varint(&mut b) {
            assert_eq!(*i.as_ref(), 127);
        }
    }

    #[test]
    fn varint_127_test_with_extra_byte() {
        let mut b = BytesMut::with_capacity(1);
        b.put_u8(127);
        b.put_u8(0);

        if let Ok(i) = decode_varint(&mut b) {
            assert_eq!(*i.as_ref(), 127);
            assert_eq!(1, b.len());
        }
    }
    #[test]
    fn varint_128_test() {
        let mut b = BytesMut::with_capacity(2);
        b.put_u8(0x80);
        b.put_u8(0x01);

        if let Ok(i) = decode_varint(&mut b) {
            assert_eq!(*i.as_ref(), 128);
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
        if let Ok(i) = decode_varint(&mut b) {
            assert_eq!(*i.as_ref(), 128);
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
        if let Ok(i) = decode_varint(&mut b) {
            assert_eq!(*i.as_ref(), 32767);
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
        if let Ok(_) = decode_varint(&mut b) {
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
        if let Ok(_) = decode_varint(&mut b) {
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
        if let Ok(i) = decode_varint(&mut b) {
            assert_eq!(*i.as_ref(), 32768);
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

        if let Ok(i) = decode_varint(&mut b) {
            assert_eq!(*i.as_ref(), 268_435_455);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_property_payload_format_indicator_with_will_message_is_unspecified_bytes_is_correct_size(
    ) {
        let mut b_prop = BytesMut::with_capacity(2);
        let mut b = BytesMut::with_capacity(100);

        b_prop.put_u8(Property::PayloadFormatIndicator as u8); // property identifier
        b_prop.put_u8(0x0); //
        if let Ok(..) = encode::encode_variable_byte_integer(
            &VariableByteInteger::new(b_prop.len() as u32),
            &mut b,
        ) {
            b.put(b_prop);

            assert_eq!(2, *b.get(0).unwrap());
        }
    }

    #[test]
    fn test_property_payload_with_zero_property_length() {
        let b_prop = BytesMut::with_capacity(2);
        let mut b = BytesMut::with_capacity(100);

        encode_variable_byte_integer(&VariableByteInteger::new(b_prop.len() as u32), &mut b);
        b.put(b_prop.to_vec().as_slice()); //insert payload format indicator property
        let blank_property: Vec<Property> = vec![];
        if let Ok(p) = decode::decode_property(&mut b) {
            assert_eq!(blank_property, p)
        }
    }

    #[test]
    fn test_property_type_byte_using_payload_format_indicator() {
        let mut b_prop = BytesMut::with_capacity(0);
        let mut b = BytesMut::with_capacity(100);

        b_prop.put_u8(Property::PayloadFormatIndicator as u8); // property identifier, Payload format indicator
        b_prop.put_u8(0x02); // value

        encode_variable_byte_integer(&VariableByteInteger::new(b_prop.len() as u32), &mut b);
        b.put(b_prop); //insert payload format indicator property

        if let Ok(p) = decode::decode_property(&mut b) {
            assert_eq!(vec![Property::PayloadFormatIndicator(Byte(0x02))], p)
        }
    }

    #[test]
    fn test_property_type_using_double_payload_format_indicator() {
        let mut b_prop = BytesMut::with_capacity(0);
        let mut b = BytesMut::with_capacity(100);

        b_prop.put_u8(Property::PayloadFormatIndicator as u8); // property identifier, Payload format indicator
        b_prop.put_u8(0x02); // value

        b_prop.put_u8(Property::PayloadFormatIndicator as u8); // property identifier, Payload format indicator
        b_prop.put_u8(0x03); // value

        encode_variable_byte_integer(&VariableByteInteger::new(b_prop.len() as u32), &mut b);
        b.put(b_prop); //insert payload format indicator property

        if let Ok(p) = decode::decode_property(&mut b) {
            assert_eq!(
                vec![
                    Property::PayloadFormatIndicator(Byte(0x02)),
                    Property::PayloadFormatIndicator(Byte(0x03))
                ],
                p
            )
        }
    }

    #[test]
    fn test_property_type_using_all_data_types() {
        let mut b_prop = BytesMut::with_capacity(100);
        let mut b = BytesMut::with_capacity(100);

        payload_format_indicator(99, &mut b_prop);
        message_expiry_interval(123456, &mut b_prop);
        content_type("hello", &mut b_prop);
        let mut binary_data = BytesMut::with_capacity(0);
        binary_data.put(vec![1u8, 2, 3, 4, 5].as_slice());
        correlation_data(&binary_data, &mut b_prop);
        subscription_identifier(1, &mut b_prop);
        println!("sizeof subsctiption id is {}", b_prop.len());

        encode_variable_byte_integer(&VariableByteInteger::new(b_prop.len() as u32), &mut b);

        b.put(b_prop);
        println!("size of variable byte integer is {}", b.len());
        if let Ok(p) = decode::decode_property(&mut b) {
            assert_eq!(
                vec![
                    Property::PayloadFormatIndicator(Byte(99)),
                    Property::MessageExpiryInterval(FourByteInteger(123456)),
                    Property::ContentType(Utf8EncodedString(String::from("hello"))),
                    Property::CorrelationData(BinaryData(vec![1u8, 2, 3, 4, 5])),
                    Property::SubscriptionIdentifier(VariableByteInteger(1))
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
        binary_data.put(vec![1u8, 2, 3, 4, 5].as_slice());
        correlation_data(&binary_data, &mut b_prop);
        payload_format_indicator(99, &mut b_prop);

        encode_variable_byte_integer(&VariableByteInteger::new(b_prop.len() as u32), &mut b);
        b.put(b_prop);
        println!("property before decoding is {:?}", b);
        if let Ok(p) = decode::decode_property(&mut b) {
            assert_eq!(
                vec![
                    Property::SubscriptionIdentifier(VariableByteInteger(12345)),
                    Property::MessageExpiryInterval(FourByteInteger(123456)),
                    Property::ContentType(Utf8EncodedString(String::from("hello"))),
                    Property::CorrelationData(BinaryData(vec![1u8, 2, 3, 4, 5])),
                    Property::PayloadFormatIndicator(Byte(99))
                ],
                p
            )
        }
    }

    #[test]
    fn test_for_invalid_properties_for_packet_type_connect_without_will_flag_not_set() {
        let assigned_property: Vec<Property> = vec![
            Property::AssignedClientIdentifier(Utf8EncodedString(String::from("hello"))),
            Property::SessionExpiryInterval(FourByteInteger(8)),
            Property::CorrelationData(BinaryData(vec![1, 2, 3, 4, 5])),
            Property::ResponseTopic(Utf8EncodedString(String::from("world"))),
        ];

        // let invalid_property_set: Vec<Property> = vec![
        //     Property::AssignedClientIdentifier,
        //     Property::CorrelationData,
        //     Property::ResponseTopic,
        // ];
        let invalid_property_set: Vec<Property> = vec![
            Property::AssignedClientIdentifier(Utf8EncodedString(String::from("hello"))),
            Property::CorrelationData(BinaryData(vec![1, 2, 3, 4, 5])),
            Property::ResponseTopic(Utf8EncodedString(String::from("world"))),
        ];

        assert_eq!(
            invalid_property_set,
            //invalid_property_for_packet_type(&assigned_property, vec![], CONNECT)
            invalid_property_for_connect_packet_type(&assigned_property, false)
        );
    }

    #[test]
    fn test_for_invalid_properties_for_packet_type_connect_with_will_flag_set() {
        let assigned_property: Vec<Property> = vec![
            Property::AssignedClientIdentifier(Utf8EncodedString(String::from("hello"))),
            Property::SessionExpiryInterval(FourByteInteger(8)),
            Property::CorrelationData(BinaryData(vec![1, 2, 3, 4, 5])),
            Property::AssignedClientIdentifier(Utf8EncodedString(String::from("world"))),
        ];

        let invalid_property: Vec<Property> = vec![
            Property::AssignedClientIdentifier(Utf8EncodedString(String::from("hello"))),
            Property::AssignedClientIdentifier(Utf8EncodedString(String::from("world"))),
        ];

        assert_eq!(
            invalid_property,
            invalid_property_for_connect_packet_type(&assigned_property, true)
        );
    }

    #[test]
    fn should_return_invalid_properties_for_packet_type_connack_with_will_flag_not_set() {
        let assigned_property: Vec<Property> = vec![
            Property::SubscriptionIdentifier(VariableByteInteger(8)),
            Property::SessionExpiryInterval(FourByteInteger(8)),
            Property::CorrelationData(BinaryData(vec![1, 2, 3, 4, 5])),
            Property::ResponseTopic(Utf8EncodedString(String::from("world"))),
        ];

        let invalid_property_set: Vec<Property> = vec![
            Property::SubscriptionIdentifier(VariableByteInteger(8)),
            Property::CorrelationData(BinaryData(vec![1, 2, 3, 4, 5])),
            Property::ResponseTopic(Utf8EncodedString(String::from("world"))),
        ];

        assert_eq!(
            invalid_property_set,
            invalid_property_for_non_connect_packet_type(&assigned_property, PacketTypes::Connack)
        );
    }

    #[test]
    fn should_return_invalid_properties_for_packet_type_publish_with_will_flag_not_set() {
        let assigned_property: Vec<Property> = vec![
            Property::SessionExpiryInterval(FourByteInteger(8)),
            Property::CorrelationData(BinaryData(vec![1, 2, 3, 4, 5])),
            Property::ResponseTopic(Utf8EncodedString(String::from("world"))),
        ];

        let invalid_property: Vec<Property> =
            vec![Property::SessionExpiryInterval(FourByteInteger(8))];

        assert_eq!(
            invalid_property,
            invalid_property_for_non_connect_packet_type(&assigned_property, PacketTypes::Publish)
        );
    }

    // puback

    #[test]
    fn should_return_invalid_properties_for_packet_type_puback_with_will_flag_not_set() {
        let assigned_property: Vec<Property> = vec![
            Property::ResponseTopic(Utf8EncodedString(String::from("hello"))),
            Property::CorrelationData(BinaryData(vec![1, 2, 3, 4, 5])),
            Property::ReasonString(Utf8EncodedString(String::from("world"))),
        ];

        let invalid_property: Vec<Property> = vec![
            Property::ResponseTopic(Utf8EncodedString(String::from("hello"))),
            Property::CorrelationData(BinaryData(vec![1, 2, 3, 4, 5])),
        ];

        assert_eq!(
            invalid_property,
            invalid_property_for_non_connect_packet_type(&assigned_property, PacketTypes::Puback)
        );
    }

    // pubrec
    #[test]
    fn should_return_invalid_properties_for_packet_type_pubrec_with_will_flag_not_set() {
        let assigned_property: Vec<Property> = vec![
            Property::ResponseTopic(Utf8EncodedString(String::from("hello"))),
            Property::CorrelationData(BinaryData(vec![1, 2, 3, 4, 5])),
            Property::ReasonString(Utf8EncodedString(String::from("world"))),
        ];

        let invalid_property: Vec<Property> = vec![
            Property::ResponseTopic(Utf8EncodedString(String::from("hello"))),
            Property::CorrelationData(BinaryData(vec![1, 2, 3, 4, 5])),
        ];

        assert_eq!(
            invalid_property,
            invalid_property_for_non_connect_packet_type(&assigned_property, PacketTypes::Pubrec)
        );
    }

    // pubrel
    #[test]
    fn should_return_invalid_properties_for_packet_type_pubrel_with_will_flag_not_set() {
        let assigned_property: Vec<Property> = vec![
            Property::ResponseTopic(Utf8EncodedString(String::from("hello"))),
            Property::CorrelationData(BinaryData(vec![1, 2, 3, 4, 5])),
            Property::ReasonString(Utf8EncodedString(String::from("world"))),
        ];

        let invalid_property: Vec<Property> = vec![
            Property::ResponseTopic(Utf8EncodedString(String::from("hello"))),
            Property::CorrelationData(BinaryData(vec![1, 2, 3, 4, 5])),
        ];

        assert_eq!(
            invalid_property,
            invalid_property_for_non_connect_packet_type(&assigned_property, PacketTypes::Pubrel)
        );
    }

    // pubcomp
    #[test]
    fn should_return_invalid_properties_for_packet_type_pubcomp_with_will_flag_not_set() {
        let assigned_property: Vec<Property> = vec![
            Property::ResponseTopic(Utf8EncodedString(String::from("hello"))),
            Property::CorrelationData(BinaryData(vec![1, 2, 3, 4, 5])),
            Property::ReasonString(Utf8EncodedString(String::from("world"))),
        ];

        let invalid_property: Vec<Property> = vec![
            Property::ResponseTopic(Utf8EncodedString(String::from("hello"))),
            Property::CorrelationData(BinaryData(vec![1, 2, 3, 4, 5])),
        ];

        assert_eq!(
            invalid_property,
            invalid_property_for_non_connect_packet_type(&assigned_property, PacketTypes::Pubcomp)
        );
    }

    // subscribe
    #[test]
    fn should_return_invalid_properties_for_packet_type_subscribe_with_will_flag_not_set() {
        let assigned_property: Vec<Property> = vec![
            Property::ResponseTopic(Utf8EncodedString(String::from("hello"))),
            Property::CorrelationData(BinaryData(vec![1, 2, 3, 4, 5])),
            Property::ReasonString(Utf8EncodedString(String::from("world"))),
            Property::SubscriptionIdentifier(VariableByteInteger(8)),
        ];

        let invalid_property: Vec<Property> = vec![
            Property::ResponseTopic(Utf8EncodedString(String::from("hello"))),
            Property::CorrelationData(BinaryData(vec![1, 2, 3, 4, 5])),
            Property::ReasonString(Utf8EncodedString(String::from("world"))),
        ];
        assert_eq!(
            invalid_property,
            invalid_property_for_non_connect_packet_type(
                &assigned_property,
                PacketTypes::Subscribe
            )
        );
    }

    // suback
    #[test]
    fn should_return_invalid_properties_for_packet_type_suback_with_will_flag_not_set() {
        let assigned_property: Vec<Property> = vec![
            Property::ResponseTopic(Utf8EncodedString(String::from("hello"))),
            Property::CorrelationData(BinaryData(vec![1, 2, 3, 4, 5])),
            Property::ReasonString(Utf8EncodedString(String::from("world"))),
            Property::SubscriptionIdentifier(VariableByteInteger(8)),
        ];

        let invalid_property: Vec<Property> = vec![
            Property::ResponseTopic(Utf8EncodedString(String::from("hello"))),
            Property::CorrelationData(BinaryData(vec![1, 2, 3, 4, 5])),
            Property::SubscriptionIdentifier(VariableByteInteger(8)),
        ];

        assert_eq!(
            invalid_property,
            invalid_property_for_non_connect_packet_type(&assigned_property, PacketTypes::Suback)
        );
    }

    // unsubscribe
    #[test]
    fn should_return_invalid_properties_for_packet_type_unsubscribe_with_will_flag_not_set() {
        let assigned_property: Vec<Property> = vec![
            Property::ResponseTopic(Utf8EncodedString(String::from("hello"))),
            Property::CorrelationData(BinaryData(vec![1, 2, 3, 4, 5])),
            Property::ReasonString(Utf8EncodedString(String::from("world"))),
            Property::SubscriptionIdentifier(VariableByteInteger(8)),
        ];

        let invalid_property: Vec<Property> = vec![
            Property::ResponseTopic(Utf8EncodedString(String::from("hello"))),
            Property::CorrelationData(BinaryData(vec![1, 2, 3, 4, 5])),
            Property::ReasonString(Utf8EncodedString(String::from("world"))),
            Property::SubscriptionIdentifier(VariableByteInteger(8)),
        ];

        assert_eq!(
            invalid_property,
            invalid_property_for_non_connect_packet_type(
                &assigned_property,
                PacketTypes::Unsubscribe
            )
        );
    }
    // unsuback
    #[test]
    fn should_return_invalid_properties_for_packet_type_unsuback_with_will_flag_not_set() {
        let assigned_property: Vec<Property> = vec![
            Property::ResponseTopic(Utf8EncodedString(String::from(String::from("hello")))),
            Property::CorrelationData(BinaryData(vec![1, 2, 3, 4, 5])),
            Property::ReasonString(Utf8EncodedString(String::from("world"))),
            Property::SubscriptionIdentifier(VariableByteInteger(8)),
            Property::User(Utf8StringPair(String::from("hello"), String::from("world"))),
        ];

        let invalid_property: Vec<Property> = vec![
            Property::ResponseTopic(Utf8EncodedString(String::from(String::from("hello")))),
            Property::CorrelationData(BinaryData(vec![1, 2, 3, 4, 5])),
            Property::SubscriptionIdentifier(VariableByteInteger(8)),
        ];

        assert_eq!(
            invalid_property,
            invalid_property_for_non_connect_packet_type(&assigned_property, PacketTypes::Unsuback),
        );
    }
    // pingreq
    #[test]
    fn should_return_invalid_properties_for_packet_type_pingreq_with_will_flag_not_set() {
        let assigned_property: Vec<Property> = vec![
            Property::ResponseTopic(Utf8EncodedString(String::from("hello"))),
            Property::CorrelationData(BinaryData(vec![1, 2, 3, 4, 5])),
            Property::ReasonString(Utf8EncodedString(String::from(String::from("world")))),
            Property::SubscriptionIdentifier(VariableByteInteger(8)),
            Property::User(Utf8StringPair(String::from("key"), String::from("value"))),
        ];

        let invalid_property: Vec<Property> = vec![
            Property::ResponseTopic(Utf8EncodedString(String::from("hello"))),
            Property::CorrelationData(BinaryData(vec![1, 2, 3, 4, 5])),
            Property::ReasonString(Utf8EncodedString(String::from(String::from("world")))),
            Property::SubscriptionIdentifier(VariableByteInteger(8)),
            Property::User(Utf8StringPair(String::from("key"), String::from("value"))),
        ];

        assert_eq!(
            invalid_property,
            invalid_property_for_non_connect_packet_type(&assigned_property, PacketTypes::Pingreq)
        );
    }

    // disconnect
    #[test]
    fn should_return_invalid_properties_for_packet_type_disconnect_with_will_flag_not_set() {
        let assigned_property: Vec<Property> = vec![
            Property::ResponseTopic(Utf8EncodedString(String::from("hello"))),
            Property::CorrelationData(BinaryData(vec![1, 2, 3, 4, 5])),
            Property::ReasonString(Utf8EncodedString(String::from("world"))),
            Property::SubscriptionIdentifier(VariableByteInteger(8)),
            Property::User(Utf8StringPair(String::from("key"), String::from("value"))),
        ];

        let invalid_property: Vec<Property> = vec![
            Property::ResponseTopic(Utf8EncodedString(String::from("hello"))),
            Property::CorrelationData(BinaryData(vec![1, 2, 3, 4, 5])),
            Property::SubscriptionIdentifier(VariableByteInteger(8)),
        ];

        assert_eq!(
            invalid_property,
            invalid_property_for_non_connect_packet_type(
                &assigned_property,
                PacketTypes::Disconnect
            )
        );
    }
    // auth
    #[test]
    fn should_return_invalid_properties_for_packet_type_auth_with_will_flag_not_set() {
        let assigned_property: Vec<Property> = vec![
            Property::ResponseTopic(Utf8EncodedString(String::from("hello"))),
            Property::CorrelationData(BinaryData(vec![1, 2, 3, 4, 5])),
            Property::ReasonString(Utf8EncodedString(String::from("world"))),
            Property::SubscriptionIdentifier(VariableByteInteger(8)),
            Property::User(Utf8StringPair(String::from("key"), String::from("value"))),
        ];

        // let invalid_property_set: Vec<Property> = vec![
        //     Property::ResponseTopic,
        //     Property::CorrelationData,
        //     Property::SubscriptionIdentifier,
        // ];

        let invalid_property: Vec<Property> = vec![
            Property::ResponseTopic(Utf8EncodedString(String::from("hello"))),
            Property::CorrelationData(BinaryData(vec![1, 2, 3, 4, 5])),
            Property::SubscriptionIdentifier(VariableByteInteger(8)),
        ];

        assert_eq!(
            invalid_property,
            invalid_property_for_non_connect_packet_type(&assigned_property, PacketTypes::Auth)
        );
    }
    #[test]
    fn test_property_type_four_byte_integer_using_message_expiry_interval_type() {
        let mut b_prop = BytesMut::with_capacity(2);
        let mut b = BytesMut::with_capacity(100);
        b_prop.put_u8(0x02); // property identifier, message expiry interval
        b_prop.put_u32(65530); // message expiry interval

        encode_variable_byte_integer(&VariableByteInteger::new(b_prop.len() as u32), &mut b);
        b.put(b_prop);

        if let Ok(p) = decode::decode_property(&mut b) {
            assert_eq!(
                vec![Property::MessageExpiryInterval(FourByteInteger(65530))],
                p
            )
        }
    }

    #[test]
    fn test_property_type_utf_8_encode_string_using_content_type() {
        let mut b_prop = BytesMut::with_capacity(2);
        let mut b = BytesMut::with_capacity(100);
        let b_string = String::from("hello world");
        b_prop.put_u8(Property::ContentType as u8); // property identifier, UTF-8 Encoded String

        b_prop.put_u16(b_string.len() as u16);
        b_prop.put(b_string.as_bytes());

        encode_variable_byte_integer(&VariableByteInteger::new(b_prop.len() as u32), &mut b);
        b.put(b_prop);

        if let Ok(p) = decode::decode_property(&mut b) {
            assert_eq!(
                vec![Property::ContentType(Utf8EncodedString(String::from(
                    "hello world"
                )))],
                p
            )
        }
    }

    #[test]
    fn test_property_type_binary_data_using_correlation_data() {
        let mut b_prop = BytesMut::with_capacity(2);
        let mut b = BytesMut::with_capacity(100);
        let b_binarydata = vec![1u8, 2u8, 3u8, 4u8, 5u8];

        b_prop.put_u8(Property::CorrelationData as u8);
        b_prop.put_u16(b_binarydata.len() as u16);
        b_prop.put(b_binarydata.as_slice());
        encode_variable_byte_integer(&VariableByteInteger::new(b_prop.len() as u32), &mut b);
        b.put(b_prop);

        if let Ok(p) = decode::decode_property(&mut b) {
            assert_eq!(
                vec![Property::CorrelationData(BinaryData(vec![1, 2, 3, 4, 5]))],
                p
            )
        }
    }

    #[test]
    fn test_property_type_variable_byte_integer() {
        let mut b_prop = BytesMut::with_capacity(20);
        let mut b = BytesMut::with_capacity(100);
        let b_integer: u32 = 268_435_455;

        b_prop.put_u8(Property::SubscriptionIdentifier as u8);
        encode_variable_byte_integer(&VariableByteInteger::new(b_integer), &mut b_prop);

        encode_variable_byte_integer(&VariableByteInteger::new(b_prop.len() as u32), &mut b); // size of property
        b.put(b_prop);
        println!("properties is {:?}", b);

        if let Ok(p) = decode::decode_property(&mut b) {
            assert_eq!(
                vec![Property::SubscriptionIdentifier(VariableByteInteger(
                    268_435_455
                ))],
                p
            )
        }
    }

    #[test]
    fn test_property_type_four_byte_integer() {
        let mut b_prop = BytesMut::with_capacity(2);
        let mut b = BytesMut::with_capacity(100);
        let b_integer: FourByteInteger = FourByteInteger::new(269_435_455);

        b_prop.put_u8(Property::SessionExpiryInterval as u8);
        b_prop.put_u32(*b_integer.as_ref());
        encode_variable_byte_integer(&VariableByteInteger::new(b_prop.len() as u32), &mut b);
        b.put(b_prop);

        if let Ok(p) = decode::decode_property(&mut b) {
            assert_eq!(
                vec![Property::SessionExpiryInterval(FourByteInteger(
                    269_435_455
                ))],
                p
            )
        }
    }

    #[test]
    fn test_property_type_two_integer_using_server_keep_alive() {
        let mut b_prop = BytesMut::with_capacity(2);
        let mut b = BytesMut::with_capacity(100);

        b_prop.put_u8(Property::ServerKeepAlive as u8);
        b_prop.put_u16(0x1001);

        encode_variable_byte_integer(&VariableByteInteger::new(b_prop.len() as u32), &mut b);
        b.put(b_prop);

        if let Ok(p) = decode::decode_property(&mut b) {
            assert_eq!(vec![Property::ServerKeepAlive(TwoByteInteger(0x1001))], p)
        }
    }

    #[test]
    fn test_property_type_utf8_string_pair_using_user_property() {
        let mut b_prop = BytesMut::with_capacity(2);
        let mut b = BytesMut::with_capacity(100);

        let key = b"Hello";
        let value = b"World";

        b_prop.put_u8(Property::User as u8);
        encode::utf8_string_pair("Hello", "World", &mut b_prop);
        encode_variable_byte_integer(&VariableByteInteger::new(b_prop.len() as u32), &mut b);
        b.put(b_prop);

        if let Ok(p) = decode::decode_property(&mut b) {
            assert_eq!(
                vec![Property::User(Utf8StringPair(
                    String::from("Hello"),
                    String::from("World")
                ))],
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
        let properties: Vec<Property> = vec![Property::User(Utf8StringPair(
            String::from("key"),
            String::from("value"),
        ))];
        let returned_properties: HashMap<PropertyIdentifier, Vec<Property>> = HashMap::new();
        assert_eq!(non_unique_properties(&properties), returned_properties);
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
        let returned_properties: HashMap<PropertyIdentifier, Vec<Property>> = HashMap::new();
        assert_eq!(non_unique_properties(&properties), returned_properties);
    }

    #[test]
    fn test_two_assigned_client_id_dentifier_are_invalid() {
        let properties: Vec<Property> = vec![
            Property::AssignedClientIdentifier(Utf8EncodedString(String::from("hello"))),
            Property::AssignedClientIdentifier(Utf8EncodedString(String::from("hello"))),
        ];

        // let returned_properties: Vec<Property> = vec![Property::AssignedClientIdentifier {
        //     value: Utf8EncodedString(String::from("hello")),
        // }];

        let mut return_properties: HashMap<PropertyIdentifier, Vec<Property>> = HashMap::new();
        return_properties.insert(
            PropertyIdentifier::new(PropertyIdentifierConstant::AssignedClientIdentifier),
            vec![
                Property::AssignedClientIdentifier(Utf8EncodedString(String::from("hello"))),
                Property::AssignedClientIdentifier(Utf8EncodedString(String::from("hello"))),
            ],
        );
        assert_eq!(non_unique_properties(&properties), return_properties);
    }

    #[test]
    fn test_parse_connect_control_packet() {
        // let mut packets = BytesMut::with_capacity(200);
        // packets.put_u8(0b0001_0000);
        // //packets.put variable byte integer

        // select control packets type

        //
    }
}
