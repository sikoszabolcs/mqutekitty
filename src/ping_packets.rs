use crate::{
    control_packets::{ControlPacketFlags, Encodable, FixedHeader},
    ControlPacketType,
};

pub struct PingReqPacket {
    pub fixed_header: FixedHeader,
}

impl Encodable for PingReqPacket {
    fn encode(self) -> Vec<u8> {
        self.fixed_header.encode()
    }
}

impl PingReqPacket {
    pub fn new() -> PingReqPacket {
        PingReqPacket {
            fixed_header: FixedHeader {
                packet_type: ControlPacketType::PingReq,
                packet_flags: ControlPacketFlags::PING_REQ_FLAGS,
                remaining_length: 0,
            },
        }
    }
}

#[derive(Debug)]
pub struct PingRespPacket {
    pub fixed_header: FixedHeader,
}

impl From<&[u8]> for PingRespPacket {
    fn from(bytes: &[u8]) -> Self {
        PingRespPacket {
            fixed_header: FixedHeader::from(bytes),
        }
    }
}

impl Encodable for PingRespPacket {
    fn encode(self) -> Vec<u8> {
        self.fixed_header.encode()
    }
}

impl PingRespPacket {
    pub fn new() -> PingRespPacket {
        PingRespPacket {
            fixed_header: FixedHeader {
                packet_type: ControlPacketType::PingResp,
                packet_flags: ControlPacketFlags::PING_RESP_FLAGS,
                remaining_length: 0,
            },
        }
    }
}

#[cfg(test)]
mod ping_req_packet_tests {
    use crate::{
        control_packets::{ControlPacketFlags, ControlPacketType},
        ping_packets::{Encodable, PingReqPacket},
    };

    #[test]
    fn create_test() {
        let ping_req_packet = PingReqPacket::new();

        assert_eq!(
            ping_req_packet.fixed_header.packet_type,
            ControlPacketType::PingReq
        );
        assert_eq!(
            ping_req_packet.fixed_header.packet_flags,
            ControlPacketFlags::PING_REQ_FLAGS
        );
        assert_eq!(ping_req_packet.fixed_header.remaining_length, 0);
    }

    #[test]
    fn encode_test() {
        let ping_req_packet_bytes = PingReqPacket::new().encode();

        println!("{:?}", ping_req_packet_bytes);
        assert_eq!(ping_req_packet_bytes.len(), 2);
        assert_eq!(ping_req_packet_bytes[0], 0b1100_0000);
        assert_eq!(ping_req_packet_bytes[1], 0x00);
    }
}

#[cfg(test)]
mod ping_resp_packet_tests {
    use crate::{
        control_packets::{ControlPacketFlags, ControlPacketType},
        ping_packets::{Encodable, PingRespPacket},
    };

    #[test]
    fn create_test() {
        let ping_resp_packet = PingRespPacket::new();

        assert_eq!(
            ping_resp_packet.fixed_header.packet_type,
            ControlPacketType::PingResp
        );
        assert_eq!(
            ping_resp_packet.fixed_header.packet_flags,
            ControlPacketFlags::PING_RESP_FLAGS
        );
        assert_eq!(ping_resp_packet.fixed_header.remaining_length, 0);
    }

    #[test]
    fn encode_test() {
        let ping_resp_packet_bytes = PingRespPacket::new().encode();

        println!("{:?}", ping_resp_packet_bytes);
        assert_eq!(ping_resp_packet_bytes.len(), 2);
        assert_eq!(ping_resp_packet_bytes[0], 0b1101_0000);
        assert_eq!(ping_resp_packet_bytes[1], 0x00);
    }
}
