use crate::decode::{decode_binary, decode_property, decode_utf8_string, decode_varint};
use crate::encode::{encode_utf8_encoded_string, encode_variable_byte_integer};
use crate::mqttbroker::packets::connect::validation::will_properties_are_valid;
use crate::mqttbroker::packets::error::PropertyError;
use crate::mqttbroker::packets::ConnectPacketBuildError::{
    WillFlagNotSet, WillPayLoadNotSet, WillTopicNotSet,
};
use crate::mqttbroker::packets::{
    connect_flags, encode_properties, BuilderLifecycle, ConnectPacketBuildError, Decoder, Encoder,
    GeneratePacketParts, PacketTypes,
};
use crate::mqttbroker::primitive_types::VariableByteInteger;
use crate::mqttbroker::properties::{
    diff, invalid_property_for_connect_packet_type, non_unique_properties,
    valid_properties_for_will, Property,
};
use bytes::{Buf, BufMut, BytesMut};
use std::io::Error;
use tokio::io;
use tracing::debug;

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
    pub variable_header_properties: Vec<Property>,
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
        println!(
            "will_flag is {:?}",
            (self.connect_flags & connect_flags::WILL_FLAG) == connect_flags::WILL_FLAG
        );
        (self.connect_flags & connect_flags::WILL_FLAG) == connect_flags::WILL_FLAG
    }

    pub fn clean_start_flag(&self) -> bool {
        self.connect_flags & connect_flags::CLEAN_START > 0
    }

    pub fn builder() -> ConnectBuilder {
        ConnectBuilder::default()
    }
}

pub mod validation {
    use crate::mqttbroker::packets::connect::Connect;
    use crate::mqttbroker::packets::connect_flags;
    use crate::mqttbroker::packets::ConnectPacketBuildError::{
        WillFlagNotSet, WillPayLoadNotSet, WillTopicNotSet,
    };

    pub fn will_properties_are_valid(connect_packet: &Connect) -> anyhow::Result<()> {
        if is_will_flag_not_set_and_will_properties_set_error(&connect_packet) {
            return Err(WillFlagNotSet.into());
        }

        if is_will_flag_set_and_will_topic_is_empty_error(&connect_packet) {
            return Err(WillTopicNotSet.into());
        }

        if is_will_flag_set_and_will_payload_is_empty_error(&connect_packet) {
            return Err(WillPayLoadNotSet.into());
        }

        Ok(())
    }

    fn is_will_flag_not_set_and_will_properties_set_error(connect_packet: &Connect) -> bool {
        if connect_packet.will_flag() {
            return false;
        }

        !connect_packet.will_flag()
            && connect_packet.will_properties.is_some()
            && (connect_packet.will_properties.as_ref().unwrap().len() > 0)
    }

    fn is_will_flag_set_and_will_topic_is_empty_error(connect_packet: &Connect) -> bool {
        if !connect_packet.will_flag() {
            return false;
        }

        // connect_packet.will_flag()
        //     && ((connect_packet.will_topic.is_some()
        //         && !connect_packet.will_topic.as_ref().unwrap().is_empty())
        //         || connect_packet.will_topic.is_none())

        if connect_packet.will_flag() && connect_packet.will_topic.is_none() {
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

    //fn is_will_flag_set_and

    #[cfg(test)]
    pub mod test {
        use crate::mqttbroker::packets::connect::validation::{
            is_will_flag_not_set_and_will_properties_set_error,
            is_will_flag_set_and_will_payload_is_empty_error,
            is_will_flag_set_and_will_topic_is_empty_error,
        };
        use crate::mqttbroker::packets::connect::Connect;
        use crate::mqttbroker::packets::connect_flags;
        use crate::mqttbroker::properties::Property;

        #[test]
        fn test_is_will_flag_not_set_and_will_properties_set() {
            // fix this first
            let connect_packet = Connect {
                will_topic: Some(String::from("hello")),
                will_payload: Some(vec![1, 2, 3]), //will_properties: Some(vec![Property::new()])..Default::default(),
                ..Default::default()
            };

            println!("connect_packet is {:?}", connect_packet);

            let ret = connect_packet.will_flag();

            let result = is_will_flag_not_set_and_will_properties_set_error(&connect_packet);

            assert!(result)
        }

        #[test]
        fn test_is_will_flag_set_and_topic_is_empty() {
            let connect_packet = Connect {
                connect_flags: connect_flags::WILL_FLAG,
                will_topic: Some(String::from("")),
                ..Default::default()
            };
            println!("{:?}", connect_packet);
            assert!(is_will_flag_set_and_will_topic_is_empty_error(
                &connect_packet
            ))
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
            ))
        }
    }
}

