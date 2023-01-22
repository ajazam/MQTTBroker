use crate::mqttbroker::packets::{
    encode_properties, encode_properties_to_vec, BuilderLifecycle, Decoder, Encoder,
    GeneratePacketParts, PacketTypes, Properties,
};
use crate::mqttbroker::properties::Property;
use bytes::{Buf, BufMut, BytesMut};
use std::io::Error;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Publish {
    //fixed header
    packet_type: u8,
    packet_type_low_nibble: u8,

    //variable_header
    topic_name: String,
    packet_id: Option<u16>,
    variable_header_properties: Option<Vec<Property>>,

    //payload
    application_message: Option<Vec<u8>>,
}

impl Publish {
    pub fn dup(&self) -> bool {
        self.packet_type_low_nibble & 8 == 8
    }

    pub fn qos(&self) -> Qos {
        let qos = (self.packet_type_low_nibble >> 1) & 3;

        if qos == 0 {
            Qos::Q0
        } else if qos == 1 {
            Qos::Q1(self.packet_id.unwrap())
        } else if qos == 2 {
            Qos::Q2(self.packet_id.unwrap())
        } else {
            Qos::Reserved
        }
    }

    pub fn qos_number(&self) -> u8 {
        match self.qos() {
            Qos::Q0 => 0,
            Qos::Q1(_) => 1,
            Qos::Q2(_) => 2,
            Qos::Reserved => 3,
        }
    }

    pub fn packet_id(self) -> Option<u16> {
        self.packet_id
    }

    pub fn retain(&self) -> bool {
        self.packet_type_low_nibble & 1 == 1
    }
}
#[repr(u8)]
pub enum Qos {
    Q0 = 0u8,
    Q1(u16) = 1u8,
    Q2(u16) = 2u8,
    Reserved = 3u8,
}

impl Default for Publish {
    fn default() -> Self {
        Publish {
            packet_type: PacketTypes::Publish as u8,
            packet_type_low_nibble: 0,
            topic_name: "".to_string(),
            packet_id: None,
            variable_header_properties: None,
            application_message: None,
        }
    }
}

pub mod encode_validation {}

pub mod decode_validate {
    use crate::mqttbroker::packets::publish::{Publish, PublishBuilder, PublishError};

    //[MQTT-2.2.1-2]
    fn is_qos_and_packet_id_valid(s: &PublishBuilder) -> Result<(), PublishError> {
        if (1u8..=2).contains(&s.packet.qos_number()) && s.packet.packet_id.is_none() {
            return Err(PublishError::NoPacketIdForQos(s.packet.qos_number()));
        }

        Ok(())
    }

    //[MQTT-3.3.1-2]
    fn is_dup_valid(s: &PublishBuilder) -> Result<(), PublishError> {
        if s.packet.dup() && s.packet.qos_number() == 0 {
            return Err(PublishError::DupInvalidForQos0);
        }

        Ok(())
    }

    //[MQTT-3.3.1-4]
    fn is_both_qos_bits_set(s: &PublishBuilder) -> Result<(), PublishError> {
        if s.packet.qos_number() == 3 {
            return Err(PublishError::BothQosBitsAreSet);
        }

        Ok(())
    }

    fn is_topic_name_present(s: &PublishBuilder) -> Result<(), PublishError> {
        if s.packet.topic_name.is_empty() {
            return Err(PublishError::TopicNotPresent);
        }

        Ok(())
    }

