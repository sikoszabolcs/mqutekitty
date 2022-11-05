use crate::{
    control_packets::{ControlPacketFlags, Encodable, FixedHeader},
    ControlPacketType,
};

pub(crate) struct DisconnectPacket {
    pub fixed_header: FixedHeader,
}

impl DisconnectPacket {
    pub fn new() -> DisconnectPacket {
        DisconnectPacket {
            fixed_header: FixedHeader {
                packet_type: ControlPacketType::Disconnect,
                packet_flags: ControlPacketFlags::DISCONNECT_FLAGS,
                remaining_length: 0,
            },
        }
    }
}

impl Encodable for DisconnectPacket {
    fn encode(&self) -> Vec<u8> {
        self.fixed_header.encode()
    }
}

#[cfg(test)]
mod disconnect_packet_tests {
    use crate::{
        control_packets::{ControlPacketFlags, ControlPacketType, Encodable},
        disconnect_packet::DisconnectPacket,
    };

    #[test]
    fn create_test() {
        let disconnect_packet = DisconnectPacket::new();

        assert_eq!(
            disconnect_packet.fixed_header.packet_type,
            ControlPacketType::Disconnect
        );
        assert_eq!(
            disconnect_packet.fixed_header.packet_flags,
            ControlPacketFlags::DISCONNECT_FLAGS
        );
        assert_eq!(disconnect_packet.fixed_header.remaining_length, 0);
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
