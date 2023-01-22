use crate::mqttbroker::packets::pubrec::PubRec;
use crate::mqttbroker::packets::reason_codes::PUBREC;
use crate::mqttbroker::packets::{BuilderLifecycle, PacketTypes, Properties};
use crate::mqttbroker::properties::Property;
use std::io::Error;

#[derive(Debug, Clone, Default)]
pub struct PubRecBuilder {
    pub packet: PubRec,
}

impl PubRecBuilder {
    pub fn set_packet_id(mut self, packet_id: u16) -> Self {
        self.packet.packet_id = packet_id;
        self
    }

    pub fn set_reason_code(mut self, reason_code: PUBREC) -> Self {
        self.packet.reason_code = reason_code;
        self
    }
}

impl Properties for PubRecBuilder {
    fn packet_type(&self) -> PacketTypes {
        PacketTypes::Puback
    }

    fn packet_type_string(&self) -> String {
        String::from("PUBREC")
    }

    fn variable_header_properties(&self) -> &Option<Vec<Property>> {
        &self.packet.variable_header_properties
    }

    fn set_variable_header_properties(&mut self, p: Option<Vec<Property>>) {
        self.packet.variable_header_properties = p;
    }
}

impl BuilderLifecycle<PubRec, Error> for PubRecBuilder {
    fn new() -> Self {
        Default::default()
    }

    fn build(self) -> Result<PubRec, Error> {
        let packet = self.packet;
        Ok(packet)
    }
}