    pub fn validation_checks(s: &PublishBuilder) -> Result<(), Vec<PublishError>> {
        //todo check for any more outstanding tests
        let mut errors = vec![];

        let ret = is_qos_and_packet_id_valid(s);
        if ret.is_err() {
            errors.push(ret.err().unwrap())
        }

        let ret = is_dup_valid(s);
        if ret.is_err() {
            errors.push(ret.err().unwrap())
        }

        let ret = is_both_qos_bits_set(s);
        if ret.is_err() {
            errors.push(ret.err().unwrap());
        }

        let ret = is_topic_name_present(s);
        if ret.is_err() {
            errors.push(ret.err().unwrap())
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct PublishBuilder {
    pub packet: Publish,
}

impl PublishBuilder {
    pub fn set_dup(self, dup: bool) {}

    pub fn set_retain(self, retain: bool) {}

    pub fn set_topic_with_payload(mut self, topic_name: String, payload: Option<Vec<u8>>) -> Self {
        self.packet.topic_name = topic_name;
        self.packet.application_message = payload;
        self
    }

    fn set_qos_only(mut self, qos: u8, packet_id: Option<u16>) -> Self {
        let qos_mask: u8 = 0b00000110;
        let mut actual_qos = qos << 1;
        actual_qos &= qos_mask;

        self.packet.packet_type_low_nibble |= actual_qos;
        self.packet.packet_id = packet_id;
        self
    }

    pub fn set_qos(self, qos: Qos) -> Self {
        match qos {
            Qos::Q0 => self.set_qos_only(0, None),
            Qos::Q1(packet_id) => self.set_qos_only(1, Some(packet_id)),
            Qos::Q2(packet_id) => self.set_qos_only(2, Some(packet_id)),
            Qos::Reserved => self.set_qos_only(3, None),
        }
    }
}

use crate::decode::{decode_property, utf8_string, varint};
use crate::encode::utf8_encoded_string;
use crate::mqttbroker::packets::publish::decode_validate::validation_checks;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum PublishError {
    #[error("There is no Packet ID for qos level {0}. Require a Packet ID for qos > 0")]
    NoPacketIdForQos(u8),
    #[error("Dup flag is set to 1 when Qos is 0. Dup flag should be set to 0")]
    DupInvalidForQos0,
    #[error("Malformed packet. Both Qos bits are set to 1")]
    BothQosBitsAreSet,
    #[error("Topic name not present")]
    TopicNotPresent,
}

impl BuilderLifecycle<Publish, Vec<PublishError>> for PublishBuilder {
    fn new() -> Self {
        Default::default()
    }

    fn build(self) -> Result<Publish, Vec<PublishError>> {
        validation_checks(&self)?;

        Ok(self.packet)
    }
}

impl Properties for PublishBuilder {
    fn packet_type(&self) -> PacketTypes {
        PacketTypes::Publish
    }

    fn packet_type_string(&self) -> String {
        String::from("PUBLISH")
    }

    fn variable_header_properties(&self) -> &Option<Vec<Property>> {
        &self.packet.variable_header_properties
    }

    fn set_variable_header_properties(&mut self, p: Option<Vec<Property>>) {
        self.packet.variable_header_properties = p;
    }
}

impl GeneratePacketParts for Publish {
    fn generate_variable_header(&self) -> BytesMut {
        // fields are Topic Name, Packet Identifier, Properties
        let mut variable_header = BytesMut::with_capacity(200);
        //encode topic name
        utf8_encoded_string("topic", &self.topic_name, &mut variable_header);
        //encode packet identifier
        if (1..=2).contains(&self.qos_number()) {
            variable_header.put_u16(self.packet_id.unwrap())
        }

        variable_header = encode_properties(variable_header, &self.variable_header_properties);

        variable_header
    }

    fn generate_payload(&self) -> BytesMut {
        // fields are Application Message
        let mut payload = BytesMut::with_capacity(200);
        if self.application_message.is_some() {
            let local_appmessage = self.application_message.clone();
            payload.put(local_appmessage.unwrap().as_slice());
        }

        println!("encode payload is {payload:?}",);

        payload
    }
}

impl Encoder<Publish> for Publish {}

impl Decoder<Publish> for Publish {
    fn decode(bytes: &mut BytesMut) -> anyhow::Result<Publish> {
        let packet_type_with_flags = bytes.get_u8();
        let packet_type = packet_type_with_flags >> 4;
        let packet_type_low_nibble = packet_type_with_flags & 0x0f;
        let packet_size = varint(bytes).unwrap();
        let topic_name = utf8_string(String::from("topic_name"), bytes).unwrap();
        // 0b000_0110 mask for Qos Level.
        let qos = packet_type_low_nibble & 0b0000_0110 >> 1;
        println!("qos is {qos}");
        let packet_id = if (1..=2).contains(&qos) {
            println!("got packet_id");
            Some(bytes.get_u16())
        } else {
            println!("packet _id isnot present");
            None
        };

        let variable_header_properties = decode_property(bytes);

        let application_message = if !bytes.is_empty() {
            Some(bytes.to_vec())
        } else {
            None
        };

        Ok(Publish {
            packet_type,
            packet_type_low_nibble,
            topic_name,
            packet_id,
            variable_header_properties,
            application_message,
        })
    }
}

#[cfg(test)]
pub mod test {
    use crate::mqttbroker::packets::publish::{Publish, PublishBuilder, Qos};
    use pretty_hex::{pretty_hex, PrettyHex};

    use crate::mqttbroker::packets::{BuilderLifecycle, Decoder, Encoder, Properties};
    use crate::mqttbroker::primitive_types::TwoByteInteger;
    use crate::mqttbroker::properties::Property;

    #[test]
    pub fn test_encode_decode_publish_packet() {
        let mut packet_builder = PublishBuilder::new();
        let props = vec![Property::TopicAlias(TwoByteInteger(70))];
        let res = packet_builder.set_properties(&props);
        packet_builder =
            packet_builder.set_topic_with_payload(String::from("abcdef"), Some(vec![1u8, 2, 3, 4]));
        packet_builder = packet_builder.set_qos(Qos::Q1(100));
        let built_packet = packet_builder.build().unwrap();
        let mut serialized_packet = Publish::encode(
            built_packet.packet_type,
            built_packet.packet_type_low_nibble,
            &built_packet,
        )
        .unwrap();
        let copy = serialized_packet.clone().to_vec();
        let deserialized_packet = Publish::decode(&mut serialized_packet).unwrap();

        println!("serialized packet is {copy:?}");
        println!("hex is {}", copy.hex_dump());

        assert_eq!(built_packet, deserialized_packet);
    }
}
