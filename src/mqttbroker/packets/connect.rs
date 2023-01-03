use crate::decode::{binary, property, utf8_string, varint};
use crate::encode::{utf8_encoded_string, variable_byte_integer};
use crate::mqttbroker::packets::connect::encode_validation::will_properties_are_valid;
use crate::mqttbroker::packets::error::PropertyError;
use crate::mqttbroker::packets::{
    connect_flags, encode_properties, BuilderLifecycle, Decoder, Encoder, GeneratePacketParts,
    PacketTypes, Properties,
};
use crate::mqttbroker::primitive_types::VariableByteInteger;
use crate::mqttbroker::properties::{
    invalid_property, invalid_property_for_packet_type, non_unique, valid_properties_for_will,
    Property,
};
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
    pub packet_type_flags: u8,

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

    pub fn builder() -> Builder {
        Builder::default()
    }
}

pub mod encode_validation {
    use crate::mqttbroker::packets::connect::Connect;
    use crate::mqttbroker::packets::ConnectPacketBuildError::{
        WillFlagNotSet, WillPayLoadNotSet, WillTopicNotSet,
    };

    pub fn will_properties_are_valid(connect_packet: &Connect) -> anyhow::Result<()> {
        if is_will_flag_not_set_and_will_properties_set_error(connect_packet) {
            return Err(WillFlagNotSet.into());
        }

        if is_will_flag_set_and_will_topic_is_empty_error(connect_packet) {
            return Err(WillTopicNotSet.into());
        }

        if is_will_flag_set_and_will_payload_is_empty_error(connect_packet) {
            return Err(WillPayLoadNotSet.into());
        }

        Ok(())
    }

    fn is_will_flag_not_set_and_will_properties_set_error(connect_packet: &Connect) -> bool {
        if connect_packet.will_flag() {
            return false;
        }

        let ret_value = !connect_packet.will_flag()
            && connect_packet.will_properties.is_some()
            && connect_packet.will_properties.as_ref().is_some();

        ret_value
    }

    fn is_will_flag_set_and_will_topic_is_empty_error(connect_packet: &Connect) -> bool {
        if !connect_packet.will_flag() {
            return false;
        }

        // connect_packet.will_flag()
        //     && ((connect_packet.will_topic.is_some()
        //         && !connect_packet.will_topic.as_ref().unwrap().is_empty())
        //         || connect_packet.will_topic.is_none())

        if connect_packet.will_flag() && (connect_packet.will_topic.is_none()) {
            return true;
        }

        if connect_packet.will_flag()
            && connect_packet.will_topic.is_some()
            && connect_packet.will_topic.as_ref().unwrap().is_empty()
        {
            return true;
        }

        false
    }

    fn is_will_flag_set_and_will_payload_is_empty_error(connect_packet: &Connect) -> bool {
        if !connect_packet.will_flag() {
            return false;
        }

        if connect_packet.will_payload.is_some()
            && connect_packet.will_payload.as_ref().unwrap().is_empty()
        {
            return true;
        }

        if connect_packet.will_flag() {
            return connect_packet.will_payload.is_none();
        }

        if connect_packet.will_payload.is_none() {
            return true;
        }

        false
    }

    fn is_will_flag_not_set_and_qos_not_0_error(connect_packet: &Connect) -> bool {
        if connect_packet.will_flag() {
            return false;
        }

        !connect_packet.will_flag() && connect_packet.will_qos_flag() == 0
    }

    #[cfg(test)]
    pub mod test {
        use crate::mqttbroker::packets::connect::encode_validation::{
            is_will_flag_not_set_and_will_properties_set_error,
            is_will_flag_set_and_will_payload_is_empty_error,
            is_will_flag_set_and_will_topic_is_empty_error,
        };
        use crate::mqttbroker::packets::connect::Connect;
        use crate::mqttbroker::packets::connect_flags;
        use crate::mqttbroker::primitive_types::FourByteInteger;
        use crate::mqttbroker::properties::Property;
        use tracing::trace;

        #[test]
        fn test_is_will_flag_not_set_and_will_properties_set() {
            let connect_packet = Connect {
                will_topic: Some(String::from("hello")),
                will_payload: Some(vec![1, 2, 3]), //will_properties: Some(vec![Property::new()])..Default::default(),
                will_properties: Some(vec![Property::WillDelayInterval(FourByteInteger(400))]),
                ..Default::default()
            };

            trace!("connect_packet is {connect_packet:?}");

            //let ret = connect_packet.will_flag();

            let result = is_will_flag_not_set_and_will_properties_set_error(&connect_packet);

            assert!(result);
        }

