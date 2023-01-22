pub mod auth;
pub mod connack;
pub mod connect;
pub mod disconnect;
pub mod pingreq;
pub mod pingresp;
pub mod puback;
pub mod pubcomp;
pub mod publish;
pub mod pubrec;

pub mod pubrel;
pub mod suback;
pub mod subscribe;
pub mod unsuback;
pub mod unsubscribe;

use bytes::{Buf, BufMut, BytesMut};
mod error {
    use crate::mqttbroker::properties::{Property, PropertyIdentifier};
    use std::collections::HashMap;
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum PropertyError {
        #[error("property {0:?} has already been inserted, you are trying to insert a duplicate copy into {1}")]
        PropertyAlreadyInserted(HashMap<PropertyIdentifier, Vec<Property>>, String),
        // #[error("property {0} is not valid for Will Topic of CONNECT Payload")]
        // InvalidConnectPayloadWillProperty(String),
        // #[error("property {0} is not valid for Properties of CONNECT Variable Header")]
        // InvalidConnectVariableHeaderProperty(String),
        #[error("property {0:?} is not valid for packets type {1}")]
        InvalidProperty(Vec<Property>, String),
    }
}

pub trait BuilderLifecycle<T, E> {
    fn new() -> Self;
    fn build(self) -> Result<T, E>;
}

pub trait GeneratePacketParts {
    //fn generate_fixed_header(&self, fixed_header_remaining_length: usize) -> BytesMut;

    fn generate_fixed_header(
        packet_type: u8,
        packet_type_low_nibble: u8,
        fixed_header_remaining_length: usize,
    ) -> BytesMut {
        let mut fixed_header = BytesMut::with_capacity(5);

        fixed_header = encode_fixed_header(
            fixed_header,
            packet_type,
            packet_type_low_nibble,
            fixed_header_remaining_length,
        );

        fixed_header
    }

    fn generate_variable_header(&self) -> BytesMut;
    fn generate_payload(&self) -> BytesMut;
}

use crate::encode::variable_byte_integer;
use crate::mqttbroker::packets::error::PropertyError;
use crate::mqttbroker::primitive_types::VariableByteInteger;
use crate::mqttbroker::properties::{invalid_property_for_packet_type, non_unique, Property};
use thiserror::Error;
use tracing::trace;

#[derive(Error, Debug)]
pub enum ConnectPacketBuildError {
    #[error("Will flag is not set")]
    WillFlagNotSet,
    #[error("Will topic not set")]
    WillTopicNotSet,
    #[error("Will payload not set")]
    WillPayLoadNotSet,
}

mod connect_flags {
    pub const CLEAN_START: u8 = 2;
    pub const WILL_FLAG: u8 = 4;
    pub const WILL_QOS_MASK: u8 = 8 + 16;
    // pub const WILL_QOS0: u8 = 8;
    // pub const WILL_QOS1: u8 = 1 << 2;
    // pub const WILL_QOS2: u8 = 2 << 2;
    pub const WILL_RETAIN: u8 = 1 << 5;
    pub const USER_NAME_FLAG: u8 = 1 << 7;
    pub const PASSWORD_FLAG: u8 = 1 << 6;
}

pub mod properties {

    use crate::mqttbroker::primitive_types::{
        BinaryData, Byte, FourByteInteger, TwoByteInteger, Utf8EncodedString, Utf8StringPair,
        VariableByteInteger,
    };
    use std::collections::HashMap;

    #[derive(Default, Debug, PartialEq, Eq, Clone)]
    pub struct ConnectProperties {
        property_map: HashMap<u8, PropertyContainer>,
    }

    macro_rules! create_property_struct {
                ($($struc:ident),*) => {
                    $(
                        #[derive(Default, Debug, PartialEq, Eq, Clone)]
                        pub struct $struc {
                            property_map: HashMap<u8, PropertyContainer>,
                        }
                    )*
                };
            }

    create_property_struct!(
        ConnAckProperties,
        PublishProperties,
        PubackProperties,
        PubrecProperties,
        PubrelProperties,
        PubcompProperties,
        SubscribeProperties,
        SubackProperties,
        UnSubscribeProperties,
        UnsubackProperties,
        DisconnectProperties,
        AuthProperties,
        WillProperties
    );

    macro_rules! impl_modify_properties {
                ($($struc: ty),*) => {
                    $(
                        impl ModifyProperties for $struc {
                           fn get_properties(&mut self) -> &mut HashMap<u8, PropertyContainer> {
                                &mut self.property_map
                            }
                        }
                    )*
                };
            }

    impl_modify_properties!(
        ConnectProperties,
        ConnAckProperties,
        PublishProperties,
        PubackProperties,
        PubrecProperties,
        PubrelProperties,
        PubcompProperties,
        SubscribeProperties,
        SubackProperties,
        UnSubscribeProperties,
        UnsubackProperties,
        DisconnectProperties,
        AuthProperties,
        WillProperties
    );

