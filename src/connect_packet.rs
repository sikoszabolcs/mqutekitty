use core::time;
use std::{error::Error, fmt};

use crate::control_packets::{ControlPacketFlags, ControlPacketType, Encodable, FixedHeader};

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

#[derive(Clone, Copy)]
pub struct ProtocolName<'a> {
    pub length: u16,
    pub name: &'a str,
}

const MQTT_PROTOCOL_NAME: &'static str = "MQTT";

// 3.1.2.1. Protocol Level
//  ---------------------------------------------------------
// | bit    | Description    | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
//  ---------------------------------------------------------
// | byte 7 |     Level(4)   | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 |
//  ---------------------------------------------------------
#[derive(Clone, Copy)]
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

    pub fn new() -> Self {
        ConnectFlagsBuilder::default()
    }

    pub fn user_name(&mut self) -> &mut Self {
        self.byte_rep = self.byte_rep | ConnectFlagsBuilder::USER_NAME_MASK;
        self
    }

    pub fn password(&mut self) -> &mut Self {
        self.byte_rep = self.byte_rep | ConnectFlagsBuilder::PASSWORD_MASK;
        self
    }

    pub fn will_retain(&mut self) -> &mut Self {
        self.byte_rep = self.byte_rep | ConnectFlagsBuilder::WILL_RETAIN_MASK;
        self
    }

    pub fn will_qos(&mut self, qos: u8) -> &mut Self {
        assert!(qos < 3);
        self.byte_rep = self.byte_rep | ((qos << 3u8) & ConnectFlagsBuilder::WILL_QOS_MASK);
        self
    }

    pub fn will_flag(&mut self) -> &mut Self {
        self.byte_rep = self.byte_rep | ConnectFlagsBuilder::WILL_FLAG_MASK;
        self
    }

    pub fn clean_session(&mut self) -> &mut Self {
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
        let flags = ConnectFlagsBuilder::new().user_name().build();
        assert_eq!(flags, 0b1000_0000.into())
    }

    #[test]
    fn password_build_test() {
        let flags = ConnectFlagsBuilder::new().password().build();
        assert_eq!(flags, 0b0100_0000.into())
    }

    #[test]
    fn will_retain_build_test() {
        let flags = ConnectFlagsBuilder::new().will_retain().build();
        assert_eq!(flags, 0b0010_0000.into())
    }

    #[test]
    fn will_qos_2_build_test() {
        let flags = ConnectFlagsBuilder::new().will_qos(2).build();
        assert_eq!(flags, 0b0001_0000.into())
    }

    #[test]
    fn will_qos_1_build_test() {
        let flags = ConnectFlagsBuilder::new().will_qos(1).build();
        assert_eq!(flags, 0b0000_1000.into())
    }

    #[test]
    fn will_qos_0_build_test() {
        let flags = ConnectFlagsBuilder::new().will_qos(0).build();
        assert_eq!(flags, 0b0000_0000.into())
    }

    #[test]
    fn will_flag_build_test() {
        let flags = ConnectFlagsBuilder::new().will_flag().build();
        assert_eq!(flags, 0b0000_0100.into())
    }

    #[test]
    fn clean_session_build_test() {
        let flags = ConnectFlagsBuilder::new().clean_session().build();
        assert_eq!(flags, 0b0000_0010.into())
    }

    #[test]
    #[should_panic]
    fn will_qos_3_build_test() {
        let _flags = ConnectFlagsBuilder::new().will_qos(3).build();
    }

    #[test]
    fn build_test() {
        let flags = ConnectFlagsBuilder::new()
            .user_name()
            .password()
            .will_retain()
            .will_qos(2)
            .will_flag()
            .clean_session()
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

#[derive(Debug, Clone)]
pub struct ErrorBuildingConnectPacket;
impl fmt::Display for ErrorBuildingConnectPacket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error building connect packet")
    }
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum QoS {
    AtMostOnce = 0,
    AtLeastOnce = 1,
    ExactlyOnce = 2,
}

impl From<u8> for QoS {
    fn from(value: u8) -> Self {
        match value {
            0 => QoS::AtMostOnce,
            1 => QoS::AtLeastOnce,
            2 => QoS::ExactlyOnce,
            _ => QoS::AtMostOnce, // defaults to at most once on invalid value
        }
    }
}

