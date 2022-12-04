use crate::mqttbroker::packets::properties::UnsubackProperties;
use crate::mqttbroker::packets::reason_codes;

pub struct UnsubAck {
    //fixed header
    packet_type: u8,

    //variable header
    packet_id: u16,
    property: UnsubackProperties,

    //payload
    topic_filter: Vec<reason_codes::UNSUBACK>,
}
