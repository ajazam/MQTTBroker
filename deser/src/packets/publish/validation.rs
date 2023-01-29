pub mod encode_validation {}

pub mod decode_validate {
    use crate::packets::publish::builder::PublishBuilder;
    use crate::packets::publish::PublishError;

    //[MQTT-2.2.1-2]
    fn is_qos_and_packet_id_valid(s: &PublishBuilder) -> Result<(), PublishError> {
        if (1u8..=2).contains(&s.packet.qos_number()) && s.packet.packet_id.is_none() {
            return Err(PublishError::NoPacketIdForQos(s.packet.qos_number()));
        }

        Ok(())
    }

    //[MQTT-3.3.1-2]
    fn is_dup_valid(s: &PublishBuilder) -> Result<(), PublishError> {
        if s.packet.dup() && s.packet.qos_number() == 0 {
            return Err(PublishError::DupInvalidForQos0);
        }

        Ok(())
    }

    //[MQTT-3.3.1-4]
    fn is_both_qos_bits_set(s: &PublishBuilder) -> Result<(), PublishError> {
        if s.packet.qos_number() == 3 {
            return Err(PublishError::BothQosBitsAreSet);
        }

        Ok(())
    }

    fn is_topic_name_present(s: &PublishBuilder) -> Result<(), PublishError> {
        if s.packet.topic_name.is_empty() {
            return Err(PublishError::TopicNotPresent);
        }

        Ok(())
    }

    pub fn validation_checks(s: &PublishBuilder) -> Result<(), Vec<PublishError>> {
        //todo check for any more outstanding tests
        let mut errors = vec![];

        let ret = is_qos_and_packet_id_valid(s);
        if ret.is_err() {
            errors.push(ret.err().unwrap())
        }

        let ret = is_dup_valid(s);
        if ret.is_err() {
            errors.push(ret.err().unwrap())
        }

        let ret = is_both_qos_bits_set(s);
        if ret.is_err() {
            errors.push(ret.err().unwrap());
        }

        let ret = is_topic_name_present(s);
        if ret.is_err() {
            errors.push(ret.err().unwrap())
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(())
    }
}
