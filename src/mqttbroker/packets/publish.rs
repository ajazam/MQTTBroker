use crate::mqttbroker::packets::properties::PublishProperties;

pub struct Publish {
    //fixed header
    packet_type: u8,

    //variable_header
    topic_name: String,
    packet_id: u16,
    property: PublishProperties,

    //payload
    application_message: Vec<u8>,
}