    use crate::mqttbroker::packets::properties::private::ModifyProperties;
    use crate::mqttbroker::properties::{PropertyIdentifier, PropertyIdentifierConstant};
    use paste::paste;

    macro_rules! create_user_property_trait {
        ($propname:ident, $property_type:ident) => {
            paste! {
                pub trait $propname: ModifyProperties {

                    fn [<set_ $propname:snake>](&mut self, value: $property_type) {
                        self.[<set_ $property_type:snake >](PropertyIdentifier::new(PropertyIdentifierConstant::$propname), value);
                    }

                    fn [<$propname:snake>](&mut self) -> Option<&$property_type> {
                        self.[<$property_type:snake>](PropertyIdentifier::new(PropertyIdentifierConstant::$propname))
                    }
                }
            }
        };
    }

    create_user_property_trait!(PayloadFormatIndicator, Byte);

    // trait PayloadFormatIndicator: ModifyProperties {
    //     fn set_payload_format_indicator(&mut self, value: Byte) {
    //         self.set_byte(PropertyIdentifiers::PayloadFormatIndicator, value);
    //     }
    //
    //     fn payload_format_indicator(&mut self) -> Option<&Byte> {
    //         self.byte(PropertyIdentifiers::PayloadFormatIndicator)
    //     }
    // }

    create_user_property_trait!(MessageExpiryInterval, FourByteInteger);

    create_user_property_trait!(ContentType, Utf8EncodedString);

    create_user_property_trait!(ResponseTopic, Utf8EncodedString);

    create_user_property_trait!(CorrelationData, BinaryData);

    create_user_property_trait!(SubscriptionIdentifier, VariableByteInteger);

    create_user_property_trait!(SubscriptionIdentifierAvailable, Byte);

    create_user_property_trait!(SessionExpiryInterval, FourByteInteger);

    create_user_property_trait!(AssignedClientIdentifier, Utf8EncodedString);

    create_user_property_trait!(ServerKeepAlive, TwoByteInteger);

    create_user_property_trait!(AuthenticationMethod, Utf8EncodedString);

    create_user_property_trait!(AuthenticationData, BinaryData);

    create_user_property_trait!(RequestProblemInformation, Byte);

    create_user_property_trait!(WillDelayInterval, FourByteInteger);

    create_user_property_trait!(RequestResponseInformation, Byte);

    create_user_property_trait!(ResponseInformation, Utf8EncodedString);

    create_user_property_trait!(ServerReference, Utf8EncodedString);

    create_user_property_trait!(ReasonString, Utf8EncodedString);

    create_user_property_trait!(ReceiveMaximum, TwoByteInteger);

    create_user_property_trait!(TopicAliasMaximum, TwoByteInteger);

    create_user_property_trait!(TopicAlias, TwoByteInteger);

    create_user_property_trait!(MaximumQos, Byte);

    create_user_property_trait!(RetainAvailable, Byte);

    create_user_property_trait!(MaximumPacketSize, FourByteInteger);

    create_user_property_trait!(WildcardSubscriptionAvailable, Byte);

    create_user_property_trait!(SharedSubscriptionAvailable, VariableByteInteger);

    trait UserProperty: ModifyProperties {
        fn append_user_property(&mut self, value: Utf8StringPair) {
            self.append_utf8_string_pair(
                PropertyIdentifier::new(PropertyIdentifierConstant::User),
                value,
            );
        }

        fn clear_user_property(&mut self) {
            self.clear_utf8_string_pair(PropertyIdentifier::new(PropertyIdentifierConstant::User));
        }

        fn user_property(&mut self, key: String) -> Option<&Vec<Utf8StringPair>> {
            self.utf8_string_pair(PropertyIdentifier::new(PropertyIdentifierConstant::User))
        }
    }

    macro_rules! impl_traits_for_type {
                ($($trt:ty),*; $struc:ty) => {
                    $(
                        impl $trt for $struc{}
                    )*
                };
            }

    impl_traits_for_type!(WillDelayInterval,PayloadFormatIndicator, MessageExpiryInterval, ContentType, ResponseTopic, CorrelationData, UserProperty; ConnectProperties );

    impl_traits_for_type!(SessionExpiryInterval, ReceiveMaximum, MaximumQos, RetainAvailable, MaximumPacketSize, AssignedClientIdentifier, TopicAliasMaximum, ReasonString, UserProperty, WildcardSubscriptionAvailable, SubscriptionIdentifierAvailable, SharedSubscriptionAvailable, ServerKeepAlive, ResponseInformation, ServerReference, AuthenticationMethod, AuthenticationData; ConnAckProperties);

    impl_traits_for_type!(PayloadFormatIndicator, MessageExpiryInterval, TopicAlias, ResponseTopic, CorrelationData, UserProperty, SubscriptionIdentifier, ContentType; PublishProperties);

    impl_traits_for_type!(ReasonString, UserProperty; PubackProperties);

    impl_traits_for_type!(ReasonString, UserProperty; PubrecProperties);

