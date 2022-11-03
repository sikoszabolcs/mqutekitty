use crate::control_packets::{encode_remaining_length, ControlPacketFlags, ControlPacketType};

// 3. MQTT Control Packets
// 3.1. CONNECT - Client requests a connection to a Server
// After a Network Connection is established by a Client to a Server,
// the first Packet sent from the Client to the Server MUST be a CONNECT Packet [MQTT-3.1.0-1].
// A Client can only send the CONNECT Packet once over a Network Connection.
// The Server MUST process a second CONNECT Packet sent from a Client as a protocol violation and disconnect the Client [MQTT-3.1.0-2].

// 3.1.2. Variable header
// 3.1.2.1. Protocol name
//  ---------------------------------------------------------
// | bit    | Description    | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
//  ---------------------------------------------------------
// | byte 1 | Length MSB (0) | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
// ----------------------------------------------------------
// | byte 2 | Length LSM (4) | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 |
// ----------------------------------------------------------
// | byte 3 |       'M'      | 0 | 1 | 0 | 0 | 1 | 1 | 0 | 1 |
// ----------------------------------------------------------
// | byte 4 |       'Q'      | 0 | 1 | 0 | 1 | 0 | 0 | 0 | 1 |
// ----------------------------------------------------------
// | byte 5 |       'T'      | 0 | 1 | 0 | 1 | 0 | 1 | 0 | 0 |
// ----------------------------------------------------------
// | byte 6 |       'T'      | 0 | 1 | 0 | 1 | 0 | 1 | 0 | 0 |
// ----------------------------------------------------------

pub struct ProtocolName<'a> {
    pub length: u16,
    pub name: &'a str,
}

impl<'a> ProtocolName<'a> {}

const MQTT: &str = "MQTT";

const MQTT_PROTOCOL_NAME: ProtocolName = ProtocolName {
    length: 4,
    name: MQTT,
};

// 3.1.2.1. Protocol Level
//  ---------------------------------------------------------
// | bit    | Description    | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
//  ---------------------------------------------------------
// | byte 7 |     Level(4)   | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 |
//  ---------------------------------------------------------
#[repr(u8)]
pub enum ProtocolLevel {
    V3_1 = 3,
    V3_1_1 = 4,
    V5 = 5,
}

// 3.1.2.3. Connect Flags
// The Connect Flags byte contains a number of parameters specifying the behavior of the MQTT connection.
// It also indicates the presence or absence of fields in the payload.
//
//  -------------------------------------------------------------------------------------
// | bit    |     7      |    6     |    5   |  4  |  3 |     2     |    1    |     0    |
//  -------------------------------------------------------------------------------------
// |        | User Name  | Password |  Will  | Will QoS | Will Flag |  Clean  | Reserved |
// |        |   Flag     |   Flag   | Retain |          |           | Session |          |
//  -------------------------------------------------------------------------------------
// | byte 8 |      X     |     X    |    X   |  X  | X  |      X    |    X    |     0    |
//  -------------------------------------------------------------------------------------
// The Server MUST validate that the reserved flag in the CONNECT Control Packet is set to zero and disconnect the Client if it is not zero [MQTT-3.1.2-3].

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct ConnectFlags {
    byte_rep: u8,
}

impl ConnectFlags {
    pub fn builder() -> ConnectFlagsBuilder {
        ConnectFlagsBuilder::default()
    }
}

impl From<u8> for ConnectFlags {
    fn from(byte: u8) -> Self {
        ConnectFlags { byte_rep: byte }
    }
}

impl Into<u8> for ConnectFlags {
    fn into(self) -> u8 {
        self.byte_rep
    }
}

#[derive(Default, PartialEq)]
pub struct ConnectFlagsBuilder {
    byte_rep: u8,
}

impl ConnectFlagsBuilder {
    const USER_NAME_MASK: u8 = 0b1000_0000;
    const PASSWORD_MASK: u8 = 0b0100_0000;
    const WILL_RETAIN_MASK: u8 = 0b0010_0000;
    const WILL_QOS_MASK: u8 = 0b0001_1000;
    const WILL_FLAG_MASK: u8 = 0b0000_0100;
    const CLEAN_SESSION_MASK: u8 = 0b0000_0010;

    pub fn new() -> ConnectFlagsBuilder {
        ConnectFlagsBuilder::default()
    }

    pub fn with_user_name(&mut self) -> &mut ConnectFlagsBuilder {
        self.byte_rep = self.byte_rep | ConnectFlagsBuilder::USER_NAME_MASK;
        self
    }

    pub fn with_password(&mut self) -> &mut ConnectFlagsBuilder {
        self.byte_rep = self.byte_rep | ConnectFlagsBuilder::PASSWORD_MASK;
        self
    }

    pub fn with_will_retain(&mut self) -> &mut ConnectFlagsBuilder {
        self.byte_rep = self.byte_rep | ConnectFlagsBuilder::WILL_RETAIN_MASK;
        self
    }

