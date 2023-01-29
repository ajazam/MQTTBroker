use crate::packets::pingresp::PingResp;
use crate::packets::{BuilderLifecycle, GeneratePacketParts};
use bytes::BytesMut;
use std::io::Error;

#[derive(Debug, Clone, Default)]
pub struct PingRespBuilder {
    pub packet: PingResp,
}

impl GeneratePacketParts for PingResp {
    fn generate_variable_header(&self) -> BytesMut {
        // no variable header
        BytesMut::with_capacity(0)
    }

    fn generate_payload(&self) -> BytesMut {
        // no payload
        BytesMut::with_capacity(0)
    }
}

impl BuilderLifecycle<PingResp, Error> for PingRespBuilder {
    fn new() -> Self {
        Default::default()
    }

    fn build(self) -> Result<PingResp, Error> {
        let packet = self.packet;
        Ok(packet)
    }
}
