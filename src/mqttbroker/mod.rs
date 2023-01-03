pub mod packets;
pub mod primitive_types;
pub mod properties;

pub mod mqtt_broker {

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
                use crate::mqttbroker::packets::connect::{Builder, Connect};
                use crate::mqttbroker::packets::{BuilderLifecycle, Decoder, Encoder};
                use bytes::BytesMut;
                use tracing::{debug, trace};

                #[test]
                fn test_connect() {
                    // generate packets

                    let mut original_connect_packet = Builder::new();
                    let res = original_connect_packet.will_message(
                        &vec![],
                        "/topic".to_string(),
                        vec![77, 66, 12],
                    );

                    let original_connect_packet_clone = original_connect_packet.packet.clone();
                    let encoded_packet = original_connect_packet.build();
                    let mut encoded_packet = encoded_packet.unwrap().encode().unwrap();
                    trace!("encoded packets is {encoded_packet:?}");
                    //decode packets

                    let decoded_connect_packet = Connect::decode(&mut encoded_packet).unwrap();

                    assert_eq!(original_connect_packet_clone, decoded_connect_packet);
                    trace!("original packets is {original_connect_packet_clone:?}");
                    trace!("decoded packets is {decoded_connect_packet:?}");
                }

                #[test]
                fn connect_properties_are_valid() {
                    // don't have a test for checking the properties for a connect packets. This test
                    // needs to be written properly to reflect that functionality check
                    assert!(true);
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
