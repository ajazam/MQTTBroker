use crate::packets::reason_codes::SUBACK;
use crate::packets::suback::SubAck;
use crate::packets::{BuilderLifecycle, PacketTypes, Properties};
use crate::properties::Property;
use std::io::Error;

#[derive(Debug, Clone, Default)]
pub struct SubAckBuilder {
    pub packet: SubAck,
}

impl SubAckBuilder {
    pub fn set_packet_id(mut self, packet_id: u16) -> Self {
        self.packet.packet_id = packet_id;
        self
    }
    pub fn set_reason_code(mut self, reason_code: Vec<SUBACK>) -> Self {
        // need to have a check here for a valid reason code.
        self.packet.reason_codes = reason_code;
        self
    }
}

impl Properties for SubAckBuilder {
    fn packet_type(&self) -> PacketTypes {
        PacketTypes::Suback
    }

    fn packet_type_string(&self) -> String {
        String::from("SUBACK")
    }

    fn variable_header_properties(&self) -> &Option<Vec<Property>> {
        &self.packet.variable_header_properties
    }

    fn set_variable_header_properties(&mut self, p: Option<Vec<Property>>) {
        self.packet.variable_header_properties = p;
    }
}

impl BuilderLifecycle<SubAck, Error> for SubAckBuilder {
    fn new() -> Self {
        Default::default()
    }

    fn build(self) -> Result<SubAck, Error> {
        let suback_packet = self.packet;
        Ok(suback_packet)
    }
}