        #[test]
        fn test_is_will_flag_set_and_topic_is_empty() {
            let connect_packet = Connect {
                connect_flags: connect_flags::WILL_FLAG,
                will_topic: Some(String::from("")),
                ..Default::default()
            };
            trace!("{:?}", connect_packet);
            assert!(is_will_flag_set_and_will_topic_is_empty_error(
                &connect_packet
            ));
        }

        #[test]
        fn test_is_will_flag_set_and_will_payload_is_empty() {
            let connect_packet = Connect {
                connect_flags: connect_flags::WILL_FLAG,
                will_payload: None,
                ..Default::default()
            };
            assert!(is_will_flag_set_and_will_payload_is_empty_error(
                &connect_packet
            ));
        }
    }
}

pub mod decode_validate {
    pub fn client_id_correctly_formatted(str: String) -> bool {
        for c in str.chars() {
            if !c.is_ascii_alphanumeric() {
                return false;
            }
        }

        true
    }
    #[cfg(test)]
    pub mod test {
        use crate::mqttbroker::packets::connect::decode_validate::client_id_correctly_formatted;

        #[test]
        fn should_client_id_correctly_formatted() {
            assert!(client_id_correctly_formatted(String::from("12345ABCabc")));
        }

        #[test]
        fn should_be_incorrectly_formatted_client_id() {
            assert!(!client_id_correctly_formatted(String::from(
                "12345ABCabc_!£$%^&*()"
            )));
        }
    }
}

