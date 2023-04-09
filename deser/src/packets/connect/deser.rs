use crate::decode::{binary, decode_property, property, utf8_string, varint};
use crate::encode::{utf8_encoded_string, variable_byte_integer};
use crate::packets::connect::builder::ConnectBuilder;
use crate::packets::connect::Connect;
use crate::packets::{
    connect_flags, encode_properties, encode_properties_to_vec, Decoder, Encoder,
    GeneratePacketParts,
};
use crate::primitive_types::VariableByteInteger;
use crate::properties::Property;
use bytes::{Buf, BufMut, BytesMut};
use nu_pretty_hex::*;
use std::error::Error;
use tracing::trace;

impl GeneratePacketParts for Connect {
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

        variable_header.put_u8(ConnectBuilder::generate_connect_flags(self));

        variable_header.put_u16(self.keep_alive);

        // Connect Properties
        variable_header = encode_properties(variable_header, &self.variable_header_properties);
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

            let encoded_will_properties = encode_properties_to_vec(&self.will_properties);

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

impl Encoder<Connect> for Connect {}

impl Decoder<Connect> for Connect {
    fn decode(bytes: &mut BytesMut) -> Result<Connect, Box<dyn Error + Send + Sync>> {
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

        let variable_header_properties = decode_property(bytes);

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
            packet_type_low_nibble: packet_type_flags,
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