impl Default for Connect {
    fn default() -> Self {
        Connect {
            packet_type: PacketTypes::Connect as u8,
            packet_type_flags: 0,
            protocol_name: String::from("MQTT"),
            protocol_version: 5u8,
            keep_alive: 0,
            variable_header_properties: vec![],
            connect_flags: 0,
            client_id: "".to_string(),
            will_properties: None,
            will_topic: None,
            will_payload: None,
            username: None,
            password: None,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ConnectBuilder {
    pub packet: Connect,
}

impl ConnectBuilder {
    pub fn set_packet_type(mut self, pt: u8) -> Self {
        // don't need. We know it's for a connect packets
        self.packet.packet_type = pt;
        self
    }

    pub fn set_protocol_name(mut self, pn: String) -> Self {
        self.packet.protocol_name = pn;
        self
    }

    pub fn set_keep_alive(mut self, keep_alive: u16) -> Self {
        self.packet.keep_alive = keep_alive;
        self
    }

    pub fn set_properties(
        &mut self,
        mut property: Vec<Property>,
        will_flag: bool,
    ) -> Result<(), PropertyError> {
        //Needs to ignore multiple user properties

        let mut added_property: Vec<Property> = Vec::with_capacity(100);
        added_property.append(&mut property);

        let invalid_properties = invalid_property_for_connect_packet_type(&property, will_flag);

        if !invalid_properties.is_empty() {
            return Err(PropertyError::InvalidProperty(
                invalid_properties,
                String::from("CONNECT"),
            ));
        };

        // let non_unique_properties = check_for_non_unique_properties(&property);
        //
        // if !non_unique_properties.is_empty() {
        //     return Err(PropertyError::PropertyAlreadyInserted(
        //         non_unique_properties,
        //         String::from("CONNECT"),
        //     ));
        // }

        let mut packet_properties: Vec<Property> = vec![];
        packet_properties.append(&mut added_property); // added_property field is empty
        self.packet.variable_header_properties.clear();
        self.packet
            .variable_header_properties
            .append(&mut packet_properties);
        Ok(())
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
        self.packet.connect_flags &= !connect_flags::WILL_QOS_MASK | new_qos << 3
    }

    // pub fn connect_flags_with_clean_start(mut self, b: bool) -> Self {
    //     self.connect_packet.connect_flags_clean_start = b;
    //     self
    // }

    pub fn clean_start(mut self, b: bool) -> Self {
        if b {
            self.packet.connect_flags |= 2;
            return self;
        }

        self.packet.connect_flags = !2;
        self
    }

    //
    // pub fn connect_flags_with_will_flag(mut self, b: bool) -> Self {
    //     self.connect_packet.connect_flags_will_flag = b;
    //     self
    // }

    pub fn client_id(mut self, ci: String) -> Self {
        self.packet.client_id = ci;
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
        let mut invalid_will_properties: Vec<Property> = vec![];
        diff(
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
        let non_unique_properties = non_unique_properties(&assigned_will_properties);
        if !non_unique_properties.is_empty() {
            return Err(PropertyError::PropertyAlreadyInserted(
                non_unique_properties,
                String::from("CONNECT"),
            ));
        }

        self.packet.will_properties = Some(assigned_will_properties);
        Ok(self)
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
        mut self,
        will_properties: Vec<Property>,
        will_topic: String,
        will_payload: Vec<u8>,
    ) -> Self {
        self.packet.will_properties = Some(will_properties);
        self.packet.will_topic = Some(will_topic);
        self.packet.will_payload = Some(will_payload);
        self.packet.connect_flags |= connect_flags::WILL_FLAG;

        self
    }

    pub fn username(mut self, username: String) -> Self {
        self.packet.username = Some(username);
        self
    }

    pub fn password(mut self, password: String) -> Self {
        self.packet.password = Some(password);
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

impl GeneratePacketParts for Connect {
    fn generate_fixed_header(&self, fixed_header_remaining_length: usize) -> BytesMut {
        let mut fixed_header = BytesMut::with_capacity(5);
        fixed_header.put_u8(((self.packet_type & 0x0f) << 4) + (self.packet_type_flags & 0x0f));
        // need to capture error here.
        encode_variable_byte_integer(
            &VariableByteInteger::new(fixed_header_remaining_length as u32),
            &mut fixed_header,
        )
        .unwrap();

        fixed_header
    }

    fn generate_variable_header(&self) -> BytesMut {
        // variable header // Protocol Name, protocol level, connect flags, keep alive and properties
        let mut variable_header = BytesMut::with_capacity(200);

        encode_utf8_encoded_string(self.protocol_name.as_ref(), &mut variable_header);

        variable_header.put_u8(self.protocol_version);

        variable_header.put_u8(ConnectBuilder::generate_connect_flags(&self));

        variable_header.put_u16(self.keep_alive);

        // Connect Properties
        let encoded_variable_header_properties = if !&self.variable_header_properties.is_empty() {
            encode_properties(&Some(self.variable_header_properties.clone()))
        } else {
            vec![]
        };

        let mut encoded_variable_header_properties_size: BytesMut = BytesMut::with_capacity(4);
        encode_variable_byte_integer(
            &VariableByteInteger::new(encoded_variable_header_properties.len() as u32),
            &mut encoded_variable_header_properties_size,
        )
        .unwrap();
        variable_header.put_slice(encoded_variable_header_properties_size.iter().as_slice());
        variable_header.put_slice(encoded_variable_header_properties.as_slice());
        variable_header
    }

    // Double check if payload is being correctly generated
    fn generate_payload(&self) -> BytesMut {
        // payload details
        // client identifier, Will properties, will topic, will payload, username, password
        let mut payload = BytesMut::with_capacity(200);
        encode_utf8_encoded_string(&self.client_id, &mut payload);
        // will properties

        let encoded_will_properties = encode_properties(&self.will_properties);

        let mut encoded_will_properties_size = BytesMut::with_capacity(4);
        encode_variable_byte_integer(
            &VariableByteInteger::new(encoded_will_properties.len() as u32),
            &mut encoded_will_properties_size,
        )
        .unwrap();

        payload.put(encoded_will_properties_size);
        payload.put(encoded_will_properties.as_slice());

        if self.will_flag() {
            // will topic
            if self.will_flag() {
                let topic = &self.will_topic.as_ref().unwrap().clone();
                let len: u16 = self.will_topic.as_ref().unwrap().len() as u16;
                payload.put_u16(len);
                payload.put(self.will_topic.as_ref().unwrap().as_bytes());
            }

            // will payload
            if self.will_payload.is_some() {
                payload.put_u16(self.will_payload.as_ref().unwrap().len() as u16);
                payload.put(self.will_payload.as_ref().unwrap().as_slice());
            }
        }

        //username
        if self.username_flag() {
            payload.put(self.username.as_ref().unwrap().as_bytes())
        }

        //password
        if !self.password_flag() {
            payload.put(self.password.as_ref().unwrap().as_bytes());
        }

        // end of payload <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
        //(payload, payload_size, variable_header_size)
        payload
    }
}
impl BuilderLifecycle<Connect> for ConnectBuilder {
    fn new() -> ConnectBuilder {
        Default::default()
    }

    fn build(self) -> anyhow::Result<Connect> {
        // will_properties_are_valid(&self.packet)?;
        //
        // // start of variable header >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
        //
        // let variable_header = self.generate_variable_header();
        //
        // // end of variable header <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
        //
        // // let (payload, payload_size, variable_header_size) = self.generate_payload(&variable_header);
        // let payload = self.generate_payload();
        // let payload_size = payload.len();
        // let variable_header_size = variable_header.len();
        //
        // // start of making actual bytes for packets
        // let fixed_header_remaining_length = variable_header_size + payload_size;
        //
        // // 1 = byte 1 of fixed header, 4 = max size of Remaining Length of fixed header
        // let mut connect_packet = BytesMut::with_capacity(fixed_header_remaining_length + 1 + 4);
        //
        // // start of fixed header >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>
        //
        // let fixed_header = self.generate_fixed_header(fixed_header_remaining_length);
        //
        // // end of fixed header <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
        //
        // debug!("++++++++++++++ fixed header is {:?}", fixed_header);
        // debug!("++++++++++++++ variable header is {:?}", variable_header);
        // debug!("++++++++++++++ payload is {:?}", payload);
        // println!("JELLO WORLD");
        // connect_packet.put(fixed_header);
        // connect_packet.put(variable_header);
        // connect_packet.put(payload);
        //
        // // end of making actual bytes for packets

        let connect_packet = self.packet;
        will_properties_are_valid(&connect_packet)?;
        // check properties

        Ok(connect_packet)
    }
}

impl Encoder<Error> for Connect {
    fn encode(&self) -> anyhow::Result<BytesMut> {
        let _ = will_properties_are_valid(&self);

        // start of variable header >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

        let variable_header = self.generate_variable_header();

        // end of variable header <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

        let payload = self.generate_payload();

        // start of making actual bytes for packets
        let fixed_header_remaining_length = variable_header.len() + payload.len();

        let mut connect_packet = BytesMut::with_capacity(fixed_header_remaining_length + 1 + 4);

        // start of fixed header >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>

        let fixed_header = self.generate_fixed_header(fixed_header_remaining_length);

        // end of fixed header <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<

        debug!("++++++++++++++ fixed header is {:?}", fixed_header);
        debug!("++++++++++++++ variable header is {:?}", variable_header);
        debug!("++++++++++++++ payload is {:?}", payload);
        println!("JELLO WORLD");
        connect_packet.put(fixed_header);
        connect_packet.put(variable_header);
        connect_packet.put(payload);

        // end of making actual bytes for packets

        Ok(BytesMut::from(connect_packet.to_vec().as_slice()))
    }
}

impl Decoder<Connect, Error> for Connect {
    fn decode(bytes: &mut BytesMut) -> anyhow::Result<Connect> {
        // decode Fixed Header
        debug!("bytes left at start {}", bytes.len());
        let packet_type = bytes.get_u8();
        debug!("bytes left after pack_type {}", bytes.len());

        let packet_size = decode_varint(bytes).unwrap();

        // decode Variable Header

        debug!("bytes left after packet_size {}", bytes.len());

        let protocol_name = decode_utf8_string(String::from("protocol name"), bytes).unwrap();

        let protocol_version = bytes.get_u8();

        let connect_flags = bytes.get_u8();

        let keep_alive = bytes.get_u16();

        let variable_header_properties = decode_property(bytes).unwrap();
        debug!(
            "bytes left after variable header properties {}",
            bytes.len()
        );

        // decode Payload

        // need to check for duplicates in variable header properties
        // user property can be duplicated
        // other properties can't be duplicated

        let client_identifier =
            decode_utf8_string(String::from("client identifier"), bytes).unwrap();
        debug!("bytes left are client identifier {}", bytes.len());

        let is_will_flag = (connect_flags & connect_flags::WILL_FLAG) > 0;
        debug!(
            "current will flag value is {}, raw value is {}",
            is_will_flag, connect_flags
        );

        let will_properties: Option<Vec<Property>> = if is_will_flag {
            // Will flag is set
            Some(decode_property(bytes).unwrap())
        } else {
            None
        };

        debug!("bytes left are will_properties {}", bytes.len());

        let will_topic: String = if is_will_flag {
            debug!("decoding will topic");
            decode_utf8_string(String::from("will_topic"), bytes).unwrap()
        } else {
            String::from("")
        };

        debug!("bytes left after will_topic {}", bytes.len());

        let will_payload: Vec<u8> = if is_will_flag {
            decode_binary(String::from("payload"), bytes)
                .unwrap()
                .as_ref()
                .clone()
        } else {
            vec![]
        };

        let is_username_flag = connect_flags & connect_flags::USER_NAME_FLAG > 0;

        let username = if is_username_flag {
            decode_utf8_string(String::from("username"), bytes).unwrap()
        } else {
            String::from("")
        };

        let is_password_flag = connect_flags & connect_flags::PASSWORD_FLAG > 0;

        let password = if is_password_flag {
            decode_utf8_string(String::from("password"), bytes).unwrap()
        } else {
            String::from("")
        };
        // Successful return
        Ok(Connect {
            packet_type: packet_type >> 4,
            packet_type_flags: 0,
            protocol_name,
            protocol_version,
            connect_flags,
            keep_alive,
            variable_header_properties,
            will_properties: Some(will_properties.unwrap_or_default()),
            will_topic: Some(will_topic),
            will_payload: Some(will_payload),
            username: Some(username),
            password: Some(password),
            client_id: client_identifier,
        })
    }
}
