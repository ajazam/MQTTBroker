use crate::packets::reason_codes::UNSUBACK;
use crate::packets::unsuback::UnsubAck;
use crate::packets::{BuilderLifecycle, PacketTypes, Properties};
use crate::properties::Property;
use std::io::Error;

#[derive(Debug, Clone, Default)]
pub struct UnSubAckBuilder {
    pub packet: UnsubAck,
}

impl UnSubAckBuilder {
    pub fn set_packet_id(mut self, packet_id: u16) -> Self {
        self.packet.packet_id = packet_id;
        self
    }

    pub fn set_topic_filters(mut self, topic_filter: Vec<UNSUBACK>) -> Self {
        self.packet.topic_filters = topic_filter;
        self
    }
}

impl Properties for UnSubAckBuilder {
    fn packet_type(&self) -> PacketTypes {
        PacketTypes::Unsuback
    }

    fn packet_type_string(&self) -> String {
        String::from("UNSUBACK")
    }

    fn variable_header_properties(&self) -> &Option<Vec<Property>> {
        &self.packet.variable_header_properties
    }

    fn set_variable_header_properties(&mut self, p: Option<Vec<Property>>) {
        self.packet.variable_header_properties = p;
    }
}

impl BuilderLifecycle<UnsubAck, Error> for UnSubAckBuilder {
    fn new() -> Self {
        Default::default()
    }

    fn build(self) -> Result<UnsubAck, Error> {
        let unsuback_packet = self.packet;
        Ok(unsuback_packet)
    }
}