impl Default for Connect {
    fn default() -> Self {
        Connect {
            packet_type: (PacketTypes::Connect as u8),
            packet_type_flags: 0,
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

#[derive(Debug, Clone, Default)]
pub struct Builder {
    pub packet: Connect,
}

impl Builder {
    //Don't need to specify version number because the code is for v5.0

    pub fn set_keep_alive(mut self, keep_alive: u16) -> Self {
        self.packet.keep_alive = keep_alive;
        self
    }

    pub fn set_will_retain(mut self, retain: bool) -> Self {
        if retain {
            self.packet.connect_flags |= connect_flags::WILL_RETAIN;
            return self;
        }

        self.packet.connect_flags &= !connect_flags::WILL_RETAIN;
        self
    }

    pub fn set_will_qos(&mut self, qos: u8) {
        let new_qos = if qos > 2 { 2 } else { qos };
        self.packet.connect_flags &= !connect_flags::WILL_QOS_MASK | new_qos << 3;
    }

    pub fn clean_start(mut self, b: bool) -> Self {
        if b {
            self.packet.connect_flags |= 2;
            return self;
        }

        self.packet.connect_flags = !2;
        self
    }

    pub fn client_id(mut self, ci: String) -> Self {
        self.packet.client_id = ci;
        self
    }

    fn will_properties(
        &mut self,
        assigned_will_properties: &Vec<Property>,
    ) -> Result<(), PropertyError> {
        let mut properties: Vec<Property> = vec![];

        for p in assigned_will_properties {
            properties.push(p.clone())
        }

        // check for invalid
        let mut invalid_will_properties: Vec<Property> = vec![];
        invalid_property(
            &assigned_will_properties,
            valid_properties_for_will().as_slice(),
            &mut invalid_will_properties,
        );

        // will properties are only used in the CONNECT packets
        if !invalid_will_properties.is_empty() {
            return Err(PropertyError::InvalidProperty(
                invalid_will_properties,
                String::from("CONNECT"),
            ));
        };

        // check for duplicates
        let non_unique_properties = non_unique(&assigned_will_properties);
        if !non_unique_properties.is_empty() {
            return Err(PropertyError::PropertyAlreadyInserted(
                non_unique_properties,
                String::from("CONNECT"),
            ));
        }

        self.packet.will_properties = Some(assigned_will_properties.clone());
        Ok(())
    }

    fn will_topic(mut self, topic: String) -> Self {
        self.packet.will_topic = Some(topic);
        self
    }

    fn will_payload(mut self, will_payload: Vec<u8>) -> Self {
        self.packet.will_payload = Some(will_payload);
        self
    }

    pub fn will_message(
        &mut self,
        will_properties: &Vec<Property>,
        will_topic: String,
        will_payload: Vec<u8>,
    ) -> Result<(), PropertyError> {
        self.will_properties(will_properties)?;

        self.packet.will_topic = Some(will_topic);
        self.packet.will_payload = Some(will_payload);
        self.packet.connect_flags |= connect_flags::WILL_FLAG;

        Ok(())
    }

    pub fn username(mut self, username: Option<String>) -> Self {
        if username.is_some() && !username.as_ref().unwrap().is_empty() {
            self.packet.username = username;

            self.packet.connect_flags |= connect_flags::USER_NAME_FLAG;
        }
        self
    }

    pub fn password(mut self, password: Option<String>) -> Self {
        if password.is_some() && !password.as_ref().unwrap().is_empty() {
            self.packet.password = password;

            self.packet.connect_flags |= connect_flags::PASSWORD_FLAG;
        }

        self
    }

    pub fn generate_connect_flags(connect_packet: &Connect) -> u8 {
        let mut connect_flags = 0u8;
        if connect_packet.username.is_some() {
            connect_flags |= connect_flags::USER_NAME_FLAG;
        }

        if connect_packet.password.is_some() {
            connect_flags |= connect_flags::PASSWORD_FLAG;
        }

        connect_flags |= connect_packet.connect_flags;
        connect_flags
    }
}

impl Properties for Builder {
    fn packet_type(&self) -> PacketTypes {
        PacketTypes::Connect
    }

    fn packet_type_string(&self) -> String {
        String::from("CONNECT")
    }

    fn variable_header_properties(&self) -> &Option<Vec<Property>> {
        &self.packet.variable_header_properties
    }

    fn set_variable_header_properties(&mut self, p: Option<Vec<Property>>) {
        self.packet.variable_header_properties = p;
    }
}

impl GeneratePacketParts for Connect {
    fn generate_fixed_header(&self, fixed_header_remaining_length: usize) -> BytesMut {
        let mut fixed_header = BytesMut::with_capacity(5);

        let pt = (self.packet_type << 4) + (self.packet_type_flags & 0x0f);

        trace!("pt is {pt:?}");

        fixed_header.put_u8(pt);

        let fh_len = fixed_header.len();

        let packet_type = self.packet_type;

        let packet_type_flags = self.packet_type_flags;

        trace!("self.packet_type is {packet_type}");

        trace!("fixed header in generate_fixed_headers {fixed_header:?}, size is {fh_len:?}");

        let packet_type = *fixed_header.get(0).unwrap();
        trace!("packet type is {packet_type:?}");

        // need to capture error here.
        variable_byte_integer(
            "remaining length",
            &VariableByteInteger::new(fixed_header_remaining_length as u32),
            &mut fixed_header,
        )
        .unwrap();
        trace!(
            "fixed header is {:?}",
            fixed_header.clone().to_vec().hex_dump()
        );
        fixed_header
    }

    fn generate_variable_header(&self) -> BytesMut {
        // variable header // Protocol Name, protocol level, connect flags, keep alive and properties
        let mut variable_header = BytesMut::with_capacity(200);
        trace!("start of generate_variable_header");
        utf8_encoded_string(
            "protocol name",
            self.protocol_name.as_ref(),
            &mut variable_header,
        );

        variable_header.put_u8(self.protocol_version);

        variable_header.put_u8(Builder::generate_connect_flags(self));

        variable_header.put_u16(self.keep_alive);

        // Connect Properties
        let encoded_variable_header_properties = if self.variable_header_properties.is_none() {
            vec![]
        } else {
            encode_properties(&self.variable_header_properties)
        };

        let mut encoded_variable_header_properties_size: BytesMut = BytesMut::with_capacity(4);
        variable_byte_integer(
            "varaible header properties size",
            &VariableByteInteger::new(encoded_variable_header_properties.len() as u32),
            &mut encoded_variable_header_properties_size,
        )
        .unwrap();
        variable_header.put_slice(encoded_variable_header_properties_size.iter().as_slice());
        variable_header.put_slice(encoded_variable_header_properties.as_slice());
        trace!(
            "variable_header is {:?}",
            variable_header.clone().to_vec().hex_dump()
        );
        variable_header
    }

    // Double check if payload is being correctly generated
    fn generate_payload(&self) -> BytesMut {
        // payload details
        // client identifier, Will properties, will topic, will payload, username, password
        let mut payload = BytesMut::with_capacity(200);
        utf8_encoded_string("client id", &self.client_id, &mut payload);
        // will properties

        if self.will_flag() {
            trace!("will flag for encoding is set");

            let encoded_will_properties = encode_properties(&self.will_properties);

            let mut encoded_will_properties_size = BytesMut::with_capacity(4);
            variable_byte_integer(
                "will properties size",
                &VariableByteInteger::new(encoded_will_properties.len() as u32),
                &mut encoded_will_properties_size,
            )
            .unwrap();

            payload.put(encoded_will_properties_size);

            payload.put(encoded_will_properties.as_slice());

            // will topic
            if self.will_flag() {
                let topic = &self.will_topic.as_ref().unwrap().clone();
                utf8_encoded_string("topic", topic, &mut payload);
            }

            // will payload
            if self.will_payload.is_some() {
                payload.put_u16(self.will_payload.as_ref().unwrap().len() as u16);
                payload.put(self.will_payload.as_ref().unwrap().as_slice());
            }
        }

        //username
        if self.username_flag() {
            //payload.put(self.username.as_ref().unwrap().as_bytes());
            utf8_encoded_string("username", self.username.as_ref().unwrap(), &mut payload);
        }

        //password
        if self.password_flag() {
            //payload.put(self.password.as_ref().unwrap().as_bytes());
            utf8_encoded_string("password", self.password.as_ref().unwrap(), &mut payload);
        }

        // end of payload <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
        //(payload, payload_size, variable_header_size)
        trace!("payload is {:?}", payload.clone().to_vec().hex_dump());
        payload
    }
}
impl BuilderLifecycle<Connect> for Builder {
    fn new() -> Builder {
        Default::default()
    }

    fn build(self) -> anyhow::Result<Connect> {
        let connect_packet = self.packet;
        Ok(connect_packet)
    }
}

impl Encoder<Error> for Connect {
    fn encode(&self) -> anyhow::Result<BytesMut> {
        // checks not required here because they will be done inside the

        // let _ = will_properties_are_valid(self);

        // start of variable header >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
        trace!("start of encode");

        let variable_header = self.generate_variable_header();
        trace!(
            "variable header is {:?}",
            pretty_hex(&variable_header.to_vec())
        );

        // end of variable header <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

        let payload = self.generate_payload();

        trace!("payload is {:?}", pretty_hex(&payload.to_vec()));

        // start of making actual bytes for packets
        let fixed_header_remaining_length = variable_header.len() + payload.len();

        let mut connect_packet = BytesMut::with_capacity(fixed_header_remaining_length + 1 + 4);

        // start of fixed header >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

        let fixed_header = self.generate_fixed_header(fixed_header_remaining_length);

        trace!("fixed header is {:?}", pretty_hex(&fixed_header.to_vec()));

        // end of fixed header <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

        connect_packet.put(fixed_header);
        connect_packet.put(variable_header);
        connect_packet.put(payload);

        trace!(
            "full packet  is {:?}",
            pretty_hex(&connect_packet.clone().to_vec())
        );
        // end of making actual bytes for packets
        trace!("end of encode");

        Ok(BytesMut::from(connect_packet.to_vec().as_slice()))
    }
}

impl Decoder<Connect, Error> for Connect {
    fn decode(bytes: &mut BytesMut) -> anyhow::Result<Connect> {
        // checks are required here
        trace!("start of decode ---");
        trace!("start decoding. hex is {:?}", pretty_hex(bytes));

        // decode Fixed Header

        let packet_type_with_flags = bytes.get_u8();
        trace!("packet type with flags {:X}", packet_type_with_flags);
        let packet_type = packet_type_with_flags >> 4;
        trace!("packet_type {}", packet_type);
        let packet_type_flags = packet_type_with_flags & 0x0f;
        trace!("packet_type_flags {}", packet_type_flags);

        let packet_size = varint(bytes).unwrap();

        trace!("size of packet {}", packet_size.0);

        // decode Variable Header

        let protocol_name = utf8_string(String::from("protocol name"), bytes).unwrap();

        trace!("protocol_name is {}", protocol_name);

        // trace!("bytes after protocol are {bytes:?}");

        let protocol_version = bytes.get_u8();

        trace!("protocol version {:X}", protocol_version);

        // trace!("bytes after protocol version are {bytes:?}");

        let connect_flags = bytes.get_u8();

        trace!("connect flags are {:X}", connect_flags);

        // trace!("bytes after connect flags are {bytes:?}");

        let keep_alive = bytes.get_u16();

        trace!("keep alive {:X}", keep_alive);

        trace!("bytes before reading properties {bytes:?}");

        let variable_header_properties: Option<Vec<Property>> = {
            let p = property(bytes).unwrap();

            if !p.is_empty() {
                trace!("properties are {:?}", p);
                Some(p)
            } else {
                trace!("properties are empty");
                None
            }
        };

        trace!("bytes after reading properties {bytes:?}");

        trace!(
            "bytes left after variable header properties {}",
            bytes.len()
        );

        // decode Payload

        // need to check for duplicates in variable header properties
        // user property can be duplicated
        // other properties can't be duplicated

        let client_id = utf8_string(String::from("client identifier"), bytes).unwrap();

        trace!("client_id = {}", client_id);

        trace!("bytes after reading client_identifier {bytes:?}");

        trace!("bytes left are client identifier {}", bytes.len());

        let is_will_flag = (connect_flags & connect_flags::WILL_FLAG) > 0;
        trace!(
            "current will flag value is {}, raw value is {}",
            is_will_flag,
            connect_flags
        );

        let will_properties: Option<Vec<Property>> = if is_will_flag {
            // Will flag is set
            let prop = Some(property(bytes).unwrap());
            trace!("will properties are {:?}", prop);
            prop
        } else {
            trace!("No will properties");
            None
        };

        trace!("bytes left are will_properties {}", bytes.len());

        let will_topic: Option<String> = if is_will_flag {
            trace!("decoding will topic");
            let topic = Some(utf8_string(String::from("will_topic"), bytes).unwrap());
            trace!("will topic is {:?}", topic.clone().unwrap());
            topic
        } else {
            None
        };

        trace!("bytes left after will_topic {}", bytes.len());

        let will_payload: Option<Vec<u8>> = if is_will_flag {
            let payload = Some(
                binary(String::from("payload"), bytes)
                    .unwrap()
                    .as_ref()
                    .clone(),
            );

            if payload.is_some() {
                trace!("will payload is {:?}", payload.clone().unwrap())
            }
            payload
        } else {
            trace!("No will payload");
            None
        };

        let is_username_flag = connect_flags & connect_flags::USER_NAME_FLAG > 0;

        let username = if is_username_flag {
            let name = Some(utf8_string(String::from("username"), bytes).unwrap());
            trace!("username is {:?}", name);
            name
        } else {
            trace!("no username");
            None
        };

        let is_password_flag = connect_flags & connect_flags::PASSWORD_FLAG > 0;
        trace!("password flag is {is_password_flag}");
        trace!("bytes are {bytes:?}");
        let password = if is_password_flag {
            let passwd = Some(utf8_string(String::from("password"), bytes).unwrap());
            trace!("password is {:?}", passwd);
            passwd
        } else {
            trace!("no passworc");
            None
        };
        // Successful return
        trace!("end of decode ---");
        Ok(Connect {
            packet_type,
            packet_type_flags,
            protocol_name,
            protocol_version,
            connect_flags,
            keep_alive,
            variable_header_properties,
            will_properties,
            will_topic,
            will_payload,
            username,
            password,
            client_id,
        })
    }
}

#[cfg(test)]
pub mod test {
    use crate::mqttbroker::packets::connect::{Builder, Connect};
    use crate::mqttbroker::packets::error::PropertyError;
    use crate::mqttbroker::packets::{BuilderLifecycle, Decoder, Encoder, Properties};
    use crate::mqttbroker::primitive_types::{Byte, FourByteInteger};
    use crate::mqttbroker::properties::{Property, PropertyIdentifier};
    use pretty_hex::*;
    use std::collections::HashSet;
    use std::iter::FromIterator;
    use tracing::{trace, Level};
    use tracing_subscriber::FmtSubscriber;

    #[test]
    fn should_have_valid_variable_header_properties_for_connect_packet() {
        let mut packet = Builder::new();
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
        // let subscriber = FmtSubscriber::builder()
        //     .with_max_level(Level::TRACE)
        //     .finish();
        //
        // tracing::subscriber::set_global_default(subscriber)
        //     .expect("setting default subscriber failed");

        let mut packet = Builder::new();
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
    fn should_serdes_connect_packet() {
        // let subscriber = FmtSubscriber::builder()
        //     .with_max_level(Level::TRACE)
        //     .finish();
        //
        // tracing::subscriber::set_global_default(subscriber)
        //     .expect("setting default subscriber failed");

        let mut original_packet = Builder::new();

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

        let mut serialized_packet = built_packet.encode().unwrap();
        trace!(
            "serialized packet is {:?}",
            pretty_hex(&serialized_packet.as_ref().to_vec())
        );
        let deserialed_packet = Connect::decode(&mut serialized_packet).unwrap();

        assert_eq!(built_packet, deserialed_packet);
    }
}
