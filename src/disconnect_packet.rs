use crate::{control_packets::ControlPacketFlags, ControlPacketType};

pub struct DisconnectPacket {
    pub packet_type: ControlPacketType,
    pub packet_flags: u8,
    pub remaining_length: u32,
}

impl DisconnectPacket {
    pub fn new() -> DisconnectPacket {
        DisconnectPacket {
            packet_type: ControlPacketType::Disconnect,
            packet_flags: ControlPacketFlags::DISCONNECT_FLAGS,
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
mod disconnect_packet_tests {
    use crate::{
        control_packets::{ControlPacketFlags, ControlPacketType},
        disconnect_packet::DisconnectPacket,
    };

    #[test]
    fn create_test() {
        let disconnect_packet = DisconnectPacket::new();

        assert_eq!(disconnect_packet.packet_type, ControlPacketType::Disconnect);
        assert_eq!(
            disconnect_packet.packet_flags,
            ControlPacketFlags::DISCONNECT_FLAGS
        );
        assert_eq!(disconnect_packet.remaining_length, 0);
    }

    #[test]
    fn encode_test() {
        let disconnect_packet_bytes = DisconnectPacket::new().encode();

        println!("{:?}", disconnect_packet_bytes);
        assert_eq!(disconnect_packet_bytes.len(), 2);
        assert_eq!(disconnect_packet_bytes[0], 0b1110_0000);
        assert_eq!(disconnect_packet_bytes[1], 0x00);
    }
}
