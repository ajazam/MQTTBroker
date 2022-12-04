use crate::mqttbroker::packets::properties::DisconnectProperties;
use crate::mqttbroker::packets::reason_codes;

pub struct Disconnect {
    //fixed header
    packet_type: u8,

    // variable header
    connect_ack_flags: u8,
    disconnect_reason_code: reason_codes::DISCONNECT,

    property: Vec<DisconnectProperties>,
    //payload
    // no payload
}
