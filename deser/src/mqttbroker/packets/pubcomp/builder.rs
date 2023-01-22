use crate::mqttbroker::packets::pubcomp::PubComp;
use crate::mqttbroker::packets::reason_codes::PUBCOMP;
use crate::mqttbroker::packets::{BuilderLifecycle, PacketTypes, Properties};
use crate::mqttbroker::properties::Property;
use std::io::Error;

#[derive(Debug, Clone, Default)]
pub struct PubCompBuilder {
    pub packet: PubComp,
}

impl PubCompBuilder {
    pub fn set_packet_id(mut self, packet_id: u16) -> Self {
        self.packet.packet_id = packet_id;
        self
    }

    pub fn set_reason_code(mut self, reason_codes: PUBCOMP) -> Self {
        self.packet.reason_code = reason_codes;
        self
    }
}

impl Properties for PubCompBuilder {
    fn packet_type(&self) -> PacketTypes {
        PacketTypes::Pubcomp
    }

    fn packet_type_string(&self) -> String {
        String::from("PUBCOMP")
    }

    fn variable_header_properties(&self) -> &Option<Vec<Property>> {
        &self.packet.variable_header_properties
    }

    fn set_variable_header_properties(&mut self, p: Option<Vec<Property>>) {
        self.packet.variable_header_properties = p;
    }
}

impl BuilderLifecycle<PubComp, Error> for PubCompBuilder {
    fn new() -> Self {
        Default::default()
    }

    fn build(self) -> Result<PubComp, Error> {
        let packet = self.packet;
        Ok(packet)
    }
}
