use crate::mqttbroker::packets::reason_codes;
use crate::mqttbroker::properties::Property;

pub struct ConnAck {
    //fixed header
    packet_type: u8,

    // variable header
    connect_ack_flags: u8,
    connect_reason_code: reason_codes::CONNECTACK,
    property: Vec<Property>,
    //payload
    // no payload
}

impl ConnAck {
    const SESSION_PRESENT: u8 = 1;
    pub fn session_present(&mut self) -> bool {
        self.connect_ack_flags & ConnAck::SESSION_PRESENT == ConnAck::SESSION_PRESENT
    }
    pub fn set_session_present(&mut self, session_present: bool) {
        if session_present {
            self.connect_ack_flags |= ConnAck::SESSION_PRESENT;
        }
        self.connect_ack_flags = 0;
    }

    pub fn set_connect_reason_code(mut self, reason_code: reason_codes::CONNECTACK) {
        self.connect_reason_code = reason_code;
    }
}
