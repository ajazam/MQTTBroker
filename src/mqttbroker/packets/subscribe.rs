use crate::mqttbroker::packets::properties::SubscribeProperties;

pub struct Subscribe {
    //fixed header
    packet_type: u8,

    //variable header
    packet_id: u16,
    property: SubscribeProperties,

    //payload
    topic_filter: Vec<String>,
}
