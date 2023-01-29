pub mod builder;
mod deser;
mod validation;

use crate::decode::{binary, decode_property, property, utf8_string, varint};
use crate::encode::{utf8_encoded_string, variable_byte_integer};
use crate::packets::connect::builder::ConnectBuilder;
use crate::packets::{
    connect_flags, encode_properties, encode_properties_to_vec, BuilderLifecycle, Decoder, Encoder,
    GeneratePacketParts, PacketTypes, Properties,
};
use crate::primitive_types::VariableByteInteger;
use crate::properties::{invalid_property, non_unique, valid_properties_for_will, Property};
use bytes::{Buf, BufMut, BytesMut};
use pretty_hex::*;
use std::io::Error;
use tracing::trace;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Connect {
    /// fixed header
    pub packet_type: u8,
    /// fixed header
    ///
    pub packet_type_low_nibble: u8,

    // variable header
    pub protocol_name: String,
    pub protocol_version: u8,
    pub keep_alive: u16,
    pub variable_header_properties: Option<Vec<Property>>,
    pub connect_flags: u8,

    // payload
    pub client_id: String,
    pub will_properties: Option<Vec<Property>>,
    pub will_topic: Option<String>,
    pub will_payload: Option<Vec<u8>>,
    pub username: Option<String>,
    pub password: Option<String>,
}

// impl block for reading properties
impl Connect {
    pub fn username_flag(&self) -> bool {
        self.connect_flags & connect_flags::USER_NAME_FLAG > 0
    }

    pub fn password_flag(&self) -> bool {
        self.connect_flags & connect_flags::PASSWORD_FLAG > 0
    }

    pub fn will_retain_flag(&self) -> bool {
        (self.connect_flags & connect_flags::WILL_RETAIN) > 0
    }

    pub fn will_qos_flag(&self) -> u8 {
        (self.connect_flags & connect_flags::WILL_QOS_MASK) >> 3
    }

    pub fn will_flag(&self) -> bool {
        (self.connect_flags & connect_flags::WILL_FLAG) == connect_flags::WILL_FLAG
    }

    pub fn clean_start_flag(&self) -> bool {
        self.connect_flags & connect_flags::CLEAN_START > 0
    }

    pub fn builder() -> ConnectBuilder {
        ConnectBuilder::default()
    }
}

impl Default for Connect {
    fn default() -> Self {
        Connect {
            packet_type: PacketTypes::Connect as u8,
            packet_type_low_nibble: 0,
            protocol_name: String::from("MQTT"),
            protocol_version: 5u8,
            keep_alive: 0,
            variable_header_properties: None,
            connect_flags: 0,
            client_id: String::new(),
            will_properties: None,
            will_topic: None,
            will_payload: None,
            username: None,
            password: None,
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::packets::connect::{Connect, ConnectBuilder};
    use crate::packets::error::PropertyError;
    use crate::packets::{BuilderLifecycle, Decoder, Encoder, Properties};
    use crate::primitive_types::{Byte, FourByteInteger};
    use crate::properties::{Property, PropertyIdentifier};
    use pretty_hex::*;
    use std::collections::HashSet;
    use std::iter::FromIterator;
    use tracing::{trace, Level};
    use tracing_subscriber::FmtSubscriber;

    #[test]
    fn should_have_valid_variable_header_properties_for_connect_packet() {
        let mut packet = ConnectBuilder::new();
        let props = vec![
            Property::PayloadFormatIndicator(Byte(1)),
            Property::SessionExpiryInterval(FourByteInteger(100)),
            Property::SharedSubscriptionAvailable(Byte(1)),
        ];

        let result = packet.set_properties(&props);

        if let Err(PropertyError::InvalidProperty(..)) = result {
            assert!(true)
        } else {
            assert!(false)
        }
    }

    #[test]
    fn should_have_invalid_variable_header_properties_for_connect_packets() {
        // let subscriber = FmtSubscriber::builder.rs()
        //     .with_max_level(Level::TRACE)
        //     .finish();
        //
        // tracing::subscriber::set_global_default(subscriber)
        //     .expect("setting default subscriber failed");

        let mut packet = ConnectBuilder::new();
        let props = vec![
            Property::WillDelayInterval(FourByteInteger(1)),
            Property::SessionExpiryInterval(FourByteInteger(100)),
            Property::SharedSubscriptionAvailable(Byte(1)),
        ];
        println!("props before set_properties are {props:?}");
        let res = packet.set_properties(&props);
        println!("props after {props:?}");
        trace!("res is {:?}", res);

        if let Err(PropertyError::InvalidProperty(invalid_props, packet_type)) = res {
            assert_eq!(
                vec![
                    Property::WillDelayInterval(FourByteInteger(1)),
                    Property::SharedSubscriptionAvailable(Byte(1))
                ],
                invalid_props
            );
        } else {
            assert!(false)
        }
    }

    #[test]
    fn should_encode_decode_connect_packet() {
        // let subscriber = FmtSubscriber::builder.rs()
        //     .with_max_level(Level::TRACE)
        //     .finish();
        //
        // tracing::subscriber::set_global_default(subscriber)
        //     .expect("setting default subscriber failed");

        let mut original_packet = ConnectBuilder::new();

        // variable header fields - start
        let props = vec![
            Property::PayloadFormatIndicator(Byte(1)),
            Property::SessionExpiryInterval(FourByteInteger(231)),
            Property::SharedSubscriptionAvailable(Byte(1)),
        ];

        original_packet = original_packet.set_keep_alive(1000);
        original_packet.set_properties(&props);
        // variable header fields - end

        // payload fields - start
        original_packet = original_packet.password(Some("hello".to_string()));
        original_packet = original_packet.client_id("ID".to_string());
        original_packet = original_packet.set_keep_alive(1000);
        let res = original_packet.will_message(&vec![], "topic".to_string(), vec![1, 2, 3, 4]);
        // payload fields - end

        let built_packet = original_packet.build().unwrap();

        let mut serialized_packet = Connect::encode(
            built_packet.packet_type,
            built_packet.packet_type_low_nibble,
            &built_packet,
        )
        .unwrap();

        trace!(
            "serialized packet is {:?}",
            pretty_hex(&serialized_packet.as_ref().to_vec())
        );
        let deserialed_packet = Connect::decode(&mut serialized_packet).unwrap();

        assert_eq!(built_packet, deserialed_packet);
    }
}
