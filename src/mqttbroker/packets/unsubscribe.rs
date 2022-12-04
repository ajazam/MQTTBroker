use crate::mqttbroker::packets::properties::UnSubscribeProperties;

pub struct UnSubscribe {
    //fixed header
    packet_type: u8,

    //variable header
    packet_id: u16,
    property: UnSubscribeProperties,

    //payload
    topic_filter: Vec<String>,
}
