use crate::mqttbroker::packets::properties::SubackProperties;
use crate::mqttbroker::packets::reason_codes;

pub struct SubAck {
    //fixed header
    packet_type: u8,

    //variable header
    packet_id: u16,
    property: SubackProperties,

    //payload
    reason_code: reason_codes::SUBACK,
}
