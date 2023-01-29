use crate::packets::connect::Connect;
use crate::packets::error::PropertyError;
use crate::packets::{connect_flags, BuilderLifecycle, PacketTypes, Properties};
use crate::properties::{invalid_property, non_unique, valid_properties_for_will, Property};
use std::io::Error;

#[derive(Debug, Clone, Default)]
pub struct ConnectBuilder {
    pub packet: Connect,
}

impl ConnectBuilder {
    //Don't need to specify version number because the code is for v5.0

    pub fn set_keep_alive(mut self, keep_alive: u16) -> Self {
        self.packet.keep_alive = keep_alive;
        self
    }

    pub fn set_will_retain(mut self, retain: bool) -> Self {
        if retain {
            self.packet.connect_flags |= connect_flags::WILL_RETAIN;
            return self;
        }

        self.packet.connect_flags &= !connect_flags::WILL_RETAIN;
        self
    }

    pub fn set_will_qos(&mut self, qos: u8) {
        let new_qos = if qos > 2 { 2 } else { qos };
        self.packet.connect_flags &= !connect_flags::WILL_QOS_MASK | new_qos << 3;
    }

    pub fn clean_start(mut self, b: bool) -> Self {
        if b {
            self.packet.connect_flags |= 2;
            return self;
        }

        self.packet.connect_flags = !2;
        self
    }

    pub fn client_id(mut self, ci: String) -> Self {
        self.packet.client_id = ci;
        self
    }

    fn will_properties(
        &mut self,
        assigned_will_properties: &Vec<Property>,
    ) -> Result<(), PropertyError> {
        let mut properties: Vec<Property> = vec![];

        for p in assigned_will_properties {
            properties.push(p.clone())
        }

        // check for invalid
        let mut invalid_will_properties: Vec<Property> = vec![];
        invalid_property(
            &assigned_will_properties,
            valid_properties_for_will().as_slice(),
            &mut invalid_will_properties,
        );

        // will properties are only used in the CONNECT packets
        if !invalid_will_properties.is_empty() {
            return Err(PropertyError::InvalidProperty(
                invalid_will_properties,
                String::from("CONNECT"),
            ));
        };

        // check for duplicates
        let non_unique_properties = non_unique(&assigned_will_properties);
        if !non_unique_properties.is_empty() {
            return Err(PropertyError::PropertyAlreadyInserted(
                non_unique_properties,
                String::from("CONNECT"),
            ));
        }

        self.packet.will_properties = Some(assigned_will_properties.clone());
        Ok(())
    }

    fn will_topic(mut self, topic: String) -> Self {
        self.packet.will_topic = Some(topic);
        self
    }

    fn will_payload(mut self, will_payload: Vec<u8>) -> Self {
        self.packet.will_payload = Some(will_payload);
        self
    }

    pub fn will_message(
        &mut self,
        will_properties: &Vec<Property>,
        will_topic: String,
        will_payload: Vec<u8>,
    ) -> Result<(), PropertyError> {
        self.will_properties(will_properties)?;

        self.packet.will_topic = Some(will_topic);
        self.packet.will_payload = Some(will_payload);
        self.packet.connect_flags |= connect_flags::WILL_FLAG;

        Ok(())
    }

    pub fn username(mut self, username: Option<String>) -> Self {
        if username.is_some() && !username.as_ref().unwrap().is_empty() {
            self.packet.username = username;

            self.packet.connect_flags |= connect_flags::USER_NAME_FLAG;
        }
        self
    }

    pub fn password(mut self, password: Option<String>) -> Self {
        if password.is_some() && !password.as_ref().unwrap().is_empty() {
            self.packet.password = password;

            self.packet.connect_flags |= connect_flags::PASSWORD_FLAG;
        }

        self
    }

    pub fn generate_connect_flags(connect_packet: &Connect) -> u8 {
        let mut connect_flags = 0u8;
        if connect_packet.username.is_some() {
            connect_flags |= connect_flags::USER_NAME_FLAG;
        }

        if connect_packet.password.is_some() {
            connect_flags |= connect_flags::PASSWORD_FLAG;
        }

        connect_flags |= connect_packet.connect_flags;
        connect_flags
    }
}

impl Properties for ConnectBuilder {
    fn packet_type(&self) -> PacketTypes {
        PacketTypes::Connect
    }

    fn packet_type_string(&self) -> String {
        String::from("CONNECT")
    }

    fn variable_header_properties(&self) -> &Option<Vec<Property>> {
        &self.packet.variable_header_properties
    }

    fn set_variable_header_properties(&mut self, p: Option<Vec<Property>>) {
        self.packet.variable_header_properties = p;
    }
}

impl BuilderLifecycle<Connect, Error> for ConnectBuilder {
    fn new() -> ConnectBuilder {
        Default::default()
    }

    fn build(self) -> Result<Connect, Error> {
        let connect_packet = self.packet;
        Ok(connect_packet)
    }
}
