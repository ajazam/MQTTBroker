use crate::decode::{decode_property, utf8_string, varint};
use crate::encode::utf8_encoded_string;
use crate::mqttbroker::packets::publish::Publish;
use crate::mqttbroker::packets::{encode_properties, Decoder, Encoder, GeneratePacketParts};
use bytes::{Buf, BufMut, BytesMut};

impl GeneratePacketParts for Publish {
    fn generate_variable_header(&self) -> BytesMut {
        // fields are Topic Name, Packet Identifier, Properties
        let mut variable_header = BytesMut::with_capacity(200);
        //encode topic name
        utf8_encoded_string("topic", &self.topic_name, &mut variable_header);
        //encode packet identifier
        if (1..=2).contains(&self.qos_number()) {
            variable_header.put_u16(self.packet_id.unwrap())
        }

        variable_header = encode_properties(variable_header, &self.variable_header_properties);

        variable_header
    }

    fn generate_payload(&self) -> BytesMut {
        // fields are Application Message
        let mut payload = BytesMut::with_capacity(200);
        if self.application_message.is_some() {
            let local_appmessage = self.application_message.clone();
            payload.put(local_appmessage.unwrap().as_slice());
        }

        println!("encode payload is {payload:?}",);

        payload
    }
}

impl Encoder<Publish> for Publish {}

impl Decoder<Publish> for Publish {
    fn decode(bytes: &mut BytesMut) -> anyhow::Result<Publish> {
        let packet_type_with_flags = bytes.get_u8();
        let packet_type = packet_type_with_flags >> 4;
        let packet_type_low_nibble = packet_type_with_flags & 0x0f;
        let packet_size = varint(bytes).unwrap();
        let topic_name = utf8_string(String::from("topic_name"), bytes).unwrap();
        // 0b000_0110 mask for Qos Level.
        let qos = packet_type_low_nibble & 0b0000_0110 >> 1;
        println!("qos is {qos}");
        let packet_id = if (1..=2).contains(&qos) {
            println!("got packet_id");
            Some(bytes.get_u16())
        } else {
            println!("packet _id isnot present");
            None
        };

        let variable_header_properties = decode_property(bytes);

        let application_message = if !bytes.is_empty() {
            Some(bytes.to_vec())
        } else {
            None
        };

        Ok(Publish {
            packet_type,
            packet_type_low_nibble,
            topic_name,
            packet_id,
            variable_header_properties,
            application_message,
        })
    }
}
