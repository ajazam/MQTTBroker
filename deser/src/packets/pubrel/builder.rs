use crate::packets::pubrel::PubRel;
use crate::packets::reason_codes::PUBREL;
use crate::packets::{BuilderLifecycle, PacketTypes, Properties};
use crate::properties::Property;
use std::io::Error;

#[derive(Debug, Clone, Default)]
pub struct PubRelBuilder {
    pub packet: PubRel,
}

impl PubRelBuilder {
    pub fn set_packet_id(mut self, packet_id: u16) -> Self {
        self.packet.packet_id = packet_id;
        self
    }

    pub fn set_reason_code(mut self, reason_code: PUBREL) -> Self {
        self.packet.reason_code = reason_code;
        self
    }
}

impl Properties for PubRelBuilder {
    fn packet_type(&self) -> PacketTypes {
        PacketTypes::Pubrel
    }

    fn packet_type_string(&self) -> String {
        String::from("PUBREL")
    }

    fn variable_header_properties(&self) -> &Option<Vec<Property>> {
        &self.packet.variable_header_properties
    }

    fn set_variable_header_properties(&mut self, p: Option<Vec<Property>>) {
        self.packet.variable_header_properties = p;
    }
}

impl BuilderLifecycle<PubRel, Error> for PubRelBuilder {
    fn new() -> Self {
        Default::default()
    }

    fn build(self) -> Result<PubRel, Error> {
        let packet = self.packet;
        Ok(packet)
    }
}