    pub fn with_will_qos(&mut self, qos: u8) -> &mut ConnectFlagsBuilder {
        assert!(qos < 3);
        self.byte_rep = self.byte_rep | ((qos << 3u8) & ConnectFlagsBuilder::WILL_QOS_MASK);
        self
    }

    pub fn with_will_flag(&mut self) -> &mut ConnectFlagsBuilder {
        self.byte_rep = self.byte_rep | ConnectFlagsBuilder::WILL_FLAG_MASK;
        self
    }

    pub fn with_clean_session(&mut self) -> &mut ConnectFlagsBuilder {
        self.byte_rep = self.byte_rep | ConnectFlagsBuilder::CLEAN_SESSION_MASK;
        self
    }

    pub fn build(&mut self) -> ConnectFlags {
        ConnectFlags {
            byte_rep: self.byte_rep,
        }
    }
}

#[cfg(test)]
mod connect_flags_tests {

    use super::ConnectFlagsBuilder;

    #[test]
    fn user_name_build_test() {
        let flags = ConnectFlagsBuilder::new().with_user_name().build();
        assert_eq!(flags, 0b1000_0000.into())
    }

    #[test]
    fn password_build_test() {
        let flags = ConnectFlagsBuilder::new().with_password().build();
        assert_eq!(flags, 0b0100_0000.into())
    }

    #[test]
    fn will_retain_build_test() {
        let flags = ConnectFlagsBuilder::new().with_will_retain().build();
        assert_eq!(flags, 0b0010_0000.into())
    }

    #[test]
    fn will_qos_2_build_test() {
        let flags = ConnectFlagsBuilder::new().with_will_qos(2).build();
        assert_eq!(flags, 0b0001_0000.into())
    }

    #[test]
    fn will_qos_1_build_test() {
        let flags = ConnectFlagsBuilder::new().with_will_qos(1).build();
        assert_eq!(flags, 0b0000_1000.into())
    }

    #[test]
    fn will_qos_0_build_test() {
        let flags = ConnectFlagsBuilder::new().with_will_qos(0).build();
        assert_eq!(flags, 0b0000_0000.into())
    }

    #[test]
    fn will_flag_build_test() {
        let flags = ConnectFlagsBuilder::new().with_will_flag().build();
        assert_eq!(flags, 0b0000_0100.into())
    }

    #[test]
    fn clean_session_build_test() {
        let flags = ConnectFlagsBuilder::new().with_clean_session().build();
        assert_eq!(flags, 0b0000_0010.into())
    }

    #[test]
    #[should_panic]
    fn will_qos_3_build_test() {
        let _flags = ConnectFlagsBuilder::new().with_will_qos(3).build();
    }

    #[test]
    fn build_test() {
        let flags = ConnectFlagsBuilder::new()
            .with_user_name()
            .with_password()
            .with_will_retain()
            .with_will_qos(2)
            .with_will_flag()
            .with_clean_session()
            .build();

        assert_eq!(flags, 0b1111_0110.into());
    }
}

// 3.1.2.10. Keep Alive
//  ----------------------------------------
// | bit    | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
//  ----------------------------------------
// | byte 9 |        Keep Alive MSB         |
//  ----------------------------------------
// | byte 8 |        Keep Alive LSB         |
//  ----------------------------------------
// The Keep Alive is a time interval measured in seconds.
// Expressed as a 16-bit word, it is the maximum time interval that is permitted
// to elapse between the point at which the Client finishes transmitting one Control Packet
// and the point it starts sending the next.
// It is the responsibility of the Client to ensure that the interval between Control Packets
// being sent does not exceed the Keep Alive value.
// In the absence of sending any other Control Packets, the Client MUST send a PINGREQ Packet [MQTT-3.1.2-23].

// 3.1.3. Payload
// The payload of the CONNECT Packet contains one or more length-prefixed fields,
// whose presence is determined by the flags in the variable header.
// These fields, if present, MUST appear in the order
// Client Identifier, Will Topic, Will Message, User Name, Password [MQTT-3.1.3-1].
pub struct ConnectPacket<'a> {
    pub packet_type: ControlPacketType, // 4 bits + 4 bits reserved
    pub packet_flags: u8,
    pub remaining_length: u32, // up to 4 bytes
    pub protocol_name: ProtocolName<'a>,
    pub protocol_level: ProtocolLevel,
    pub connect_flags: ConnectFlags,
    pub keep_alive: u16,
    pub client_id: String,
}

