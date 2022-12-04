use crate::mqttbroker::packets::properties::AuthProperties;
use crate::mqttbroker::packets::reason_codes;

pub struct Auth {
    //fixed header
    packet_type: u8,

    // variable header
    connect_ack_flags: u8,
    connect_reason_code: reason_codes::AUTH,
    property: Vec<AuthProperties>,
    //payload
    // no payload
}
