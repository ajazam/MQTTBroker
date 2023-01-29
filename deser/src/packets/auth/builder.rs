use crate::packets::auth::Auth;
use crate::packets::reason_codes::AUTH;
use crate::packets::{BuilderLifecycle, PacketTypes, Properties};
use crate::properties::Property;
use std::io::Error;

#[derive(Debug, Clone, Default)]
pub struct AuthBuilder {
    pub packet: Auth,
}

impl AuthBuilder {
    pub fn set_reason_code(mut self, reason_code: AUTH) -> Self {
        self.packet.reason_code = reason_code;
        self
    }
}

impl Properties for AuthBuilder {
    fn packet_type(&self) -> PacketTypes {
        PacketTypes::Auth
    }

    fn packet_type_string(&self) -> String {
        String::from("AUTH")
    }

    fn variable_header_properties(&self) -> &Option<Vec<Property>> {
        &self.packet.variable_header_properties
    }

    fn set_variable_header_properties(&mut self, p: Option<Vec<Property>>) {
        self.packet.variable_header_properties = p;
    }
}

impl BuilderLifecycle<Auth, Error> for AuthBuilder {
    fn new() -> Self {
        Default::default()
    }

    fn build(self) -> Result<Auth, Error> {
        let packet = self.packet;
        Ok(packet)
    }
}