impl From<QoS> for u8 {
    fn from(value: QoS) -> Self {
        match value {
            QoS::AtMostOnce => 0,
            QoS::AtLeastOnce => 1,
            QoS::ExactlyOnce => 2,
        }
    }
}

pub struct Builder {
    user_name: Option<String>,
    password: Option<String>,
    protocol_name: String,
    protocol_level: Option<ProtocolLevel>,
    will_topic: Option<String>,
    will_message: Option<String>,
    will_qos: Option<QoS>,
    keep_alive_interval: time::Duration,
    client_id: Option<String>,
    clean_session: bool,
    will_retain: bool,
}

const U16_SIZE_IN_BYTES: usize = 2;
const U8_SIZE_IN_BYTES: usize = 1;

impl Builder {
    pub fn new() -> Self {
        Builder {
            user_name: None,
            password: None,
            protocol_name: MQTT_PROTOCOL_NAME.to_string(),
            protocol_level: Some(ProtocolLevel::V3_1_1), // default to v3.1.1
            will_retain: false,
            will_topic: None,
            will_message: None,
            will_qos: Some(QoS::AtMostOnce), // default to 0 (at most once)
            keep_alive_interval: time::Duration::from_secs(60), // default to 60 seconds
            client_id: None,
            clean_session: false,
        }
    }

    pub fn user_name(&mut self, user_name: String) -> &mut Self {
        self.user_name = Some(user_name);
        self
    }

    pub fn password(&mut self, password: String) -> &mut Self {
        self.password = Some(password);
        self
    }

    pub fn protocol_level(&mut self, protocol: ProtocolLevel) -> &mut Self {
        self.protocol_level = Some(protocol);
        self
    }

    pub fn keep_alive_interval(&mut self, keep_alive: time::Duration) -> &mut Self {
        self.keep_alive_interval = keep_alive;
        self
    }

    pub fn client_id(&mut self, client_id: &str) -> &mut Self {
        self.client_id = Some(client_id.to_string());
        self
    }

    pub fn clean_session(&mut self) -> &mut Self {
        self.clean_session = true;
        self
    }

    pub fn will_retain(&mut self) -> &mut Self {
        self.will_retain = true;
        self
    }

    pub fn will_topic(&mut self, will_topic: &str) -> &mut Self {
        self.will_topic = Some(will_topic.to_string());
        self
    }

    pub fn will_message(&mut self, will_message: &str) -> &mut Self {
        self.will_message = Some(will_message.to_string());
        self
    }

    pub fn will_qos(&mut self, qos: u8) -> &mut Self {
        self.will_qos = Some(qos.into());
        self
    }

    pub fn calc_flags(&self) -> u8 {
        let mut flags = 0b0000_0000;
        flags = flags
            | if self.user_name.is_some() {
                ConnectFlagsBuilder::USER_NAME_MASK
            } else {
                0
            };
        flags = flags
            | if self.password.is_some() && self.user_name.is_some() {
                ConnectFlagsBuilder::PASSWORD_MASK
            } else {
                0
            };
        flags = flags
            | if self.will_retain == true {
                ConnectFlagsBuilder::WILL_RETAIN_MASK
            } else {
                0
            };

        let will_qos = match self.will_qos {
            Some(qos) => qos,
            None => QoS::AtLeastOnce,
        };
        flags = flags | ((will_qos as u8) << 3u8) & ConnectFlagsBuilder::WILL_QOS_MASK;

        flags = flags
            | if self.will_message.is_some() && self.will_topic.is_some() {
                ConnectFlagsBuilder::WILL_FLAG_MASK
            } else {
                0
            };
        flags = flags
            | if self.clean_session {
                ConnectFlagsBuilder::CLEAN_SESSION_MASK
            } else {
                0
            };
        flags
    }

