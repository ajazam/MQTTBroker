use crate::mqttbroker::packets::puback::PubAck;
use crate::mqttbroker::packets::reason_codes::PUBACK;
use crate::mqttbroker::packets::{BuilderLifecycle, PacketTypes, Properties};
use crate::mqttbroker::properties::Property;
use std::io::Error;

#[derive(Debug, Clone, Default)]
pub struct PubAckBuilder {
    pub packet: PubAck,
}

impl PubAckBuilder {
    pub fn set_packet_id(mut self, packet_id: u16) -> Self {
        self.packet.packet_id = packet_id;
        self
    }

    pub fn set_reason_code(mut self, reason_code: PUBACK) -> Self {
        self.packet.reason_code = reason_code;
        self
    }
}

impl Properties for PubAckBuilder {
    fn packet_type(&self) -> PacketTypes {
        PacketTypes::Puback
    }

    fn packet_type_string(&self) -> String {
        String::from("PUBACK")
    }

    fn variable_header_properties(&self) -> &Option<Vec<Property>> {
        &self.packet.variable_header_properties
    }

    fn set_variable_header_properties(&mut self, p: Option<Vec<Property>>) {
        self.packet.variable_header_properties = p;
    }
}

impl BuilderLifecycle<PubAck, Error> for PubAckBuilder {
    fn new() -> Self {
        Default::default()
    }

    fn build(self) -> Result<PubAck, Error> {
        let puback_packet = self.packet;
        Ok(puback_packet)
    }
}
