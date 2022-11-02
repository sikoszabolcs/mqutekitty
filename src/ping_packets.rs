use crate::{control_packets::ControlPacketFlags, ControlPacketType};

pub struct PingReqPacket {
    pub packet_type: ControlPacketType,
    pub packet_flags: u8,
    pub remaining_length: u32,
}

impl PingReqPacket {
    pub fn new() -> PingReqPacket {
        PingReqPacket {
            packet_type: ControlPacketType::PingReq,
            packet_flags: ControlPacketFlags::PING_REQ_FLAGS,
            remaining_length: 0,
        }
    }

    pub fn encode(self) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();
        // Fixed Header
        let packet_type_repr: u8 = self.packet_type.into();
        let fixed_header_byte1: u8 = packet_type_repr << 4u8 | self.packet_flags & 0b00001111;
        vec.push(fixed_header_byte1);
        let remaining_length = 0;
        vec.push(remaining_length);
        vec
    }
}

#[derive(Debug)]
pub struct PingRespPacket {
    pub packet_type: ControlPacketType,
    pub packet_flags: u8,
    pub remaining_length: u32,
}

impl From<&[u8]> for PingRespPacket {
    fn from(bytes: &[u8]) -> Self {
        PingRespPacket {
            packet_type: ControlPacketType::from((bytes[0] >> 4) & 0x0f),
            packet_flags: bytes[0] & 0xf0,
            remaining_length: bytes[1].into(),
        }
    }
}

impl PingRespPacket {
    pub fn new() -> PingRespPacket {
        PingRespPacket {
            packet_type: ControlPacketType::PingResp,
            packet_flags: ControlPacketFlags::PING_RESP_FLAGS,
            remaining_length: 0,
        }
    }

    pub fn encode(self) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();
        // Fixed Header
        let packet_type_repr: u8 = self.packet_type.into();
        let fixed_header_byte1: u8 = packet_type_repr << 4u8 | self.packet_flags & 0b00001111;
        vec.push(fixed_header_byte1);
        let remaining_length = 0;
        vec.push(remaining_length);
        vec
    }
}

#[cfg(test)]
mod ping_req_packet_tests {
    use crate::{
        control_packets::{ControlPacketFlags, ControlPacketType},
        ping_packets::PingReqPacket,
    };

    #[test]
    fn create_test() {
        let ping_req_packet = PingReqPacket::new();

        assert_eq!(ping_req_packet.packet_type, ControlPacketType::PingReq);
        assert_eq!(
            ping_req_packet.packet_flags,
            ControlPacketFlags::PING_REQ_FLAGS
        );
        assert_eq!(ping_req_packet.remaining_length, 0);
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
        ping_packets::PingRespPacket,
    };

    #[test]
    fn create_test() {
        let ping_resp_packet = PingRespPacket::new();

        assert_eq!(ping_resp_packet.packet_type, ControlPacketType::PingResp);
        assert_eq!(
            ping_resp_packet.packet_flags,
            ControlPacketFlags::PING_RESP_FLAGS
        );
        assert_eq!(ping_resp_packet.remaining_length, 0);
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
