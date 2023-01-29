use crate::packets::connack::ConnAck;
use crate::packets::reason_codes::CONNECTACK;
use crate::packets::PacketTypes::Connack;
use crate::packets::{BuilderLifecycle, PacketTypes, Properties};
use crate::properties::Property;
use std::io::Error;

#[derive(Debug, Clone, Default)]
pub struct ConnAckBuilder {
    pub packet: ConnAck,
}

impl ConnAckBuilder {
    pub fn set_session_present(mut self, session_present: bool) -> Self {
        if session_present {
            self.packet.connect_ack_flags = 1
        } else {
            self.packet.connect_ack_flags = 0
        }

        self
    }

    pub fn set_connect_reason_code(mut self, reason_code: CONNECTACK) -> Self {
        self.packet.connect_reason_code = reason_code as u8;
        self
    }
}

impl BuilderLifecycle<ConnAck, Error> for ConnAckBuilder {
    fn new() -> Self {
        Default::default()
    }

    fn build(self) -> Result<ConnAck, Error> {
        Ok(self.packet)
    }
}

impl Properties for ConnAckBuilder {
    fn packet_type(&self) -> PacketTypes {
        Connack
    }

    fn packet_type_string(&self) -> String {
        String::from("CONNACK")
    }

    fn variable_header_properties(&self) -> &Option<Vec<Property>> {
        &self.packet.variable_header_properties
    }

    fn set_variable_header_properties(&mut self, p: Option<Vec<Property>>) {
        self.packet.variable_header_properties = p;
    }
}
