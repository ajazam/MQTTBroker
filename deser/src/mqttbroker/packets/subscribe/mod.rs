mod builder;
mod deser;
mod validation;

use crate::encode::utf8_encoded_string;

use crate::decode::{decode_property, utf8_string, varint};
use crate::mqttbroker::packets::subscribe::builder::{
    SubscribeBuilder, SubscriptionOptionsBuilder,
};
use crate::mqttbroker::packets::{
    encode_properties, BuilderLifecycle, Decoder, Encoder, GeneratePacketParts, PacketTypes,
    Properties,
};
use crate::mqttbroker::properties::Property;
use bytes::{Buf, BufMut, BytesMut};
use std::io::Error;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Subscribe {
    //fixed header
    packet_type: u8,
    packet_type_low_nibble: u8,

    //variable header
    packet_id: u16,
    variable_header_properties: Option<Vec<Property>>,

    //payload
    topic_filters: Vec<TopicFilterAndSubscriptionOptions>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TopicFilterAndSubscriptionOptions {
    pub topic_filter: String,
    pub subscription_options: SubscriptionOptions,
}

impl TopicFilterAndSubscriptionOptions {
    pub fn new(topic_filter: String, subscription_options: SubscriptionOptions) -> Self {
        TopicFilterAndSubscriptionOptions {
            topic_filter,
            subscription_options,
        }
    }

    pub fn qos(&self) -> u8 {
        self.subscription_options.raw_value & 0b0000_0011
    }

    pub fn no_local(&self) -> bool {
        self.subscription_options.raw_value & 0b0000_0100 == 0b0000_0100
    }

    pub fn retain_as_published(&self) -> bool {
        self.subscription_options.raw_value & 0b0000_1000 == 0b0000_1000
    }

    /// return value.
    /// 0 = Send retained messages at the time of the subscribe.
    /// 1 = Send retained messages at subsvribe only if the subscription does not current exist.
    /// 2 = Do not send retained messages at the time of the subscribe.
    /// 3 = Protocol error.
    pub fn retain_handling(&self) -> u8 {
        (self.subscription_options.raw_value & 0b0011_0000) >> 4
    }
}

impl Default for TopicFilterAndSubscriptionOptions {
    fn default() -> Self {
        TopicFilterAndSubscriptionOptions {
            topic_filter: "".to_string(),
            subscription_options: SubscriptionOptions { raw_value: 0 },
        }
    }
}

impl Default for Subscribe {
    fn default() -> Self {
        Subscribe {
            packet_type: PacketTypes::Subscribe as u8,
            packet_type_low_nibble: 0,
            packet_id: 0,
            variable_header_properties: None,
            topic_filters: vec![],
        }
    }
}

#[repr(u8)]
pub enum QOS {
    Qos0 = 0b0000_0000,
    Qos1 = 0b0000_0001,
    Qos2 = 0b0000_0010,
    // protocol error = 0b0000_0011
}

#[repr(u8)]
pub enum RetainHandlingOptions {
    SendRetainedMessagesAtTheTimeOfSubscribe = 0b0000_0000,
    SendRetainedMessagesAtSubscribeOnlyIfTheSubscriptionDoesNotCurrentlyExist = 0b0001_0000,
    DoNotSendRetainedMessagesAtTheTimeOfTheSubscribe = 0b0010_0000,
    // protocol error = 0b0011_0000
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct SubscriptionOptions {
    pub raw_value: u8,
}

#[cfg(test)]
pub mod test {
    use crate::mqttbroker::packets::subscribe::{
        RetainHandlingOptions, Subscribe, SubscribeBuilder, SubscriptionOptions,
        SubscriptionOptionsBuilder, TopicFilterAndSubscriptionOptions, QOS,
    };
    use crate::mqttbroker::packets::{BuilderLifecycle, Decoder, Encoder};
    use crate::mqttbroker::primitive_types::VariableByteInteger;
    use crate::mqttbroker::properties::Property;

    #[test]
    pub fn should_encode_decode_packet() {
        let mut original_packet = SubscribeBuilder::new();
        let props = vec![Property::SubscriptionIdentifier(VariableByteInteger(100))];
        original_packet = original_packet.set_packet_id(100);
        let mut subscriptions_options_builder = SubscriptionOptionsBuilder::new();
        subscriptions_options_builder = subscriptions_options_builder
            .set_no_local(true)
            .set_retain_handling_option(
                RetainHandlingOptions::DoNotSendRetainedMessagesAtTheTimeOfTheSubscribe,
            )
            .set_qos(QOS::Qos1);
        let subscription_options = subscriptions_options_builder.build().unwrap();

        let topic_filter = vec![TopicFilterAndSubscriptionOptions::new(
            String::from("/#"),
            subscription_options,
        )];
        original_packet = original_packet.set_topic_filter(topic_filter);

        let built_packet = original_packet.build().unwrap();
        let mut serialized_packet = Subscribe::encode(
            built_packet.packet_type,
            built_packet.packet_type_low_nibble,
            &built_packet,
        )
        .unwrap();

        let deserialized_packet = Subscribe::decode(&mut serialized_packet).unwrap();
        assert_eq!(built_packet, deserialized_packet);
    }
}