    impl_traits_for_type!(ReasonString, UserProperty; PubrelProperties);

    impl_traits_for_type!(ReasonString, UserProperty; PubcompProperties);

    impl_traits_for_type!(SubscriptionIdentifier, UserProperty; SubscribeProperties);

    impl_traits_for_type!(ReasonString, UserProperty; SubackProperties);

    impl_traits_for_type!(UserProperty; UnSubscribeProperties);

    impl_traits_for_type!(UserProperty; UnsubackProperties);

    impl_traits_for_type!(SessionExpiryInterval, ReasonString, UserProperty, ServerReference; DisconnectProperties);

    impl_traits_for_type!(AuthenticationMethod, AuthenticationData, ReasonString, UserProperty; AuthProperties);

    mod private {
        use crate::mqttbroker::packets::properties::PropertyContainer;
        use crate::mqttbroker::primitive_types::{
            BinaryData, Byte, FourByteInteger, TwoByteInteger, Utf8EncodedString, Utf8StringPair,
            VariableByteInteger,
        };
        use crate::mqttbroker::properties::PropertyIdentifier;
        use std::collections::HashMap;

        pub trait ModifyProperties {
            #[doc(hidden)]
            fn get_properties(&mut self) -> &mut HashMap<u8, PropertyContainer>;
            #[doc(hidden)]
            fn four_byte_integer(&mut self, key: PropertyIdentifier) -> Option<&FourByteInteger> {
                match self.get_properties().get(&(key.value as u8)) {
                    Some(PropertyContainer::FourByteInteger(x)) => Some(x),
                    _ => None,
                }
            }
            #[doc(hidden)]
            fn set_four_byte_integer(
                &mut self,
                key: PropertyIdentifier,
                value: FourByteInteger,
            ) -> Option<PropertyContainer> {
                self.get_properties().insert(
                    key.value as u8,
                    PropertyContainer::FourByteInteger(FourByteInteger::new(*value.as_ref())),
                )
            }
            #[doc(hidden)]
            fn utf8_encoded_string(
                &mut self,
                key: PropertyIdentifier,
            ) -> Option<&Utf8EncodedString> {
                match self.get_properties().get(&(key.value as u8)) {
                    Some(PropertyContainer::UTF8EncodeString(x)) => Some(x),
                    _ => None,
                }
            }
            #[doc(hidden)]
            fn set_utf8_encoded_string(
                &mut self,
                key: PropertyIdentifier,
                value: Utf8EncodedString,
            ) -> Option<PropertyContainer> {
                self.get_properties()
                    .insert(key.value as u8, PropertyContainer::UTF8EncodeString(value))
            }
            #[doc(hidden)]
            fn binary_data(&mut self, key: PropertyIdentifier) -> Option<&BinaryData> {
                match self.get_properties().get(&(key.value as u8)) {
                    Some(PropertyContainer::BinaryData(x)) => Some(x),
                    _ => None,
                }
            }
            #[doc(hidden)]
            fn set_binary_data(
                &mut self,
                key: PropertyIdentifier,
                value: BinaryData,
            ) -> Option<PropertyContainer> {
                self.get_properties().insert(
                    key.value as u8,
                    PropertyContainer::BinaryData(BinaryData::new(value.0)),
                )
            }
            #[doc(hidden)]
            fn byte(&mut self, key: PropertyIdentifier) -> Option<&Byte> {
                match self.get_properties().get(&(key.value as u8)) {
                    Some(PropertyContainer::Byte(x)) => Some(x),
                    _ => None,
                }
            }
            #[doc(hidden)]
            fn set_byte(
                &mut self,
                key: PropertyIdentifier,
                value: Byte,
            ) -> Option<PropertyContainer> {
                self.get_properties().insert(
                    key.value as u8,
                    PropertyContainer::Byte(Byte::new(*value.as_ref())),
                )
            }
            #[doc(hidden)]
            fn variable_byte_integer(
                &mut self,
                key: PropertyIdentifier,
            ) -> Option<&VariableByteInteger> {
                match self.get_properties().get(&(key.value as u8)) {
                    Some(PropertyContainer::VariableByteInteger(x)) => Some(x),
                    _ => None,
                }
            }
            #[doc(hidden)]
            fn set_variable_byte_integer(
                &mut self,
                key: PropertyIdentifier,
                value: VariableByteInteger,
            ) -> Option<PropertyContainer> {
                self.get_properties().insert(
                    key.value as u8,
                    PropertyContainer::VariableByteInteger(VariableByteInteger::new(
                        *value.as_ref(),
                    )),
                )
            }
            #[doc(hidden)]
            fn two_byte_integer(&mut self, key: PropertyIdentifier) -> Option<&TwoByteInteger> {
                match self.get_properties().get(&(key.value as u8)) {
                    Some(PropertyContainer::TwoByteInteger(x)) => Some(x),
                    _ => None,
                }
            }
            #[doc(hidden)]
            fn set_two_byte_integer(
                &mut self,
                key: PropertyIdentifier,
                value: TwoByteInteger,
            ) -> Option<PropertyContainer> {
                self.get_properties().insert(
                    key.value as u8,
                    PropertyContainer::TwoByteInteger(TwoByteInteger::new(*value.as_ref())),
                )
            }
            #[doc(hidden)]
            fn utf8_string_pair(
                &mut self,
                key: PropertyIdentifier,
            ) -> Option<&Vec<Utf8StringPair>> {
                match self.get_properties().get(&(key.value as u8)) {
                    Some(PropertyContainer::UTF8StringPairList(x)) => Some(x),
                    _ => None,
                }
            }
            #[doc(hidden)]
            fn append_utf8_string_pair(&mut self, key: PropertyIdentifier, value: Utf8StringPair) {
                if let Some(PropertyContainer::UTF8StringPairList(ref mut lp)) =
                    self.get_properties().get_mut(&(key.value as u8))
                {
                    lp.push(value)
                }
            }
            #[doc(hidden)]
            fn clear_utf8_string_pair(&mut self, key: PropertyIdentifier) {
                if let Some(PropertyContainer::UTF8StringPairList(ref mut lp)) =
                    self.get_properties().get_mut(&(key.value as u8))
                {
                    lp.clear()
                }
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub enum PropertyContainer {
        FourByteInteger(FourByteInteger),
        UTF8EncodeString(Utf8EncodedString),
        BinaryData(BinaryData),
        Byte(Byte),
        VariableByteInteger(VariableByteInteger),
        TwoByteInteger(TwoByteInteger),
        UTF8StringPairList(Vec<Utf8StringPair>),
    }
}

#[cfg(test)]
mod properties_test {
    mod tests {
        // use crate::mqttbroker::mqtt_broker::packets::ServerKeepAlive;
        use crate::mqttbroker::primitive_types::TwoByteInteger;
        // #[test]
        // pub fn test_set_four_byte_integer() {
        //     impl MessageExpiryInterval for PropertyMap {};
        //
        //     let mut message_expiry_internal = PropertyMap::new();
        //     message_expiry_internal.set_message_expiry_interval(FourByteInteger(100));
        //     let read = message_expiry_internal.message_expiry_interval();
        //     assert_eq!(FourByteInteger(100), *read.unwrap());
        // }
        //
        // #[test]
        // pub fn test_byte() {
        //     impl PayloadFormatIndicator for PropertyMap {};
        //
        //     let mut payload_format_indicator = PropertyMap::new();
        //     payload_format_indicator.set_payload_format_indicator(Byte(100));
        //     let read = payload_format_indicator.payload_format_indicator();
        //     assert_eq!(Byte(100), *read.unwrap());
        // }
        // #[test]
        // pub fn test_utf8_encoded_string() {
        //     impl ContentType for PropertyMap {};
        //
        //     let mut content_type = PropertyMap::new();
        //     content_type.set_content_type(UTF8EncodedString(String::from("hello")));
        //     let read = content_type.content_type();
        //     assert_eq!(UTF8EncodedString(String::from("hello")), *read.unwrap());
        // }
        //
        // #[test]
        // pub fn test_binary_data() {
        //     impl CorrelationData for PropertyMap {};
        //
        //     let mut correlation_data = PropertyMap::new();
        //     correlation_data.set_correlation_data(BinaryData(vec![1, 2, 3, 4]));
        //     let read = correlation_data.correlation_data();
        //     assert_eq!(BinaryData(vec![1, 2, 3, 4]), *read.unwrap());
        // }
        //
        // #[test]
        // pub fn test_variable_byte_integer() {
        //     impl SubscriptionIdentifier for PropertyMap {};
        //
        //     let mut subscription_identifier = PropertyMap::new();
        //     subscription_identifier.set_subscription_identifier(VariableByteInteger(999));
        //     let read = subscription_identifier.subscription_identifier();
        //     assert_eq!(VariableByteInteger(999), *read.unwrap());
        // }

