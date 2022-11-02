use crate::control_packets::{
    decode_remaining_length, encode_remaining_length, ControlPacketFlags, ControlPacketType,
};

#[derive(Debug)]
pub struct ConnAck {
    pub packet_type: ControlPacketType, // 4 bits + 4 bits reserved
    pub packet_flags: u8,
    pub remaining_length: u32, // up to 4 bytes,
    pub connect_ack_flags: u8,
    pub connect_return_code: u8,
}

impl ConnAck {
    pub fn new() -> Self {
        Self {
            packet_type: ControlPacketType::ConnAck,
            packet_flags: ControlPacketFlags::CONNACK_FLAGS,
            remaining_length: 0,
            connect_ack_flags: 0,
            connect_return_code: 0,
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let remaining_length = decode_remaining_length(&bytes[1..]).unwrap();
        let remaining_length_byte_count = encode_remaining_length(remaining_length).unwrap().len();
        let connect_ack_flags_index = 1 + remaining_length_byte_count;
        let connect_return_code_index = connect_ack_flags_index + 1;
        Self {
            packet_type: ControlPacketType::ConnAck,
            packet_flags: ControlPacketFlags::CONNACK_FLAGS,
            remaining_length,
            connect_ack_flags: bytes[connect_ack_flags_index],
            connect_return_code: bytes[connect_return_code_index],
        }
    }
}
