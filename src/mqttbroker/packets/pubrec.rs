use crate::mqttbroker::packets::properties::PubrecProperties;
use crate::mqttbroker::packets::reason_codes;

pub struct PubRec {
    //fixed header
    packet_type: u8,

    //variable header
    packet_id: u16,
    reason_code: reason_codes::PUBREC,
    property: PubrecProperties, // if the remaining length is 4 then property length is zero

                                //payload
                                //no payload
}
