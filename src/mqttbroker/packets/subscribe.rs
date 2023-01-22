use crate::encode::utf8_encoded_string;

use crate::decode::{decode_property, utf8_string, varint};
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

pub mod encode_validation {}

pub mod decode_validate {}

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

impl GeneratePacketParts for Subscribe {
    fn generate_variable_header(&self) -> BytesMut {
        // need to optimise the capacity value
        let mut variable_header = BytesMut::with_capacity(200);
        // packet_identifier
        variable_header.put_u16(self.packet_id);
        // variable header properties
        variable_header = encode_properties(variable_header, &self.variable_header_properties);
        variable_header
    }

    fn generate_payload(&self) -> BytesMut {
        let mut payload = BytesMut::with_capacity(100);
        for filter in self.topic_filters.clone() {
            utf8_encoded_string("topic filter", &filter.topic_filter, &mut payload);
            payload.put_u8(filter.subscription_options.raw_value);
        }
        payload
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

impl Encoder<Subscribe> for Subscribe {}

impl Decoder<Subscribe> for Subscribe {
    fn decode(bytes: &mut BytesMut) -> anyhow::Result<Subscribe> {
        // fixed header
        let packet_with_flags = bytes.get_u8();
        let packet_type = packet_with_flags >> 4;
        let packet_type_low_nibble = packet_with_flags & 0x0f;
        // remaining length
        let _packet_size = varint(bytes).unwrap();

        // packet_Identifier
        let packet_id = bytes.get_u16();

        // variable header properties
        let variable_header_properties = decode_property(bytes);
        // topic filters
        let mut topic_filters: Vec<TopicFilterAndSubscriptionOptions> = vec![];
        while !bytes.is_empty() {
            let topic_filter = utf8_string(String::from("Topic Filter"), bytes);
            let subscription_options_raw = bytes.get_u8();
            let subscription_options = SubscriptionOptions {
                raw_value: subscription_options_raw,
            };
            topic_filters.push(TopicFilterAndSubscriptionOptions::new(
                topic_filter.unwrap(),
                subscription_options,
            ));
        }

        Ok(Subscribe {
            packet_type,
            packet_type_low_nibble,
            packet_id,
            variable_header_properties,
            topic_filters,
        })
    }
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
