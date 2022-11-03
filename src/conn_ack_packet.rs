use crate::control_packets::{
    encode_remaining_length, ControlPacketFlags, ControlPacketType, FixedHeader,
};

#[derive(Debug)]
pub struct ConnAck {
    pub fixed_header: FixedHeader,
    pub connect_ack_flags: u8,
    pub connect_return_code: u8,
}

impl From<&[u8]> for ConnAck {
    fn from(bytes: &[u8]) -> Self {
        let fixed_header = FixedHeader::from(bytes);
        let remaining_length_byte_count = encode_remaining_length(fixed_header.remaining_length)
            .unwrap()
            .len();
        let connect_ack_flags_index = 1 + remaining_length_byte_count;
        let connect_return_code_index = connect_ack_flags_index + 1;
        Self {
            fixed_header: FixedHeader::from(bytes),
            connect_ack_flags: bytes[connect_ack_flags_index],
            connect_return_code: bytes[connect_return_code_index],
        }
    }
}

impl ConnAck {
    pub fn new() -> Self {
        Self {
            fixed_header: FixedHeader {
                packet_type: ControlPacketType::ConnAck,
                packet_flags: ControlPacketFlags::CONNACK_FLAGS,
                remaining_length: 0,
            },
            connect_ack_flags: 0,
            connect_return_code: 0,
        }
    }
}
