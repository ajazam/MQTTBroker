use crate::mqttbroker::packets::pingreq::PingReq;
use crate::mqttbroker::packets::{BuilderLifecycle, GeneratePacketParts};
use bytes::BytesMut;
use std::io::Error;

#[derive(Debug, Clone, Default)]
pub struct PingReqBuilder {
    pub packet: PingReq,
}

impl GeneratePacketParts for PingReq {
    fn generate_variable_header(&self) -> BytesMut {
        // no variable header
        BytesMut::with_capacity(0)
    }

    fn generate_payload(&self) -> BytesMut {
        // no payload
        BytesMut::with_capacity(0)
    }
}

impl BuilderLifecycle<PingReq, Error> for PingReqBuilder {
    fn new() -> Self {
        Default::default()
    }

    fn build(self) -> Result<PingReq, Error> {
        let packet = self.packet;
        Ok(packet)
    }
}
