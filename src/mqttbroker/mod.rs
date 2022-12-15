pub mod packets;
pub mod primitive_types;
pub mod properties;

pub mod mqtt_broker {

    // pub mod reason_codes {
    //     pub const SUCCESS: u8 = 0x00;
    //     pub const NORMAL_DISCONNECTION: u8 = 0x00;
    //     pub const GRANTED_QOS_0: u8 = 0x00;
    //     pub const GRANTED_QOS_1: u8 = 0x01;
    //     pub const GRANTED_QOS_2: u8 = 0x02;
    //     pub const DISCONNECT_WITH_WILL_MESSAGE: u8 = 0x04;
    //     pub const NO_MATCHING_SUBSCRIBERS: u8 = 0x10;
    //     pub const NO_SUBSCRIPTION_EXISTED: u8 = 0x11;
    //     pub const CONTINUE_AUTHENTICATION: u8 = 0x18;
    //     pub const RE_AUTHENTICATE: u8 = 0x19;
    //     pub const UNSPECIFIED_ERROR: u8 = 0x80;
    //     pub const MALFORMED_PACKET: u8 = 0x81;
    //     pub const PROTOCOL_ERROR: u8 = 0x82;
    //     pub const IMPLEMENTATION_SPECIFIC_ERROR: u8 = 0x83;
    //     pub const UNSUPPORTED_PROTOCOL_VERSION: u8 = 0x84;
    //     pub const CLIENT_IDENTIFIER_NOT_VALID: u8 = 0x85;
    //     pub const BAD_USER_NAME_OR_PASSWORD: u8 = 0x86;
    //     pub const NOT_AUTHORIZED: u8 = 0x87;
    //     pub const SERVER_UNAVAILABLE: u8 = 0x88;
    //     pub const SERVER_BUSY: u8 = 0x89;
    //     pub const BANNED: u8 = 0x8a;
    //     pub const SERVER_SHUTTING_DOWN: u8 = 0x8B;
    //     pub const BAD_AUTHENTICATION_METHOD: u8 = 0x8c;
    //     pub const KEEP_ALIVE_TIMEOUT: u8 = 0x8d;
    //     pub const SESSION_TAKEN_OVER: u8 = 0x8e;
    //     pub const TOPIC_FILTER_INVALID: u8 = 0x8f;
    //     pub const TOPIC_NAME_INVALID: u8 = 0x90;
    //     pub const PACKET_IDENTIFIER_IN_USE: u8 = 0x91;
    //     pub const PACKET_IDENTIFIER_NOT_FOUND: u8 = 0x92;
    //     pub const RECEIVE_MAXIMUM_EXCEEDED: u8 = 0x93;
    //     pub const TOPIC_ALIAS_INVALID: u8 = 0x94;
    //     pub const PACKET_TOO_LARGE: u8 = 0x95;
    //     pub const MESSAGE_RATE_TOO_HIGH: u8 = 0x96;
    //     pub const QUOTA_EXCEEDED: u8 = 0x97;
    //     pub const ADMINISTRATIVE_ACTION: u8 = 0x98;
    //     pub const PAYLOAD_FORMAT_INVALID: u8 = 0x99;
    //     pub const RETAIN_NOT_SUPPORTED: u8 = 0x9a;
    //     pub const QOS_NOT_SUPPORTED: u8 = 0x9b;
    //     pub const USE_ANOTHER_SERVER: u8 = 0x9c;
    //     pub const SERVER_MOVED: u8 = 0x9d;
    //     pub const SHARED_SUBSCRIPTIONS_NOT_SUPPORTED: u8 = 0x9e;
    //     pub const CONNECTION_RATE_EXCEEDED: u8 = 0x9f;
    //     pub const MAXIMUM_CONNECT_TIME: u8 = 0xa0;
    //     pub const SUBSCRIPTION_IDENTIFIERS_NOT_SUPPORTED: u8 = 0xa1;
    //     pub const WILDCARD_SUBSCRIPTIONS_NOT_SUPPORTED: u8 = 0xa2;
    // }

    pub mod utility {
        // use tracing::debug;
        //
        // use std::collections::HashSet;

        // fn concat(mut set: HashSet<u8>, subset: &HashSet<u8>) -> HashSet<u8> {
        //     for x in subset {
        //         set.insert(*x);
        //     }
        //
        //     set
        // }
        //
        // fn valid_reason_codes() -> Vec<u8> {
        //     todo!()
        // }

        #[cfg(test)]
        mod packets_tests {
            #[test]
            fn test1() {
                // impl ServerKeepAlive for PropertyMap {};
                //
                // let mut server_keep_alive = PropertyMap::new();
                // server_keep_alive.set_server_keep_alive(TwoByteInteger(213));
                // let read = server_keep_alive.server_keep_alive();
                // assert_eq!(TwoByteInteger(213), *read.unwrap());
                //
                // let publish_props = P
                // impl SessionExpiryInterval for PublishProperties {};
                //
                // let publish_properties = PublishProperties::new(HashMap::new());
                // publish_properties
            }
        }

        mod server {
            #[cfg(test)]
            mod test {
                use crate::mqttbroker::packets::connect::{Connect, ConnectBuilder};
                use crate::mqttbroker::packets::{BuilderLifecycle, Decoder, Encoder};
                use bytes::BytesMut;
                use tracing::debug;

                #[test]
                fn test_connect() {
                    // generate packets
                    let original_connect_packet = ConnectBuilder::new().will_message(
                        vec![],
                        "/topic".to_string(),
                        vec![77, 66, 12],
                    );
                    let original_connect_packet_clone = original_connect_packet.packet.clone();
                    let encoded_packet = original_connect_packet.build();
                    let mut encoded_packet = encoded_packet.unwrap().encode().unwrap();
                    println!("encoded packets is {:?}", encoded_packet);
                    //decode packets

                    let decoded_connect_packet = Connect::decode(&mut encoded_packet).unwrap();

                    assert_eq!(original_connect_packet_clone, decoded_connect_packet);
                    println!("original packets is {:?}", original_connect_packet_clone);
                    println!("decoded packets is {:?}", decoded_connect_packet);
                }

                #[test]
                fn connect_properties_are_valid() {
                    // don't have a test for checking the properties for a connect packets. This test
                    // needs to be written properly to reflect that functionality check
                    assert!(true)
                }
            }
        }

        // async fn process<T>(socket: T) {
        //     loop {}
        // }
        //
        // struct MqttBroker<'a> {
        //     addr: &'a str,
        // }
        //
        // impl MqttBroker<'_> {
        //     async fn run(self) -> Result<(), Error> {
        //         let mut listener = TcpListener::bind(self.addr).await.unwrap();
        //
        //         loop {
        //             let (mut socket, _) = listener.accept().await?;
        //             tokio::spawn(async move {
        //                 // do something here
        //                 process(socket).await;
        //             });
        //         }
        //     }
        // }
    }
}