    pub fn calc_remaining_length(&self) -> usize {
        let mut remaining_length = 0;
        // Variable header - protocol name: 2 bytes for the length prefix + string length
        remaining_length += U16_SIZE_IN_BYTES + &self.protocol_name.len();
        // Variable header - byte 7 - protocol level
        remaining_length += U8_SIZE_IN_BYTES;
        // Variable header - byte 8 - connect flags
        remaining_length += U8_SIZE_IN_BYTES;
        // Variable header - byte 9  - keep alive MSB
        // Variable header - byte 10 - keep alive LSB
        remaining_length += U16_SIZE_IN_BYTES;

        // Payload - client id - first entry
        remaining_length += match &self.client_id {
            Some(client_id) => U16_SIZE_IN_BYTES + client_id.len(),
            None => 0,
        };

        // Payload - will topic - second entry
        remaining_length += match &self.will_topic {
            Some(will_topic) => U16_SIZE_IN_BYTES + will_topic.len(),
            None => 0,
        };

        // Payload - will message - third entry
        remaining_length += match &self.will_message {
            Some(will_message) => U16_SIZE_IN_BYTES + will_message.len(),
            None => 0,
        };

        // Payload - user name - fourth entry
        remaining_length += match &self.user_name {
            Some(user_name) => U16_SIZE_IN_BYTES + user_name.len(),
            None => 0,
        };

        // Payload - password - fifth entry
        remaining_length += match &self.password {
            Some(password) => U16_SIZE_IN_BYTES + password.len(),
            None => 0,
        };
        remaining_length
    }

    pub fn build(&mut self) -> Result<ConnectPacket, Box<dyn Error>> {
        let remaining_length = self.calc_remaining_length();
        let flags = self.calc_flags();

        Ok(ConnectPacket {
            fixed_header: FixedHeader {
                packet_type: ControlPacketType::Connect,
                packet_flags: ControlPacketFlags::CONNECT_FLAGS,
                remaining_length,
            },
            protocol_name: self.protocol_name.clone(),
            protocol_level: self.protocol_level.take().unwrap(),
            connect_flags: flags.into(),
            keep_alive: self.keep_alive_interval.as_secs().try_into()?, // max keep alive seconds is 65535
            client_id: self.client_id.take().unwrap(), // todo: I'm not sure if this is a good idea
        })
    }
}

pub struct ConnectPacket {
    pub fixed_header: FixedHeader,
    pub protocol_name: String,
    pub protocol_level: ProtocolLevel,
    pub connect_flags: ConnectFlags,
    pub keep_alive: u16,
    pub client_id: String,
}

impl ConnectPacket {
    pub fn encode(&self) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();
        vec.extend_from_slice(&mut self.fixed_header.encode());

        // Variable Header
        vec.extend_from_slice(&(self.protocol_name.len() as u16).to_be_bytes());
        vec.extend_from_slice(&mut vec![b'M', b'Q', b'T', b'T']);
        vec.push(self.protocol_level as u8); // vh byte 7
        vec.push(self.connect_flags.into()); // vh byte 8 - connected flags
        vec.extend_from_slice(&self.keep_alive.to_be_bytes());
        vec.extend_from_slice(&(self.client_id.len() as u16).to_be_bytes());
        vec.extend_from_slice(&self.client_id.as_bytes());
        return vec;
    }
}

#[cfg(test)]
mod connect_packet_tests {
    use crate::{
        connect_packet::{self},
        control_packets::ControlPacketType,
    };

    #[test]
    fn create_test() {
        let connect_packet = connect_packet::Builder::new()
            .client_id("mqutekitty_client")
            .clean_session()
            .build()
            .unwrap();

        assert_eq!(
            connect_packet.fixed_header.packet_type,
            ControlPacketType::Connect
        );
        assert_eq!(connect_packet.fixed_header.remaining_length, 29);
        assert_eq!(connect_packet.protocol_name.len(), 4);
        assert_eq!(connect_packet.protocol_name, String::from("MQTT"));
        assert_eq!(connect_packet.connect_flags, 2u8.into());
        assert_eq!(connect_packet.keep_alive, 60);
        assert_eq!(connect_packet.client_id, "mqutekitty_client");
    }

    #[test]
    fn encode_test() {
        let connect_packet = connect_packet::Builder::new()
            .clean_session()
            .client_id("mqutekitty_client")
            .build()
            .unwrap();
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
        assert_eq!(connect_packet_bytes[11], 60); // keep alive LSB
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