impl<'a> ConnectPacket<'a> {
    pub fn new(connect_flags: u8) -> Self {
        Self {
            packet_type: ControlPacketType::Connect,
            packet_flags: ControlPacketFlags::CONNECT_FLAGS,
            remaining_length: 0,
            protocol_name: MQTT_PROTOCOL_NAME,
            protocol_level: ProtocolLevel::V3_1_1,
            connect_flags: connect_flags.into(),
            keep_alive: 10, // for 10 seconds
            // payload
            client_id: String::from("mqutekitty_client"),
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();

        // Fixed Header
        let packet_type_repr: u8 = self.packet_type as u8;
        let fixed_header_byte1: u8 = packet_type_repr << 4u8 | self.packet_flags & 0b00001111;
        vec.push(fixed_header_byte1);
        let mut remaining_length = 0;

        // Variable Header
        vec.push(0); // variable header byte 1: Length MSB (0)
        remaining_length += 1;
        vec.push(4); // variable header byte 2: Length LSB (4)
        remaining_length += 1;
        vec.push(b'M');
        remaining_length += 1;
        vec.push(b'Q');
        remaining_length += 1;
        vec.push(b'T');
        remaining_length += 1;
        vec.push(b'T');
        remaining_length += 1;
        vec.push(self.protocol_level as u8); // vh byte 7
        remaining_length += 1;
        vec.push(self.connect_flags.into()); // vh byte 8 - connected flags
        remaining_length += 1;
        for keep_alive_part in self.keep_alive.to_be_bytes() {
            vec.push(keep_alive_part);
            remaining_length += 1;
        }
        let client_id_size = self.client_id.len() as u16;
        for client_id_size_part in client_id_size.to_be_bytes() {
            vec.push(client_id_size_part);
            remaining_length += 1;
        }
        for client_id_part in self.client_id.as_bytes() {
            vec.push(client_id_part.clone());
            remaining_length += 1;
        }

        let encode_result = encode_remaining_length(remaining_length);
        let encoded_remaining_length = match encode_result {
            Ok(length) => length,
            Err(error) => panic!("Error encoding ConnectPacket: {}", error),
        };

        let mut index = 1;
        for remaining_length_part in encoded_remaining_length {
            vec.insert(index, remaining_length_part);
            index += 1;
        }

        return vec;
    }
}

#[cfg(test)]
mod connect_packet_tests {
    use crate::{connect_packet::ConnectPacket, control_packets::ControlPacketType};

    #[test]
    fn create_test() {
        let connect_packet = ConnectPacket::new(2u8);

        assert_eq!(connect_packet.packet_type, ControlPacketType::Connect);
        assert_eq!(connect_packet.remaining_length, 0);
        assert_eq!(connect_packet.protocol_name.length, 4);
        assert_eq!(connect_packet.protocol_name.name, String::from("MQTT"));
        assert_eq!(connect_packet.connect_flags, 2u8.into());
        assert_eq!(connect_packet.keep_alive, 10);
        assert_eq!(connect_packet.client_id, "mqutekitty_client");
    }

    #[test]
    fn encode_test() {
        let connect_packet = ConnectPacket::new(2u8);

        let connect_packet_bytes: Vec<u8> = connect_packet.encode();

        println!("{:?}", connect_packet_bytes);
        assert_eq!(connect_packet_bytes.len(), 31);
        assert_eq!(connect_packet_bytes[0], 0x10);
        assert_eq!(connect_packet_bytes[1], 0x1d); // TODO: check if the lengt calculation is correct!
        assert_eq!(connect_packet_bytes[2], 0x00); // protocol name length MSB
        assert_eq!(connect_packet_bytes[3], 0x04); // protocol name length LSB
        assert_eq!(connect_packet_bytes[4], 0x4d); // 'M'
        assert_eq!(connect_packet_bytes[5], 0x51); // 'Q'
        assert_eq!(connect_packet_bytes[6], 0x54); // 'T'
        assert_eq!(connect_packet_bytes[7], 0x54); // 'T'
        assert_eq!(connect_packet_bytes[8], 0x04); // packet level
        assert_eq!(connect_packet_bytes[9], 0x02); // connect flags
        assert_eq!(connect_packet_bytes[10], 0); // keep alive MSB
        assert_eq!(connect_packet_bytes[11], 10); // keep alive LSB
        assert_eq!(connect_packet_bytes[12], 0); // client_id length MSB
        assert_eq!(connect_packet_bytes[13], 17); // client_id length LSB
        assert_eq!(connect_packet_bytes[14], 109); // 'm'
        assert_eq!(connect_packet_bytes[15], 113); // 'q'
        assert_eq!(connect_packet_bytes[16], 117); // 'u'
        assert_eq!(connect_packet_bytes[17], 116); // 't'
        assert_eq!(connect_packet_bytes[18], 101); // 'e'
        assert_eq!(connect_packet_bytes[19], 107); // 'k'
        assert_eq!(connect_packet_bytes[20], 105); // 'i'
        assert_eq!(connect_packet_bytes[21], 116); // 't'
        assert_eq!(connect_packet_bytes[22], 116); // 't'
        assert_eq!(connect_packet_bytes[23], 121); // 'y'
        assert_eq!(connect_packet_bytes[24], 95); // '_'
        assert_eq!(connect_packet_bytes[25], 99); // 'c'
        assert_eq!(connect_packet_bytes[26], 108); // 'l'
        assert_eq!(connect_packet_bytes[27], 105); // 'i'
        assert_eq!(connect_packet_bytes[28], 101); // 'e'
        assert_eq!(connect_packet_bytes[29], 110); // 'n'
        assert_eq!(connect_packet_bytes[30], 116); // 't'
    }
}