        use crate::mqttbroker::packets::properties::ServerKeepAlive;
        use crate::mqttbroker::packets::properties::{ConnAckProperties, PublishProperties};

        #[test]
        pub fn test_two_byte_integer() {
            let mut server_keep_alive: ConnAckProperties = Default::default();

            server_keep_alive.set_server_keep_alive(TwoByteInteger(213));
            let read = server_keep_alive.server_keep_alive();
            assert_eq!(TwoByteInteger(213), *read.unwrap());
        }

        #[test]
        pub fn test_utf8_string_pair() {
            let mut user_property: PublishProperties = Default::default();
            //let t = user_property.

            // user_property.append_user_property(UTF8StringPair(
            //     String::from("hello"),
            //     String::from("world"),
            // ));

            //let read = user_property
        }
    }
}

// #[derive(Debug)]
// pub struct ConnectPacketBuilder {
//     pub connect_packet: ConnectPacket,
// }

// Can use this from all the packets
fn encode_properties_to_vec(props: &Option<Vec<Property>>) -> Vec<u8> {
    let mut properties_vec: Vec<u8> = Vec::with_capacity(200);

    if props.is_none() {
        properties_vec = vec![]
    } else {
        if props.is_some() {
            for prop_item in props.as_ref().unwrap() {
                prop_item.encode(&mut properties_vec);
            }
        }
    }

    properties_vec
}

fn prepend_size_to_properties(properties: Vec<u8>, mut variable_header: BytesMut) -> BytesMut {
    let mut encoded_variable_header_properties_size: BytesMut = BytesMut::with_capacity(4);
    variable_byte_integer(
        "variable header properties size",
        &VariableByteInteger::new(properties.len() as u32),
        &mut encoded_variable_header_properties_size,
    )
    .unwrap();
    variable_header.put_slice(encoded_variable_header_properties_size.iter().as_slice());
    variable_header.put_slice(properties.as_slice());

    variable_header
}

pub fn encode_properties(
    mut variable_header: BytesMut,
    variable_header_properties: &Option<Vec<Property>>,
) -> BytesMut {
    let encoded_variable_header_properties = if variable_header_properties.is_none() {
        vec![]
    } else {
        encode_properties_to_vec(variable_header_properties)
    };

    variable_header =
        prepend_size_to_properties(encoded_variable_header_properties, variable_header);

    variable_header
}

pub fn encode_fixed_header(
    mut fixed_header: BytesMut,
    packet_type: u8,
    packet_type_flags: u8,
    fixed_header_remaining_length: usize,
) -> BytesMut {
    let pt = (packet_type << 4) + (packet_type_flags & 0x0f);

    fixed_header.put_u8(pt);
    // need to capture error here.
    variable_byte_integer(
        "remaining length",
        &VariableByteInteger::new(fixed_header_remaining_length as u32),
        &mut fixed_header,
    )
    .unwrap();

    fixed_header
}

#[cfg(test)]
mod encode_test {
    use crate::mqttbroker::packets::connect::builder::ConnectBuilder;
    use crate::mqttbroker::packets::BuilderLifecycle;

