pub mod encode_validation {}

pub mod decode_validate {

    use crate::packets::connect::Connect;
    use crate::packets::ConnectPacketBuildError::{
        WillFlagNotSet, WillPayLoadNotSet, WillTopicNotSet,
    };
    pub fn client_id_correctly_formatted(str: String) -> bool {
        for c in str.chars() {
            if !c.is_ascii_alphanumeric() {
                return false;
            }
        }

        true
    }

    pub fn will_properties_are_valid(connect_packet: &Connect) -> anyhow::Result<()> {
        if is_will_flag_not_set_and_will_properties_set_error(connect_packet) {
            return Err(WillFlagNotSet.into());
        }

        if is_will_flag_set_and_will_topic_is_empty_error(connect_packet) {
            return Err(WillTopicNotSet.into());
        }

        if is_will_flag_set_and_will_payload_is_empty_error(connect_packet) {
            return Err(WillPayLoadNotSet.into());
        }

        Ok(())
    }

    fn is_will_flag_not_set_and_will_properties_set_error(connect_packet: &Connect) -> bool {
        if connect_packet.will_flag() {
            return false;
        }

        let ret_value = !connect_packet.will_flag()
            && connect_packet.will_properties.is_some()
            && connect_packet.will_properties.as_ref().is_some();

        ret_value
    }

    fn is_will_flag_set_and_will_topic_is_empty_error(connect_packet: &Connect) -> bool {
        if !connect_packet.will_flag() {
            return false;
        }

        if connect_packet.will_flag() && (connect_packet.will_topic.is_none()) {
            return true;
        }

        if connect_packet.will_flag()
            && connect_packet.will_topic.is_some()
            && connect_packet.will_topic.as_ref().unwrap().is_empty()
        {
            return true;
        }

        false
    }

    fn is_will_flag_set_and_will_payload_is_empty_error(connect_packet: &Connect) -> bool {
        if !connect_packet.will_flag() {
            return false;
        }

        if connect_packet.will_payload.is_some()
            && connect_packet.will_payload.as_ref().unwrap().is_empty()
        {
            return true;
        }

        if connect_packet.will_flag() {
            return connect_packet.will_payload.is_none();
        }

        if connect_packet.will_payload.is_none() {
            return true;
        }

        false
    }

    fn is_will_flag_not_set_and_qos_not_0_error(connect_packet: &Connect) -> bool {
        if connect_packet.will_flag() {
            return false;
        }

        !connect_packet.will_flag() && connect_packet.will_qos_flag() == 0
    }

    #[cfg(test)]
    pub mod test {

        #[test]
        fn should_client_id_correctly_formatted() {
            assert!(client_id_correctly_formatted(String::from("12345ABCabc")));
        }

        #[test]
        fn should_be_incorrectly_formatted_client_id() {
            assert!(!client_id_correctly_formatted(String::from(
                "12345ABCabc_!£$%^&*()"
            )));
        }

        use crate::packets::connect::validation::decode_validate::{
            client_id_correctly_formatted, is_will_flag_not_set_and_will_properties_set_error,
            is_will_flag_set_and_will_payload_is_empty_error,
            is_will_flag_set_and_will_topic_is_empty_error,
        };
        use crate::packets::connect::Connect;
        use crate::packets::connect_flags;
        use crate::primitive_types::FourByteInteger;
        use crate::properties::Property;
        use tracing::trace;

        #[test]
        fn test_is_will_flag_not_set_and_will_properties_set() {
            let connect_packet = Connect {
                will_topic: Some(String::from("hello")),
                will_payload: Some(vec![1, 2, 3]), //will_properties: Some(vec![Property::new()])..Default::default(),
                will_properties: Some(vec![Property::WillDelayInterval(FourByteInteger(400))]),
                ..Default::default()
            };

            trace!("connect_packet is {connect_packet:?}");

            //let ret = connect_packet.will_flag();

            let result = is_will_flag_not_set_and_will_properties_set_error(&connect_packet);

            assert!(result);
        }

        #[test]
        fn test_is_will_flag_set_and_topic_is_empty() {
            let connect_packet = Connect {
                connect_flags: connect_flags::WILL_FLAG,
                will_topic: Some(String::from("")),
                ..Default::default()
            };
            trace!("{:?}", connect_packet);
            assert!(is_will_flag_set_and_will_topic_is_empty_error(
                &connect_packet
            ));
        }

        #[test]
        fn test_is_will_flag_set_and_will_payload_is_empty() {
            let connect_packet = Connect {
                connect_flags: connect_flags::WILL_FLAG,
                will_payload: None,
                ..Default::default()
            };
            assert!(is_will_flag_set_and_will_payload_is_empty_error(
                &connect_packet
            ));
        }
    }
}
