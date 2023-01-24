use crate::decode::{decode_property, utf8_string, varint};
use crate::encode::utf8_encoded_string;
use crate::mqttbroker::packets::subscribe::{
    Subscribe, SubscriptionOptions, TopicFilterAndSubscriptionOptions,
};
use crate::mqttbroker::packets::{encode_properties, Decoder, Encoder, GeneratePacketParts};
use bytes::{Buf, BufMut, BytesMut};

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