    #[test]
    pub fn will_properties_not_set() {
        let packet = ConnectBuilder::new();

        assert!(!packet.packet.will_flag())
    }
}

pub trait Decoder<T> {
    fn decode(bytes: &mut BytesMut) -> anyhow::Result<T>;
}

// pub trait Encoder<T: GeneratePacketParts, E> {
//     fn encode(&self) -> anyhow::Result<BytesMut>;
// }

pub trait Encoder<T: GeneratePacketParts> {
    //fn encode(&self) -> anyhow::Result<BytesMut>;
    fn encode(
        packet_type: u8,
        packet_type_low_nibble: u8,
        generated_packet_parts: &impl GeneratePacketParts,
    ) -> anyhow::Result<BytesMut> {
        let variable_header = generated_packet_parts.generate_variable_header();

        let payload = generated_packet_parts.generate_payload();

        let fixed_header_remaining_length = variable_header.len() + payload.len();
        let mut connack_packet = BytesMut::with_capacity(fixed_header_remaining_length + 1 + 4);
        let fixed_header = T::generate_fixed_header(
            packet_type,
            packet_type_low_nibble,
            fixed_header_remaining_length,
        );

        connack_packet.put(fixed_header);
        connack_packet.put(variable_header);
        connack_packet.put(payload);

        Ok(connack_packet)
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
#[repr(u8)]
pub enum PacketTypes {
    Connect = 0x01,
    Connack = 0x02,
    Publish = 0x03,
    Puback = 0x04,
    Pubrec = 0x05,
    Pubrel = 0x06,
    Pubcomp = 0x07,
    Subscribe = 0x08,
    Suback = 0x09,
    Unsubscribe = 0x0a,
    Unsuback = 0x0b,
    Pingreq = 0x0c,
    Pingresp = 0x0d,
    Disconnect = 0x0e,
    Auth = 0x0f,
}

pub mod reason_codes {
    use thiserror::Error;

    #[derive(Error, Debug, PartialEq, Eq)]
    pub enum ReasonCodeError {
        #[error("Invalid reason code '{0}' for packet type '{1}'")]
        InvalidReasonCode(u8, String),
    }

    pub trait DecodeReasonCode<T, E> {
        fn decode(reason_code: u8) -> Result<T, E>;
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    #[repr(u8)]
    pub enum CONNECTACK {
        Success = 0x00,
        UnspecifiedError = 0x80,
        MalformedPacket = 0x81,
        ProtocolError = 0x82,
        ImplementationSpecificError = 0x83,
        UnsupportedProtocolVersion = 0x84,
        ClientIdentifierNotValid = 0x85,
        BadUserNameOrPassword = 0x86,
        NotAuthorised = 0x87,
        ServerUnavailable = 0x88,
        ServerBusy = 0x89,
        Banned = 0x8A,
        BadAuthenticationMethod = 0x8c,
        TopicNameInvalid = 0x90,
        PacketTooLarge = 0x95,
        QuotaExceeded = 0x97,
        PayloadFormatInvalid = 0x99,
        RetainNotSupported = 0x9A,
        QosNotSupported = 0x9B,
        UseAnotherServer = 0x9C,
        ServerMoved = 0x9D,
        ConnectionRateExceeded = 0x9F,
    }

    impl DecodeReasonCode<CONNECTACK, ReasonCodeError> for CONNECTACK {
        fn decode(reason_code: u8) -> Result<CONNECTACK, ReasonCodeError> {
            let ret = match reason_code {
                0u8 => CONNECTACK::Success,
                0x80 => CONNECTACK::UnspecifiedError,
                0x81 => CONNECTACK::MalformedPacket,
                0x82 => CONNECTACK::ProtocolError,
                0x83 => CONNECTACK::ImplementationSpecificError,
                0x84 => CONNECTACK::UnsupportedProtocolVersion,
                0x85 => CONNECTACK::ClientIdentifierNotValid,
                0x86 => CONNECTACK::BadUserNameOrPassword,
                0x87 => CONNECTACK::NotAuthorised,
                0x88 => CONNECTACK::ServerUnavailable,
                0x89 => CONNECTACK::ServerBusy,
                0x8A => CONNECTACK::Banned,
                0x8c => CONNECTACK::BadAuthenticationMethod,
                0x90 => CONNECTACK::TopicNameInvalid,
                0x95 => CONNECTACK::PacketTooLarge,
                0x97 => CONNECTACK::QuotaExceeded,
                0x99 => CONNECTACK::PayloadFormatInvalid,
                0x9A => CONNECTACK::RetainNotSupported,
                0x9B => CONNECTACK::QosNotSupported,
                0x9C => CONNECTACK::UseAnotherServer,
                0x9D => CONNECTACK::ServerMoved,
                0x9F => CONNECTACK::ConnectionRateExceeded,

                n => {
                    return Err(ReasonCodeError::InvalidReasonCode(
                        n,
                        String::from("CONNECTACK"),
                    ))
                }
            };

            Ok(ret)
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    #[repr(u8)]
    pub enum PUBACK {
        Success = 0x00,
        NoMatchingSubscribers = 0x10,
        UnspecifiedError = 0x80,
        ImplementationSpecificError = 0x83,
        NotAuthorized = 0x87,
        TopicNameInvalid = 0x90,
        PacketIdentifierInUse = 0x91,
        QuotaExceeded = 0x97,
        PayloadFormatInvalid = 0x99,
    }

    impl DecodeReasonCode<PUBACK, ReasonCodeError> for PUBACK {
        fn decode(reason_code: u8) -> Result<PUBACK, ReasonCodeError> {
            let ret = match reason_code {
                0x00 => PUBACK::Success,
                0x10 => PUBACK::NoMatchingSubscribers,
                0x80 => PUBACK::UnspecifiedError,
                0x83 => PUBACK::ImplementationSpecificError,
                0x87 => PUBACK::NotAuthorized,
                0x90 => PUBACK::TopicNameInvalid,
                0x91 => PUBACK::PacketIdentifierInUse,
                0x97 => PUBACK::QuotaExceeded,
                0x99 => PUBACK::PayloadFormatInvalid,
                n => {
                    return Err(ReasonCodeError::InvalidReasonCode(
                        n,
                        String::from("PUBACK"),
                    ))
                }
            };

            Ok(ret)
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    #[repr(u8)]
    pub enum PUBREC {
        Success = 0x00,
        NoMatchingSubscribers = 0x10,
        UnspecifiedError = 0x80,
        ImplementationSpecificError = 0x83,
        NotAuthorized = 0x87,
        TopicNameInvalid = 0x90,
        PacketIdentifierInUse = 0x91,
        QuotaExceed = 0x97,
        PayloadFormatInvalid = 0x99,
    }

    impl DecodeReasonCode<PUBREC, ReasonCodeError> for PUBREC {
        fn decode(reason_code: u8) -> Result<PUBREC, ReasonCodeError> {
            let ret = match reason_code {
                0x00 => PUBREC::Success,
                0x10 => PUBREC::NoMatchingSubscribers,
                0x80 => PUBREC::UnspecifiedError,
                0x83 => PUBREC::ImplementationSpecificError,
                0x87 => PUBREC::NotAuthorized,
                0x90 => PUBREC::TopicNameInvalid,
                0x91 => PUBREC::PacketIdentifierInUse,
                0x97 => PUBREC::QuotaExceed,
                0x99 => PUBREC::PayloadFormatInvalid,
                n => {
                    return Err(ReasonCodeError::InvalidReasonCode(
                        n,
                        String::from("PUBREC"),
                    ))
                }
            };

            Ok(ret)
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    #[repr(u8)]
    pub enum PUBREL {
        Success = 0x00,
        PacketIdentifierNotFound = 0x92,
    }

    impl DecodeReasonCode<PUBREL, ReasonCodeError> for PUBREL {
        fn decode(reason_code: u8) -> Result<PUBREL, ReasonCodeError> {
            let ret = match reason_code {
                0x00 => PUBREL::Success,
                0x92 => PUBREL::PacketIdentifierNotFound,
                n => {
                    return Err(ReasonCodeError::InvalidReasonCode(
                        n,
                        String::from("PUBREL"),
                    ))
                }
            };

            Ok(ret)
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    #[repr(u8)]
    pub enum PUBCOMP {
        Success = 0x00,
        PacketIdentifierNotFound = 0x92,
    }

    impl DecodeReasonCode<PUBCOMP, ReasonCodeError> for PUBCOMP {
        fn decode(reason_code: u8) -> Result<PUBCOMP, ReasonCodeError> {
            let ret = match reason_code {
                0x00 => PUBCOMP::Success,
                0x92 => PUBCOMP::PacketIdentifierNotFound,
                n => {
                    return Err(ReasonCodeError::InvalidReasonCode(
                        n,
                        String::from("PUBCOMP"),
                    ))
                }
            };

            Ok(ret)
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    #[repr(u8)]
    pub enum DISCONNECT {
        NormalDisconnection = 0x00,
        DisconnectWithWillMessage = 0x04,
        UnspecifiedError = 0x80,
        MalformedPacket = 0x81,
        ProtocolError = 0x82,
        ImplementationSpecificError = 0x83,
        NotAuthorized = 0x87,
        ServerBusy = 0x89,
        ServerShuttingDown = 0x8b,
        KeepAliveTimeout = 0x8d,
        SessionTakenOver = 0x8e,
        TopicFilterInvalid = 0x8f,
        TopicNameInvalid = 0x90,
        ReceiveMaximumExceed = 0x93,
        TopicAliasInvalid = 0x94,
        PacketTooLarge = 0x95,
        MessageRateTooHigh = 0x96,
        QuotaExceeded = 0x97,
        AdministrativeAction = 0x98,
        PayloadFormatInvalid = 0x99,
        RetainNotSupported = 0x9a,
        QOSNotSupported = 0x9b,
        UseAnotherServer = 0x9c,
        ServerMoved = 0x9d,
        SharedSubscriptionsNotSupported = 0x9e,
        ConnectionRateExceeded = 0x9f,
        MaximumConnectTime = 0xa0,
        SubscriptionIdentifiersNotSupported = 0xa1,
        WildcardSubscriptionsNotSupported = 0xa2,
    }

    impl DecodeReasonCode<DISCONNECT, ReasonCodeError> for DISCONNECT {
        fn decode(reason_code: u8) -> Result<DISCONNECT, ReasonCodeError> {
            let ret = match reason_code {
                0x00 => DISCONNECT::NormalDisconnection,
                0x04 => DISCONNECT::DisconnectWithWillMessage,
                0x80 => DISCONNECT::UnspecifiedError,
                0x81 => DISCONNECT::MalformedPacket,
                0x82 => DISCONNECT::ProtocolError,
                0x83 => DISCONNECT::ImplementationSpecificError,
                0x87 => DISCONNECT::NotAuthorized,
                0x89 => DISCONNECT::ServerBusy,
                0x8b => DISCONNECT::ServerShuttingDown,
                0x8e => DISCONNECT::KeepAliveTimeout,
                0x8f => DISCONNECT::SessionTakenOver,
                0x90 => DISCONNECT::TopicFilterInvalid,
                0x93 => DISCONNECT::ReceiveMaximumExceed,
                0x94 => DISCONNECT::TopicAliasInvalid,
                0x95 => DISCONNECT::PacketTooLarge,
                0x96 => DISCONNECT::MessageRateTooHigh,
                0x97 => DISCONNECT::QuotaExceeded,
                0x98 => DISCONNECT::AdministrativeAction,
                0x99 => DISCONNECT::PayloadFormatInvalid,
                0x9a => DISCONNECT::RetainNotSupported,
                0x9b => DISCONNECT::QOSNotSupported,
                0x9c => DISCONNECT::UseAnotherServer,
                0x9d => DISCONNECT::ServerMoved,
                0x9e => DISCONNECT::SharedSubscriptionsNotSupported,
                0x9f => DISCONNECT::ConnectionRateExceeded,
                0xa0 => DISCONNECT::MaximumConnectTime,
                0xa1 => DISCONNECT::SubscriptionIdentifiersNotSupported,
                0xa2 => DISCONNECT::WildcardSubscriptionsNotSupported,

                n => {
                    return Err(ReasonCodeError::InvalidReasonCode(
                        n,
                        String::from("DISCONNECT"),
                    ))
                }
            };

            Ok(ret)
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    #[repr(u8)]
    pub enum AUTH {
        Success = 0x00,
        ContinueAuthentication = 0x18,
        ReAuthenticate = 0x19,
    }

    impl DecodeReasonCode<AUTH, ReasonCodeError> for AUTH {
        fn decode(reason_code: u8) -> Result<AUTH, ReasonCodeError> {
            let ret = match reason_code {
                0x00 => AUTH::Success,
                0x18 => AUTH::ContinueAuthentication,
                0x19 => AUTH::ReAuthenticate,
                n => return Err(ReasonCodeError::InvalidReasonCode(n, String::from("AUTH"))),
            };
            Ok(ret)
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    #[repr(u8)]
    pub enum UNSUBACK {
        Success = 0x99,
        NoSubscriptionExisted = 0x11,
        UnspecifiedError = 0x80,
        ImplementationSpecificError = 0x83,
        NotAuthorized = 0x87,
        TopicFilterInvalid = 0x8f,
        PacketIdentifierInUse = 0x91,
    }

    impl DecodeReasonCode<UNSUBACK, ReasonCodeError> for UNSUBACK {
        fn decode(reason_code: u8) -> Result<UNSUBACK, ReasonCodeError> {
            let ret = match reason_code {
                0x99 => UNSUBACK::Success,
                0x11 => UNSUBACK::NoSubscriptionExisted,
                0x80 => UNSUBACK::UnspecifiedError,
                0x83 => UNSUBACK::ImplementationSpecificError,
                0x87 => UNSUBACK::NotAuthorized,
                0x91 => UNSUBACK::PacketIdentifierInUse,
                n => {
                    return Err(ReasonCodeError::InvalidReasonCode(
                        n,
                        String::from("UNSUBACK"),
                    ))
                }
            };

            Ok(ret)
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    #[repr(u8)]
    pub enum SUBACK {
        GrantedQos0 = 0x00,
        GrantedQos1 = 0x01,
        GrantedQos2 = 0x02,
        UnspecifiedError = 0x80,
        ImplementationSpecificError = 0x83,
        NotAuthorized = 0x87,
        TopicFilterInvalid = 0x8f,
        PacketIdentifierInUse = 0x91,
        QuotaExceeded = 0x97,
        SharedSubscriptionsNotSupported = 0x9e,
        SubscriptionIdentifiersNotSupported = 0xa1,
        WildcardSubscriptionsNotSupported = 0xa2,
    }

    impl DecodeReasonCode<SUBACK, ReasonCodeError> for SUBACK {
        fn decode(reason_code: u8) -> Result<SUBACK, ReasonCodeError> {
            let ret = match reason_code {
                0x00 => SUBACK::GrantedQos0,
                0x01 => SUBACK::GrantedQos1,
                0x02 => SUBACK::GrantedQos2,
                0x80 => SUBACK::UnspecifiedError,
                0x83 => SUBACK::ImplementationSpecificError,
                0x87 => SUBACK::NotAuthorized,
                0x8f => SUBACK::TopicFilterInvalid,
                0x91 => SUBACK::PacketIdentifierInUse,
                0x97 => SUBACK::QuotaExceeded,
                0x9e => SUBACK::SharedSubscriptionsNotSupported,
                0xa1 => SUBACK::SubscriptionIdentifiersNotSupported,
                0xa2 => SUBACK::WildcardSubscriptionsNotSupported,
                n => {
                    return Err(ReasonCodeError::InvalidReasonCode(
                        n,
                        String::from("SUBACK"),
                    ))
                }
            };

            Ok(ret)
        }
    }
}

trait Properties {
    fn packet_type(&self) -> PacketTypes;
    fn packet_type_string(&self) -> String;
    fn variable_header_properties(&self) -> &Option<Vec<Property>>;

    fn set_variable_header_properties(&mut self, p: Option<Vec<Property>>);

    fn set_properties(&mut self, property: &Vec<Property>) -> Result<(), PropertyError> {
        let mut added_property: Vec<Property> = Vec::with_capacity(100);
        added_property.append(&mut property.clone());

        let invalid_properties = invalid_property_for_packet_type(&property, self.packet_type());

        if !invalid_properties.is_empty() {
            return Err(PropertyError::InvalidProperty(
                invalid_properties,
                self.packet_type_string(),
            ));
        };

        let non_unique_properties = non_unique(&property);
        if !non_unique_properties.is_empty() {
            return Err(PropertyError::PropertyAlreadyInserted(
                non_unique_properties,
                self.packet_type_string(),
            ));
        }

        let mut packet_properties: Vec<Property> = vec![];
        packet_properties.append(&mut added_property); // added_property field is empty
        let mut variable_header_properties = self.variable_header_properties();
        variable_header_properties = &None;

        if packet_properties.len() > 0 {
            let mut properties = vec![];
            properties.append(&mut packet_properties);
            let c = properties.clone();
            self.set_variable_header_properties(Some(c));
        } else {
            self.set_variable_header_properties(None);
        };

        trace!(
            "saved properties in variable are {:?}",
            variable_header_properties
        );
        Ok(())
    }
}
