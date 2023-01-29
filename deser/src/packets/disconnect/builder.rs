use crate::packets::disconnect::Disconnect;
use crate::packets::reason_codes::DISCONNECT;
use crate::packets::{BuilderLifecycle, PacketTypes, Properties};
use crate::properties::Property;
use std::io::Error;

#[derive(Debug, Clone, Default)]
pub struct DisconnectBuilder {
    pub packet: Disconnect,
}

impl DisconnectBuilder {
    pub fn set_reason_code(mut self, reason_code: DISCONNECT) -> Self {
        self.packet.reason_code = reason_code;
        self
    }
}

impl Properties for DisconnectBuilder {
    fn packet_type(&self) -> PacketTypes {
        PacketTypes::Disconnect
    }

    fn packet_type_string(&self) -> String {
        String::from("DISCONNECT")
    }

    fn variable_header_properties(&self) -> &Option<Vec<Property>> {
        &self.packet.variable_header_properties
    }

    fn set_variable_header_properties(&mut self, p: Option<Vec<Property>>) {
        self.packet.variable_header_properties = p;
    }
}

impl BuilderLifecycle<Disconnect, Error> for DisconnectBuilder {
    fn new() -> Self {
        Default::default()
    }

    fn build(self) -> Result<Disconnect, Error> {
        let packet = self.packet;
        Ok(packet)
    }
}
