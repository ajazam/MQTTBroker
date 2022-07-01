pub mod mqtt_broker {
    use crate::encode;
    use crate::mqttbroker::mqtt_broker::types::{
        BinaryDataT, ByteT, FourByteIntegerT, TwoByteIntegerT, Utf8stringPairT,
        VariableByteIntegerT,
    };
    use std::convert::TryFrom;
    use std::hash::Hash;

    use crate::mqttbroker::mqtt_broker::PropertyIdentifiers::{
        AssignedClientIdentifier, AuthenticationData, AuthenticationMethod, ContentType,
        CorrelationData, MaximumPacketSize, MaximumQos, MessageExpiryInterval,
        PayloadFormatIndicator, ReasonString, ReceiveMaximum, RequestProblemInformation,
        RequestResponseInformation, ResponseInformation, ResponseTopic, RetainAvailable,
        ServerKeepAlive, ServerReference, SessionExpiryInterval, SharedSubscriptionAvailable,
        SubscriptionIdentifier, SubscriptionIdentifierAvailable, TopicAlias, TopicAliasMaximum,
        UserProperty, WildcardSubscriptionAvailable, WillDelayInterval,
    };
    use anyhow::Result;
    use bytes::{BufMut, BytesMut};

    pub mod types {
        #[derive(Debug, PartialEq, Eq, Hash, Clone)]
        pub struct Utf8stringPairT {
            pub key: String,
            pub value: String,
        }
        pub type ByteT = u8;
        pub type FourByteIntegerT = u32;
        pub type BinaryDataT = Vec<u8>;
        pub type TwoByteIntegerT = u16;
        pub type VariableByteIntegerT = u32; // max is 268_435_455

        pub const MAX_VARIABLE_BYTE_INTEGER: u32 = 268_435_455;
    }

    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    pub enum PropertyElement {
        Byte { value: ByteT },
        FourByteInteger { value: FourByteIntegerT },
        UTF8EncodedString { value: String },
        BinaryData { value: BinaryDataT },
        TwoByteInteger { value: TwoByteIntegerT },
        UTF8StringPair { value: Utf8stringPairT },
        VariableByteInteger { value: VariableByteIntegerT },
    }
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub struct Property {
        pub element_value: PropertyElement,
        pub property_identifier: u8,
    }

    impl Property {
        pub fn new(ev: PropertyElement, pi: u8) -> Self {
            Property {
                element_value: ev,
                property_identifier: pi,
            }
        }
    }

    impl Property {
        pub fn encode(&self, encoded: &mut Vec<u8>) {
            encoded.put_u8(self.property_identifier);
            match self.element_value {
                PropertyElement::Byte { ref value } => {
                    encoded.put_u8(*value);
                }

                PropertyElement::FourByteInteger { ref value } => {
                    encoded.put_u32(*value);
                }

                PropertyElement::UTF8EncodedString { ref value } => {
                    let mut encoded_bytes = BytesMut::with_capacity(200);
                    encode::utf8_encoded_string(value, &mut encoded_bytes);
                    encoded.put(encoded_bytes);
                }

                PropertyElement::BinaryData { ref value } => {
                    let mut encoded_bytes = BytesMut::with_capacity(200);
                    let src = BytesMut::from(value.as_slice());
                    encode::binary_data(&src, &mut encoded_bytes);
                    encoded.put(encoded_bytes);
                }

                PropertyElement::TwoByteInteger { ref value } => encoded.put_u16(*value),

                PropertyElement::UTF8StringPair { ref value } => {
                    let mut encoded_bytes = BytesMut::with_capacity(200);
                    encode::utf8_string_pair(&value.key, &value.value, &mut encoded_bytes);
                    encoded.put(encoded_bytes);
                }

                PropertyElement::VariableByteInteger { ref value } => {
                    let mut encoded_bytes = BytesMut::with_capacity(200);
                    encode::variable_byte_integer(*value, &mut encoded_bytes);
                    encoded.put(encoded_bytes);
                }
            }
        }
    }

    impl Into<u8> for Property {
        fn into(self) -> u8 {
            self.property_identifier
        }
    }

    #[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
    #[repr(u8)]
    pub enum PropertyIdentifiers {
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

    impl From<Property> for PropertyIdentifiers {
        fn from(p: Property) -> Self {
            PropertyIdentifiers::try_from(p.property_identifier).unwrap()
        }
    }

    impl TryFrom<u8> for PropertyIdentifiers {
        type Error = ();
        fn try_from(item: u8) -> Result<Self, Self::Error> {
            match item {
                0x01 => Ok(PayloadFormatIndicator),
                0x02 => Ok(MessageExpiryInterval),
                0x03 => Ok(ContentType),
                0x08 => Ok(ResponseTopic),
                0x09 => Ok(CorrelationData),
                0x0b => Ok(SubscriptionIdentifier),
                0x11 => Ok(SessionExpiryInterval),
                0x12 => Ok(AssignedClientIdentifier),
                0x13 => Ok(ServerKeepAlive),
                0x15 => Ok(AuthenticationMethod),
                0x16 => Ok(AuthenticationData),
                0x17 => Ok(RequestProblemInformation),
                0x18 => Ok(WillDelayInterval),
                0x19 => Ok(RequestResponseInformation),
                0x1a => Ok(ResponseInformation),
                0x1c => Ok(ServerReference),
                0x1f => Ok(ReasonString),
                0x21 => Ok(ReceiveMaximum),
                0x22 => Ok(TopicAliasMaximum),
                0x23 => Ok(TopicAlias),
                0x24 => Ok(MaximumQos),
                0x25 => Ok(RetainAvailable),
                0x26 => Ok(UserProperty),
                0x27 => Ok(MaximumPacketSize),
                0x28 => Ok(WildcardSubscriptionAvailable),
                0x29 => Ok(SubscriptionIdentifierAvailable),
                0x2a => Ok(SharedSubscriptionAvailable),
                _ => Err(()),
            }
        }
    }

    #[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
    #[repr(u8)]
    pub enum packet_types {
        CONNECT = 0x01,
        CONNACK = 0x02,
        PUBLISH = 0x03,
        PUBACK = 0x04,
        PUBREC = 0x05,
        PUBREL = 0x06,
        PUBCOMP = 0x07,
        SUBSCRIBE = 0x08,
        SUBACK = 0x09,
        UNSUBSCRIBE = 0x0a,
        UNSUBACK = 0x0b,
        PINGREQ = 0x0c,
        PINGRESP = 0x0d,
        DISCONNECT = 0x0e,
        AUTH = 0x0f,
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
        pub const NOT_AUTHORIZED: u8 = 0x87;
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
        pub const MAXIMUM_CONNECT_TIME: u8 = 0xa0;
        pub const SUBSCRIPTION_IDENTIFIERS_NOT_SUPPORTED: u8 = 0xa1;
        pub const WILDCARD_SUBSCRIPTIONS_NOT_SUPPORTED: u8 = 0xa2;
    }

    pub mod utility {
        use tracing::debug;

        use crate::mqttbroker::mqtt_broker::PropertyIdentifiers::{
            AssignedClientIdentifier, AuthenticationData, AuthenticationMethod, ContentType,
            CorrelationData, MaximumPacketSize, MaximumQos, MessageExpiryInterval,
            PayloadFormatIndicator, ReasonString, ReceiveMaximum, RequestProblemInformation,
            RequestResponseInformation, ResponseInformation, ResponseTopic, RetainAvailable,
            ServerKeepAlive, ServerReference, SessionExpiryInterval, SharedSubscriptionAvailable,
            SubscriptionIdentifier, SubscriptionIdentifierAvailable, TopicAlias, TopicAliasMaximum,
            UserProperty, WildcardSubscriptionAvailable, WillDelayInterval,
        };
        use crate::mqttbroker::mqtt_broker::{packet_types, Property, PropertyIdentifiers};
        use std::collections::{HashMap, HashSet};
        use std::convert::TryFrom;

        fn concat(mut set: HashSet<u8>, subset: &HashSet<u8>) -> HashSet<u8> {
            for x in subset {
                set.insert(*x);
            }

            set
        }

        /// Will return a list of invalid properties
        ///
        pub fn invalid_property_for_packet_type(
            properties: &[Property],
            validated_properties: Vec<PropertyIdentifiers>,
            pack_type: packet_types,
        ) -> Vec<Property> {
            let mut valid_property_identifiers: Vec<PropertyIdentifiers> = vec![];

            valid_property_identifiers.extend(validated_properties);

            let property_extension = match pack_type {
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
                _ => vec![],
            };

            valid_property_identifiers.extend(property_extension);

            let mut invalid: Vec<Property> = Vec::with_capacity(13);

            debug!("valid properties are {:?}", valid_property_identifiers);

            diff(properties, &valid_property_identifiers, &mut invalid);

            invalid
        }

        fn diff(
            left: &[Property],
            right: &Vec<PropertyIdentifiers>,
            differences: &mut Vec<Property>,
        ) {
            for property in left {
                if !right
                    .contains(&PropertyIdentifiers::try_from(property.property_identifier).unwrap())
                {
                    differences.push(property.clone());
                }
            }
        }

        pub fn valid_properties_for_will() -> Vec<PropertyIdentifiers> {
            vec![
                WillDelayInterval,
                PayloadFormatIndicator,
                MessageExpiryInterval,
                ContentType,
                ResponseTopic,
                CorrelationData,
                UserProperty,
            ]
        }

        fn valid_properties_for_connect_packet() -> Vec<PropertyIdentifiers> {
            vec![
                SessionExpiryInterval,
                AuthenticationMethod,
                AuthenticationData,
                RequestProblemInformation,
                RequestResponseInformation,
                ReceiveMaximum,
                TopicAliasMaximum,
                UserProperty,
                MaximumPacketSize,
            ]
        }

        fn valid_properties_for_connack_packet() -> Vec<PropertyIdentifiers> {
            vec![
                SessionExpiryInterval,
                AssignedClientIdentifier,
                ServerKeepAlive,
                AuthenticationMethod,
                AuthenticationData,
                ResponseInformation,
                ServerReference,
                ReasonString,
                ReceiveMaximum,
                TopicAliasMaximum,
                MaximumQos,
                RetainAvailable,
                UserProperty,
                MaximumPacketSize,
                WildcardSubscriptionAvailable,
                SubscriptionIdentifierAvailable,
                SharedSubscriptionAvailable,
            ]
        }

        fn valid_properties_for_publish_packet() -> Vec<PropertyIdentifiers> {
            vec![
                PayloadFormatIndicator,
                MessageExpiryInterval,
                ContentType,
                ResponseTopic,
                CorrelationData,
                SubscriptionIdentifier,
                TopicAlias,
                UserProperty,
            ]
        }

        fn valid_properties_for_puback_packet() -> Vec<PropertyIdentifiers> {
            vec![ReasonString, UserProperty]
        }

        fn valid_properties_for_pubrec_packet() -> Vec<PropertyIdentifiers> {
            vec![ReasonString, UserProperty]
        }

        fn valid_properties_for_pubrel_packet() -> Vec<PropertyIdentifiers> {
            vec![ReasonString, UserProperty]
        }

        fn valid_properties_for_pubcomp_packet() -> Vec<PropertyIdentifiers> {
            vec![ReasonString, UserProperty]
        }

        fn valid_properties_for_subscribe_packet() -> Vec<PropertyIdentifiers> {
            vec![SubscriptionIdentifier, UserProperty]
        }

        fn valid_properties_for_suback_packet() -> Vec<PropertyIdentifiers> {
            vec![ReasonString, UserProperty]
        }

        fn valid_properties_for_unsubscribe_packet() -> Vec<PropertyIdentifiers> {
            vec![UserProperty]
        }

        fn valid_properties_for_unsuback_packet() -> Vec<PropertyIdentifiers> {
            vec![ReasonString, UserProperty]
        }

        fn valid_properties_for_disconnect_packet() -> Vec<PropertyIdentifiers> {
            vec![
                SessionExpiryInterval,
                ServerReference,
                ReasonString,
                UserProperty,
            ]
        }

        fn valid_properties_for_auth_packet() -> Vec<PropertyIdentifiers> {
            vec![
                AuthenticationMethod,
                AuthenticationData,
                ReasonString,
                UserProperty,
            ]
        }

        fn packet_identifier_present(mqtt_control_packet: packet_types, qos: u8) -> bool {
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
            }
        }
        ///
        /// checks whether the properties are unique, or not. Not sure if this is required as
        pub fn check_for_non_unique_properties(props: &Vec<Property>) -> Vec<Property> {
            let mut prop_count: HashMap<Property, u8> = HashMap::new();
            for p in props {
                prop_count.insert(p.clone(), 1);
                let count = prop_count.entry(p.clone()).or_insert(0);
                *count += 1;
            }

            prop_count.retain(|k, v| k.property_identifier != UserProperty as u8);

            prop_count.retain(|_, v| *v > 1);
            prop_count.into_keys().collect()
        }

        ///
        /// Returns whether the property has been successfully added. If not then it already exists
        /// and the property is a duplicate
        ///  # Arguments
        ///
        /// * `props` - List of properties
        /// * `to_add` - Property to add
        ///
        pub fn add_property(props: &mut Vec<Property>, mut to_add: Property) -> bool {
            for p in props.iter() {
                if p.property_identifier == to_add.property_identifier
                    && p.property_identifier != UserProperty as u8
                {
                    return false;
                }
            }

            // add to list if not property not already exists or is userproperty property
            props.push(to_add);

            true
        }

        #[cfg(test)]
        mod tests {
            use crate::mqttbroker::mqtt_broker::utility::add_property;
            use crate::mqttbroker::mqtt_broker::{Property, PropertyElement, PropertyIdentifiers};
            use test_log::test;
            #[test]
            fn test_add_property_with_duplicate_property() {
                let mut properties = vec![Property {
                    element_value: PropertyElement::Byte { value: 0x01 },
                    property_identifier: PropertyIdentifiers::SubscriptionIdentifier as u8,
                }];
                let prop_to_add = Property {
                    element_value: PropertyElement::Byte { value: 0x01 },
                    property_identifier: PropertyIdentifiers::SubscriptionIdentifier as u8,
                };
                assert!(!add_property(&mut properties, prop_to_add))
            }

            #[test]
            fn test_add_property_with_unique_property() {
                let mut properties = vec![Property {
                    element_value: PropertyElement::Byte { value: 0x01 },
                    property_identifier: PropertyIdentifiers::SubscriptionIdentifier as u8,
                }];
                let prop_to_add = Property {
                    element_value: PropertyElement::Byte { value: 0x01 },
                    property_identifier: PropertyIdentifiers::UserProperty as u8,
                };
                assert!(add_property(&mut properties, prop_to_add))
            }

            #[test]
            fn test_add_property_with_duplicate_userproperty() {
                let mut properties = vec![Property {
                    element_value: PropertyElement::Byte { value: 0x01 },
                    property_identifier: PropertyIdentifiers::UserProperty as u8,
                }];
                let prop_to_add = Property {
                    element_value: PropertyElement::Byte { value: 0x01 },
                    property_identifier: PropertyIdentifiers::UserProperty as u8,
                };
                assert!(add_property(&mut properties, prop_to_add))
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
        use crate::decode::{binary, property, utf8_string, varint};
        use crate::encode::{utf8_encoded_string, variable_byte_integer};
        use crate::mqttbroker::mqtt_broker::packets::error::PropertyError;
        use crate::mqttbroker::mqtt_broker::utility::{
            check_for_non_unique_properties, invalid_property_for_packet_type,
            valid_properties_for_will,
        };
        use crate::mqttbroker::mqtt_broker::{packet_types, PropertyIdentifiers};
        use crate::mqttbroker::mqtt_broker::{Property, ReasonCode};
        use anyhow::Error;
        use bytes::{Buf, BufMut, BytesMut};
        use std::collections::HashSet;
        use std::io;
        use tracing::debug;

        mod error {
            use crate::mqttbroker::mqtt_broker::Property;
            use thiserror::Error;

            #[derive(Error, Debug)]
            pub enum PropertyError {
                #[error("property {0:?} has already been inserted, you are trying to insert a duplicate copy into {1}")]
                PropertyAlreadyInserted(Vec<Property>, String),
                // #[error("property {0} is not valid for Will T opic of CONNECT Payload")]
                // InvalidConnectPayloadWillProperty(String),
                // #[error("property {0} is not valid for Properties of CONNECT Variable Header")]
                // InvalidConnectVariableHeaderProperty(String),
                #[error("property {0:?} is not valid for packet type {1}")]
                InvalidProperty(Vec<Property>, String),
            }
        }

        pub enum ControlPacket {
            Connect {
                packet_type: Option<u8>,
                protocol_name: Option<String>,
                protocol_version: Option<u8>,
                connect_flags: Option<u8>,
                keep_alive: Option<u16>,
                variable_header_properties: Option<Vec<Property>>,
                client_identifier: Option<String>,
                will_properties: Option<Vec<Property>>,
                will_topic: Option<String>,
                will_payload: Option<Vec<u8>>,
                username: Option<String>,
                password: Option<String>,
            },
        }

        use crate::mqttbroker::mqtt_broker::packets::ConnectPacketBuildError::{
            WillFlagNotSet, WillPayLoadNotSet, WillTopicNotSet,
        };
        use thiserror::Error;

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
            pub const CLEAN_START: u8 = 1;
            pub const WILL_FLAG: u8 = 2;
            pub const WILL_QOS_MASK: u8 = 4 + 8;
            pub const WILL_QOS0: u8 = 0;
            pub const WILL_QOS1: u8 = 1 << 2;
            pub const WILL_QOS2: u8 = 2 << 2;
            pub const WILL_RETAIN: u8 = 1 << 5;
            pub const USER_NAME_FLAG: u8 = 1 << 7;
            pub const PASSWORD_FLAG: u8 = 1 << 6;
        }

        #[derive(Default, Debug, PartialEq, Clone)]
        pub struct ConnectPacket {
            // fixed header
            pub packet_type: u8,
            pub packet_type_flags: u8,
            // variable header
            pub protocol_name: String,
            pub protocol_version: u8,
            pub keep_alive: u16,
            pub variable_header_properties: Vec<Property>,
            // pub connect_flags_will_retain: bool,
            // pub connect_flags_will_qos: u8,
            // pub connect_flags_clean_start: bool,
            // pub connect_flags_will_flag: bool,
            connect_flags: u8,
            // payload
            pub client_id: String,
            pub will_properties: Vec<Property>,
            pub will_topic: String,
            pub will_payload: Vec<u8>,
            pub username: String,
            pub password: String,
        }

        impl ConnectPacket {
            pub fn will_flag(&self) -> bool {
                self.connect_flags & connect_flags::WILL_FLAG > 0
            }

            pub fn will_retain(&self) -> bool {
                (self.connect_flags & connect_flags::WILL_RETAIN) > 0
            }
            pub fn set_will_retain(&mut self, retain: bool) {
                if retain {
                    self.connect_flags |= connect_flags::WILL_RETAIN;
                    return;
                }

                self.connect_flags &= !connect_flags::WILL_RETAIN
            }

            pub fn will_qos(&self) -> u8 {
                (self.connect_flags & connect_flags::WILL_QOS_MASK) >> 3
            }

            pub fn set_will_qos(&mut self, qos: u8) {
                let new_qos = if qos > 2 { 2 } else { qos };
                self.connect_flags &= !connect_flags::WILL_QOS_MASK | new_qos << 3
            }

            pub fn clean_start(&self) -> bool {
                (self.connect_flags & connect_flags::CLEAN_START) > 0
            }

            pub fn set_clean_start(&mut self, clean_start: bool) {
                self.connect_flags &= !connect_flags::CLEAN_START | if clean_start { 1 } else { 0 }
            }
        }
        #[derive(Debug)]
        pub struct ConnectPacketBuilder {
            pub connect_packet: ConnectPacket,
        }

        use tracing::instrument;

        impl ConnectPacketBuilder {
            pub fn new() -> Self {
                ConnectPacketBuilder {
                    connect_packet: ConnectPacket {
                        packet_type: packet_types::CONNECT as u8,
                        packet_type_flags: 0,
                        protocol_name: String::from("MQTT"),
                        protocol_version: 5u8,
                        keep_alive: 0,
                        variable_header_properties: vec![],
                        connect_flags: 0,
                        client_id: "".to_string(),
                        will_properties: vec![],
                        will_topic: String::from(""),
                        will_payload: vec![],
                        username: String::from(""),
                        password: String::from(""),
                    },
                }
            }

            pub fn packet_type(mut self, pt: u8) -> Self {
                self.connect_packet.packet_type = pt;
                self
            }

            pub fn protocol_name(mut self, pn: String) -> Self {
                self.connect_packet.protocol_name = pn;
                self
            }

            pub fn keep_alive(mut self, keep_alive: u16) -> Self {
                self.connect_packet.keep_alive = keep_alive;
                self
            }

            pub fn set_packet_properties(
                property: Vec<Property>,
                will_flag: bool,
            ) -> Result<Vec<Property>, PropertyError> {
                let mut added_property: Vec<Property> = Vec::with_capacity(100);

                let mut valid_will_properties: Vec<PropertyIdentifiers> = vec![];

                if will_flag {
                    valid_will_properties = valid_properties_for_will();
                }

                let invalid_properties = invalid_property_for_packet_type(
                    &property,
                    valid_will_properties,
                    packet_types::CONNECT,
                );

                if !invalid_properties.is_empty() {
                    return Err(PropertyError::InvalidProperty(
                        invalid_properties,
                        String::from("CONNECT"),
                    ));
                };

                let non_unique_properties = check_for_non_unique_properties(&property);
                if !non_unique_properties.is_empty() {
                    return Err(PropertyError::PropertyAlreadyInserted(
                        non_unique_properties,
                        String::from("CONNECT"),
                    ));
                }

                let mut packet_properties: Vec<Property> = vec![];
                packet_properties.append(&mut added_property);

                Ok(packet_properties)
            }

            pub fn connect_flags_with_will_retain_flag(mut self, b: bool) -> Self {
                self.connect_packet.connect_flags != 4;
                self
            }

            pub fn set_will_retain(&mut self, retain: bool) {
                if retain {
                    self.connect_packet.connect_flags |= connect_flags::WILL_RETAIN;
                    return;
                }

                self.connect_packet.connect_flags &= !connect_flags::WILL_RETAIN
            }

            // pub fn connect_flags_with_will_qos(mut self, q: u8) -> Self {
            //     self.connect_packet.connect_flags_will_qos = if q > 2 { 2 } else { q };
            //     self
            // }
            //

            pub fn set_will_qos(&mut self, qos: u8) {
                let new_qos = if qos > 2 { 2 } else { qos };
                self.connect_packet.connect_flags &= !connect_flags::WILL_QOS_MASK | new_qos << 3
            }

            // pub fn connect_flags_with_clean_start(mut self, b: bool) -> Self {
            //     self.connect_packet.connect_flags_clean_start = b;
            //     self
            // }
            //
            // pub fn connect_flags_with_will_flag(mut self, b: bool) -> Self {
            //     self.connect_packet.connect_flags_will_flag = b;
            //     self
            // }

            pub fn client_id(mut self, ci: String) -> Self {
                self.connect_packet.client_id = ci;
                self
            }

            fn will_properties(
                mut self,
                assigned_will_properties: Vec<Property>,
            ) -> Result<Self, PropertyError> {
                let mut properties: Vec<Property> = vec![];

                for p in &assigned_will_properties {
                    properties.push(p.clone())
                }

                // check for invalid
                let invalid_will_properties =
                    invalid_property_for_packet_type(&properties, vec![], packet_types::CONNECT);

                // will properties are only used in the CONNECT packet
                if !invalid_will_properties.is_empty() {
                    return Err(PropertyError::InvalidProperty(
                        invalid_will_properties,
                        String::from("CONNECT"),
                    ));
                };

                // check for duplicates
                let non_unique_properties =
                    check_for_non_unique_properties(&assigned_will_properties);
                if !non_unique_properties.is_empty() {
                    return Err(PropertyError::PropertyAlreadyInserted(
                        non_unique_properties,
                        String::from("CONNECT"),
                    ));
                }

                self.connect_packet.will_properties = assigned_will_properties;
                Ok(self)
            }

            fn will_topic(mut self, topic: String) -> Self {
                self.connect_packet.will_topic = topic;
                self
            }

            fn will_payload(mut self, will_payload: Vec<u8>) -> Self {
                self.connect_packet.will_payload = will_payload;
                self
            }

            pub fn will_message(
                mut self,
                will_properties: Vec<Property>,
                will_topic: String,
                will_payload: Vec<u8>,
            ) -> Self {
                self.connect_packet.will_properties = will_properties;
                self.connect_packet.will_topic = will_topic;
                self.connect_packet.will_payload = will_payload;
                self.connect_packet.connect_flags |= connect_flags::WILL_FLAG;

                self
            }

            pub fn username(mut self, username: String) -> Self {
                self.connect_packet.username = username;
                self
            }

            pub fn password(mut self, password: String) -> Self {
                self.connect_packet.password = password;
                self
            }

            pub fn generate_connect_flags(connect_packet: &ConnectPacket) -> u8 {
                let mut connect_flags = 0u8;
                if !connect_packet.username.is_empty() {
                    connect_flags |= connect_flags::USER_NAME_FLAG;
                }

                if !connect_packet.password.is_empty() {
                    connect_flags |= connect_flags::PASSWORD_FLAG;
                }

                connect_flags |= connect_packet.connect_flags;
                connect_flags
            }

            fn will_properties_are_valid(&self) -> anyhow::Result<()> {
                let is_will_flag_not_set_and_will_properties_set =
                    !self.connect_packet.connect_flags == connect_flags::WILL_FLAG
                        && (!self.connect_packet.will_properties.is_empty()
                            || !self.connect_packet.will_topic.is_empty()
                            || !self.connect_packet.will_payload.is_empty());

                if is_will_flag_not_set_and_will_properties_set {
                    return Err(WillFlagNotSet.into());
                }

                let is_will_flag_set_and_topic_not_set =
                    self.connect_packet.will_flag() && !self.connect_packet.will_topic.is_empty();

                if is_will_flag_set_and_topic_not_set {
                    return Err(WillTopicNotSet.into());
                }

                let is_will_flag_set_and_will_payload_not_set =
                    self.connect_packet.will_flag() && !self.connect_packet.will_payload.is_empty();

                if is_will_flag_set_and_will_payload_not_set {
                    return Err(WillPayLoadNotSet.into());
                }

                Ok(())
            }

            #[instrument]
            pub fn build(self) -> Result<Vec<u8>, io::Error> {
                let _ = self.will_properties_are_valid();

                // start of fixed header >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
                // fixed header // control packet type, Remaining length (variable header + payload)

                let mut fixed_header = BytesMut::with_capacity(5);
                fixed_header.put_u8(
                    ((self.connect_packet.packet_type & 0x0f) << 4)
                        + (self.connect_packet.packet_type_flags & 0x0f),
                );

                // fixed_header // add remaining length here

                // end of fixed header <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

                // start of variable header >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

                let variable_header = self.generate_variable_header();

                // end of variable header <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

                let (payload, payload_size, variable_header_size) =
                    self.generate_payload(&variable_header);

                // start of making actual bytes for packet
                let fixed_header_remaining_length = variable_header_size + payload_size;

                let mut connect_packet = BytesMut::with_capacity(200);
                variable_byte_integer(fixed_header_remaining_length as u32, &mut fixed_header)
                    .unwrap();
                debug!("++++++++++++++ fixed header is {:?}", fixed_header);
                debug!("++++++++++++++ variable header is {:?}", variable_header);
                debug!("++++++++++++++ payload is {:?}", payload);
                println!("JELLO WORLD");
                connect_packet.put(fixed_header);
                connect_packet.put(variable_header);
                connect_packet.put(payload);

                // end of making actual bytes for packet

                Ok(connect_packet.to_vec())
            }

            fn generate_payload(self, variable_header: &BytesMut) -> (BytesMut, usize, usize) {
                // payload details
                // client identifier, Will properties, will topic, will payload, username, password
                let mut payload = BytesMut::with_capacity(200);
                utf8_encoded_string(&self.connect_packet.client_id, &mut payload);
                // will properties

                let encoded_will_properties =
                    encode_properties(&self.connect_packet.will_properties);

                let mut encoded_will_properties_size = BytesMut::with_capacity(4);
                variable_byte_integer(
                    encoded_will_properties.len() as u32,
                    &mut encoded_will_properties_size,
                )
                .unwrap();

                payload.put(encoded_will_properties_size);
                payload.put(encoded_will_properties.as_slice());

                // will topic
                if self.connect_packet.will_topic.len() > 0 {
                    payload.put_u16(self.connect_packet.will_topic.len() as u16);
                    payload.put(self.connect_packet.will_topic.as_bytes());
                }

                // will payload
                if self.connect_packet.will_payload.len() > 0 {
                    payload.put_u16(self.connect_packet.will_payload.len() as u16);
                    payload.put(self.connect_packet.will_payload.as_slice());
                }

                //username
                if self.connect_packet.username.len() > 0 {
                    payload.put(self.connect_packet.username.as_bytes());
                }

                //password
                if self.connect_packet.password.len() > 0 {
                    payload.put(self.connect_packet.password.as_bytes());
                }

                let payload_size = payload.len();
                let variable_header_size = variable_header.len();

                // end of payload <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
                (payload, payload_size, variable_header_size)
            }

            fn generate_variable_header(&self) -> BytesMut {
                // variable header // Protocol Name, protocol level, connect flags, keep alive and properties
                let mut variable_header = BytesMut::with_capacity(200);

                utf8_encoded_string(
                    self.connect_packet.protocol_name.as_ref(),
                    &mut variable_header,
                );

                variable_header.put_u8(self.connect_packet.protocol_version);

                variable_header.put_u8(ConnectPacketBuilder::generate_connect_flags(
                    &self.connect_packet,
                ));

                variable_header.put_u16(self.connect_packet.keep_alive);

                // Connect Properties
                let encoded_variable_header_properties =
                    if !&self.connect_packet.variable_header_properties.is_empty() {
                        encode_properties(&self.connect_packet.variable_header_properties)
                    } else {
                        vec![]
                    };

                let mut encoded_variable_header_properties_size: BytesMut =
                    BytesMut::with_capacity(4);
                variable_byte_integer(
                    encoded_variable_header_properties.len() as u32,
                    &mut encoded_variable_header_properties_size,
                )
                .unwrap();
                variable_header
                    .put_slice(encoded_variable_header_properties_size.iter().as_slice());
                variable_header.put_slice(encoded_variable_header_properties.as_slice());
                variable_header
            }
        }

        // Can use this from all the packets
        pub fn encode_properties(props: &Vec<Property>) -> Vec<u8> {
            let mut properties_vec: Vec<u8> = Vec::with_capacity(200);

            for prop_item in props {
                prop_item.encode(&mut properties_vec);
            }

            properties_vec
        }

        #[cfg(test)]
        mod encode_test {
            use crate::mqttbroker::mqtt_broker::packets::ConnectPacketBuilder;

            #[test]
            pub fn will_properties_not_set() {
                let packet = ConnectPacketBuilder::new();

                assert_eq!(false, packet.connect_packet.will_flag())
            }
        }

        pub struct ConnAck {
            //fixed header
            packet_type: u8,

            // variable header
            connect_ack_flags: u8,
            reason_code: ReasonCode,
            property: Vec<Property>,
            //payload
            // no payload
        }

        pub struct Publish {
            //fixed header
            packet_type: u8,

            //variable_header
            topic_name: String,
            packet_id: u16,
            property: Vec<Property>,

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
            property: Option<Vec<Property>>,
            //payload
            //no payload
        }

        pub struct PubRec {
            //fixed header
            packet_type: u8,

            //variable header
            packet_id: u16,
            reason_code: ReasonCode,
            property: Vec<Property>, // if the remaining length is 4 then property length is zero

                                     //payload
                                     //no payload
        }

        pub struct PubRel {
            //fixed header
            packet_type: u8,

            //variable header
            packet_id: u16,
            reason_code: ReasonCode,
            property: Vec<Property>, // if the remaining length is 4 then property length is zero

                                     //payload
                                     //no payload
        }

        pub struct PubComp {
            //fixed header
            packet_type: u8,

            //variable header
            packet_id: u16,
            reason_code: ReasonCode,
            property: Vec<Property>, // if the remaining length is 4 then property length is zero

                                     //payload
                                     //no payload
        }

        pub struct Subscribe {
            //fixed header
            packet_type: u8,

            //variable header
            packet_id: u16,
            property: Vec<Property>,

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

        pub trait Decoder {
            fn decode(bytes: &mut BytesMut) -> (Option<ConnectPacket>, Option<ReasonCode>);
        }

        impl Decoder for ConnectPacket {
            fn decode(bytes: &mut BytesMut) -> (Option<ConnectPacket>, Option<ReasonCode>) {
                debug!("bytes left at start {}", bytes.len());
                let packet_type = bytes.get_u8();
                debug!("bytes left after pack_type {}", bytes.len());

                let packet_size = varint(bytes).unwrap();

                debug!("bytes left after packet_size {}", bytes.len());

                let protocol_name = utf8_string(String::from("protocol name"), bytes).unwrap();

                let protocol_version = bytes.get_u8();

                let connect_flags = bytes.get_u8();

                let keep_alive = bytes.get_u16();

                let variable_header_properties = property(bytes).unwrap();
                debug!(
                    "bytes left after variable header properties {}",
                    bytes.len()
                );

                // need to check for duplicates in variable header properties
                // user property can be duplicated
                // other properties can't be duplicated

                let client_identifier =
                    utf8_string(String::from("client identifier"), bytes).unwrap();
                debug!("bytes left are client identifier {}", bytes.len());

                let is_will_flag = (connect_flags & connect_flags::WILL_FLAG) > 0;
                debug!(
                    "current will flag value is {}, raw value is {}",
                    is_will_flag, connect_flags
                );

                let will_properties: Option<Vec<Property>> = if is_will_flag {
                    // Will flag is set
                    Some(property(bytes).unwrap())
                } else {
                    None
                };

                debug!("bytes left are will_properties {}", bytes.len());

                let will_topic: String = if is_will_flag {
                    debug!("decoding will topic");
                    utf8_string(String::from("will_topic"), bytes).unwrap()
                } else {
                    String::from("")
                };

                debug!("bytes left after will_topic {}", bytes.len());

                let will_payload: Vec<u8> = if is_will_flag {
                    binary(String::from("payload"), bytes).unwrap()
                } else {
                    vec![]
                };

                let is_username_flag = connect_flags & connect_flags::USER_NAME_FLAG > 0;

                let username = if is_username_flag {
                    utf8_string(String::from("username"), bytes).unwrap()
                } else {
                    String::from("")
                };

                let is_password_flag = connect_flags & connect_flags::PASSWORD_FLAG > 0;

                let password = if is_password_flag {
                    utf8_string(String::from("password"), bytes).unwrap()
                } else {
                    String::from("")
                };
                // Successful return
                (
                    Some(ConnectPacket {
                        packet_type: packet_type >> 4,
                        packet_type_flags: 0,
                        protocol_name,
                        protocol_version,
                        connect_flags,
                        keep_alive,
                        variable_header_properties,
                        will_properties: will_properties.unwrap_or_default(),
                        will_topic,
                        will_payload,
                        username,
                        password,
                        client_id: client_identifier,
                    }),
                    None,
                )
            }
        }
    }

    mod server {

        #[cfg(test)]
        mod test {
            use crate::mqttbroker::mqtt_broker::packets::{
                ConnectPacket, ConnectPacketBuilder, Decoder,
            };
            use bytes::BytesMut;
            use tracing::debug;

            #[test]
            fn test_connect() {
                // generate packet
                let original_connect_packet = ConnectPacketBuilder::new().will_message(
                    vec![],
                    "/topic".to_string(),
                    vec![77, 66, 12],
                );
                let original_connect_packet_clone = original_connect_packet.connect_packet.clone();
                let encoded_packet = original_connect_packet.build().unwrap();
                debug!("encoded packet is {:?}", encoded_packet);
                //decode packet
                let decoded_connect_packet =
                    ConnectPacket::decode(&mut BytesMut::from(encoded_packet.as_slice()))
                        .0
                        .unwrap();

                assert_eq!(original_connect_packet_clone, decoded_connect_packet);
                debug!("original packet is {:?}", original_connect_packet_clone);
                debug!("decoded packet is {:?}", decoded_connect_packet);
            }
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
