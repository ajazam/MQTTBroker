use crate::mqttbroker::packets::publish::validation::decode_validate::validation_checks;
use crate::mqttbroker::packets::publish::{Publish, PublishError, Qos};
use crate::mqttbroker::packets::{BuilderLifecycle, PacketTypes, Properties};
use crate::mqttbroker::properties::Property;

#[derive(Debug, Clone, Default)]
pub struct PublishBuilder {
    pub packet: Publish,
}

impl PublishBuilder {
    // pub fn set_dup(self, dup: bool) {}
    //
    // pub fn set_retain(self, retain: bool) {}

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

impl BuilderLifecycle<Publish, Vec<PublishError>> for PublishBuilder {
    fn new() -> Self {
        Default::default()
    }

    fn build(self) -> Result<Publish, Vec<PublishError>> {
        validation_checks(&self)?;

        Ok(self.packet)
    }
}
