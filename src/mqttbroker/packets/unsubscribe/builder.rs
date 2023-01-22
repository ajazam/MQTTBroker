use crate::mqttbroker::packets::unsubscribe::UnSubscribe;
use crate::mqttbroker::packets::{BuilderLifecycle, PacketTypes, Properties};
use crate::mqttbroker::properties::Property;
use std::io::Error;

#[derive(Debug, Clone, Default)]
pub struct UnsubscribeBuilder {
    pub packet: UnSubscribe,
}

impl UnsubscribeBuilder {
    pub fn set_packet_id(mut self, packet_id: u16) -> Self {
        self.packet.packet_id = packet_id;
        self
    }

    pub fn set_topic_filters(mut self, topic_filters: Vec<String>) -> Self {
        self.packet.topic_filters = topic_filters;
        self
    }
}

impl Properties for UnsubscribeBuilder {
    fn packet_type(&self) -> PacketTypes {
        PacketTypes::Unsubscribe
    }

    fn packet_type_string(&self) -> String {
        String::from("UNSUBSCRIBE")
    }

    fn variable_header_properties(&self) -> &Option<Vec<Property>> {
        &self.packet.variable_header_properties
    }

    fn set_variable_header_properties(&mut self, p: Option<Vec<Property>>) {
        self.packet.variable_header_properties = p;
    }
}

impl BuilderLifecycle<UnSubscribe, Error> for UnsubscribeBuilder {
    fn new() -> Self {
        Default::default()
    }

    fn build(self) -> Result<UnSubscribe, Error> {
        let unsubscribe_packet = self.packet;
        Ok(unsubscribe_packet)
    }
}
