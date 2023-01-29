use crate::packets::subscribe::{
    RetainHandlingOptions, Subscribe, SubscriptionOptions, TopicFilterAndSubscriptionOptions, QOS,
};
use crate::packets::{BuilderLifecycle, PacketTypes, Properties};
use crate::properties::Property;
use std::io::Error;

#[derive(Debug, Clone, Default)]
pub struct SubscribeBuilder {
    pub packet: Subscribe,
}

impl SubscribeBuilder {
    pub fn set_packet_id(mut self, packet_id: u16) -> Self {
        self.packet.packet_id = packet_id;
        self
    }
    pub fn set_topic_filter(
        mut self,
        topic_filters: Vec<TopicFilterAndSubscriptionOptions>,
    ) -> Self {
        self.packet.topic_filters = topic_filters;
        self
    }
}

impl Properties for SubscribeBuilder {
    fn packet_type(&self) -> PacketTypes {
        PacketTypes::Subscribe
    }

    fn packet_type_string(&self) -> String {
        String::from("SUBSCRIBE")
    }

    fn variable_header_properties(&self) -> &Option<Vec<Property>> {
        &self.packet.variable_header_properties
    }

    fn set_variable_header_properties(&mut self, p: Option<Vec<Property>>) {
        self.packet.variable_header_properties = p;
    }
}

impl BuilderLifecycle<Subscribe, Error> for SubscribeBuilder {
    fn new() -> Self {
        Default::default()
    }

    fn build(self) -> Result<Subscribe, Error> {
        let packet = self.packet;
        Ok(packet)
    }
}

#[derive(Debug, Clone, Default)]
pub struct SubscriptionOptionsBuilder {
    pub subscription_options: SubscriptionOptions,
}

impl SubscriptionOptionsBuilder {
    pub fn set_qos(mut self, qos: QOS) -> Self {
        // 0b000_0011 qos bits
        self.subscription_options.raw_value |= 0b0000_0011 & (qos as u8);
        self
    }

    pub fn set_no_local(mut self, set: bool) -> Self {
        // 0b000_0100 No Local option
        if set {
            self.subscription_options.raw_value |= 0b0000_0100;
        } else {
            self.subscription_options.raw_value |= 0b1111_1011;
        }

        self
    }

    pub fn set_retain_as_published(mut self, set: bool) -> Self {
        // 0b000_1000 No Local option
        if set {
            self.subscription_options.raw_value |= 0b0000_1000;
        } else {
            self.subscription_options.raw_value |= 0b1111_0111;
        }

        self
    }

    pub fn set_retain_handling_option(
        mut self,
        retain_handling_option: RetainHandlingOptions,
    ) -> Self {
        self.subscription_options.raw_value |= 0b0011_0000 & (retain_handling_option as u8);
        self
    }
}

impl BuilderLifecycle<SubscriptionOptions, Error> for SubscriptionOptionsBuilder {
    fn new() -> Self {
        Default::default()
    }

    fn build(self) -> Result<SubscriptionOptions, Error> {
        let subscription_options = self.subscription_options;
        Ok(subscription_options)
    }
}
