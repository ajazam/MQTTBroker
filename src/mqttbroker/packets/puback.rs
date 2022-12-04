use crate::mqttbroker::packets::properties::PubackProperties;
use crate::mqttbroker::packets::reason_codes;

pub struct PubAck {
    //fixed header
    packet_type: u8,

    //variable header
    packet_id: u16,
    reason_code: reason_codes::PUBACK,
    // not available if remaining length in fixed header
    // is 2, which means there is only a packet_id in variable header. If there is no Reason code then 0x00(Success) used by the client.
    property: Option<PubackProperties>,
    //payload
    //no payload
}
