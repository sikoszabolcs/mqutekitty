use std::{fmt, net::TcpStream, io::Write};

// 2. MQTT Control Packet format
// 2.1. Structure of an MQTT Control Packet
//  ------------------------------------------------
// |   Fixed header (in all MQTT control packets)   |
//  ------------------------------------------------
// | Variable header (in some MQTT control packets) |
//  ------------------------------------------------
// |     Payload (in some MQTT control packets)     |
//  ------------------------------------------------

// 2.2. Fixed header
//  ----------------------------------------------------------------------------------------------------
// | bit    |    7    |    6    |    5    |    4    |     3      |     2      |     1      |      0     |
//  ----------------------------------------------------------------------------------------------------
// | byte 1 |  MQTT control packet type             |  Flags specific to each MQTT control packet type  |
// -----------------------------------------------------------------------------------------------------
// | byte 2 |                               Remaining length                                            |
// -----------------------------------------------------------------------------------------------------

// 2.2.1. MQTT Control Packet type
#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum ControlPacketType {
    Connect = 1,
    ConnAck = 2,
    Publish = 3,
    PubAck = 4,
    PubRec = 5,
    PubRel = 6,
    PubComp = 7,
    Subscribe = 8,
    SubAck = 9,
    Unsubscribe = 10,
    UnsubAck = 11,
    PingReq = 12,
    PingResp = 13,
    Disconnect = 14,
}

// 2.2.2. Flags
pub struct ControlPacketFlags {}

impl ControlPacketFlags {
    pub const CONNECT_FLAGS: u8 = 0;
    pub const CONNACK_FLAGS: u8 = 0;
    // PUBLISH_FLAGS are not constant, they change with config
    //  ---------------------------------
    // | Bit 3 | Bit 2 | Bit 1 |  Bit 0  |
    //  ---------------------------------
    // |  DUP  |  QoS  |  QoS  | Retain  |
    //  ---------------------------------
    // DUP - Duplicate delivery of a PUBLISH Control Packet
    // QoS - PUBLISH Quality of Service
    pub const PUB_ACK_FLAGS: u8 = 0;
    pub const PUB_REC_FLAGS: u8 = 0;
    pub const PUB_REL_FLAGS: u8 = 2;
    pub const PUB_COMP_FLAGS: u8 = 0;
    pub const SUBSCRIBE_FLAGS: u8 = 2;
    pub const SUB_ACK_FLAGS: u8 = 0;
    pub const UNSUBSCRIBE_FLAGS: u8 = 2;
    pub const UNSUB_ACK_FLAGS: u8 = 0;
    pub const PING_REQ_FLAGS: u8 = 0;
    pub const PING_RESP_FLAGS: u8 = 0;
    pub const DISCONNECT_FLAGS: u8 = 0;
}

// 2.2.3. Remaining Length
// The Remaining Length is the number of bytes remaining within the current packet,
// including data in the variable header and the payload.
// The Remaining Length does not include the bytes used to encode the Remaining Length.
//
// The Remaining Length is encoded using a variable length encoding scheme
// which uses a single byte for values up to 127.
// Larger values are handled as follows.
// The least significant seven bits of each byte encode the data,
// and the most significant bit is used to indicate that there are following bytes
// in the representation.
// Thus each byte encodes 128 values and a "continuation bit".
// The maximum number of bytes in the Remaining Length field is four.

#[derive(Debug, Clone)]
pub struct EncodeError;

impl fmt::Display for EncodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error encoding value")
    }
}

pub fn encode_remaining_length(mut length: u32) -> Result<Vec<u8>, EncodeError> {
    let mut vec: Vec<u8> = Vec::new();

    loop {
        let mut encoded_byte: u8 = (length % 128).try_into().unwrap();
        length = length / 128;
        if length > 0 {
            encoded_byte = encoded_byte | 128;
        }
        vec.push(encoded_byte);
        if length <= 0 {
            if vec.len() > 4 {
                return Err(EncodeError);
            }
            return Ok(vec);
        }
    }
}

const MAX_REMAINING_LENGTH: u32 = 128 * 128 * 128;

#[derive(Debug, Clone)]
pub struct DecodeError;

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error decoding value")
    }
}

pub fn decode_remaining_length(encoded: &[u8]) -> Result<u32, DecodeError> {
    let mut multiplier: u32 = 1;
    let mut value: u32 = 0;
    let mut index: usize = 0;
    loop {
        value = value + (encoded[index] & 127) as u32 * multiplier;
        multiplier *= 128;
        if multiplier > MAX_REMAINING_LENGTH {
            return Err(DecodeError);
        }
        if (encoded[index] & 128) == 0 {
            return Ok(value);
        }
        index += 1;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn decode_remaining_length_test() {
        let lengt_bytes: [u8; 2] = [193, 2];
        let length = super::decode_remaining_length(&lengt_bytes);
        assert_eq!(length.unwrap(), 321);
    }

    #[test]
    fn decode_remaining_length_error_test() {
        let lengt_bytes: [u8; 5] = [193, 193, 193, 193, 193];
        let length = super::decode_remaining_length(&lengt_bytes);
        assert!(length.is_err());
    }

    #[test]
    fn encode_remaining_length_test() {
        let length = 321;
        let length_bytes = super::encode_remaining_length(length).unwrap();
        assert_eq!(length_bytes.len(), 2);
        assert_eq!(length_bytes[0], 193);
        assert_eq!(length_bytes[1], 2);
    }

    #[test]
    fn encode_remaining_length_max_test() {
        let length = 268435455;
        let length_bytes = super::encode_remaining_length(length).unwrap();
        assert_eq!(length_bytes.len(), 4);
        assert_eq!(length_bytes[0], 255);
        assert_eq!(length_bytes[1], 255);
        assert_eq!(length_bytes[2], 255);
        assert_eq!(length_bytes[3], 127);
    }

    #[test]
    fn encode_remaining_length_error_test() {
        let length = 268435456;
        let length_bytes = super::encode_remaining_length(length);
        assert!(length_bytes.is_err());
    }
}

// 2.3. Variable header
// 2.3.1 Packet identifier
//  ----------------------------------------------------------------------------------------
// | bit    |    7    |    6    |    5    |    4    |    3    |    2    |    1    |    0    |
//  ----------------------------------------------------------------------------------------
// | byte 1 |                             Packet identifier MSB                             |
// -----------------------------------------------------------------------------------------
// | byte 2 |                             Packet identifier LSB                             |
// -----------------------------------------------------------------------------------------
//
// The variable header component of many of the Control Packet types includes a 2 byte Packet Identifier field.
// These Control Packets are PUBLISH (where QoS > 0), PUBACK, PUBREC, PUBREL, PUBCOMP, SUBSCRIBE, SUBACK, UNSUBSCRIBE, UNSUBACK.
//
// The Client and Server assign Packet Identifiers independently of each other.
// As a result, Client Server pairs can participate in concurrent message exchanges using the same Packet Identifiers.
//
// 2.4. Payload
// Some MQTT Control Packets contain a payload as the final part of the packet.
//  ---------------------------
// | Control Packet | Payload  |
//  ---------------------------
// | CONNECT        | Required |
// | PUBLISH        | Optional |
// | SUBSCRIBE      | Required |
// | SUBACK         | Required |
// | UNSUBSCRIBE    | Required |
//  ---------------------------

// pub struct MqttPacket{
//     packet_type:ControlPacketType,
//     packet_flags:u8,
//     remaining_length:u32,
//     packet_id:Option<u16>,
//     payload:String
// }

// impl MqttPacket{
//     pub fn new() -> Self {
//         Self {
//             packet_type: ControlPacketType::Connect,
//             packet_flags: ControlPacketFlags::CONNECT_FLAGS,
//             remaining_length: 0,
//             packet_id: None,
//             payload:String::from("")
//         }
//     }

//     pub fn from_bytes(packet_bytes: &[u8]) -> MqttPacket{
//         MqttPacket::new()
//     }
// }

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
// | byte 2 | Length LSM (4) | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 |                                                              |
// ----------------------------------------------------------
// | byte 3 |       'M'      | 0 | 1 | 0 | 0 | 1 | 1 | 0 | 1 |                                                              |
// ----------------------------------------------------------
// | byte 4 |       'Q'      | 0 | 1 | 0 | 1 | 0 | 0 | 0 | 1 |                                                              |
// ----------------------------------------------------------
// | byte 5 |       'T'      | 0 | 1 | 0 | 1 | 0 | 1 | 0 | 0 |                                                              |
// ----------------------------------------------------------
// | byte 6 |       'T'      | 0 | 1 | 0 | 1 | 0 | 1 | 0 | 0 |                                                              |
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

#[derive(PartialEq, Debug)]
pub struct ConnectFlags {
    pub raw_byte: u8,
}

impl ConnectFlags {
    pub fn from_byte(byte: u8) -> ConnectFlags {
        return ConnectFlags { raw_byte: byte };
    }

    // pub fn user_name_flag(&self) -> bool{
    //     return self.raw_byte[8] == 1;
    // }
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
    pub fn new() -> Self {
        Self {
            packet_type: ControlPacketType::Connect,
            packet_flags: ControlPacketFlags::CONNECT_FLAGS,
            remaining_length: 0,
            protocol_name: MQTT_PROTOCOL_NAME,
            protocol_level: ProtocolLevel::V3_1_1,
            connect_flags: ConnectFlags::from_byte(2), // clean session bit to 1, the rest to 0
            keep_alive: 10,                            // for 10 seconds
            // payload
            client_id: String::from("mqutekitty_client"),
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();

        // Fixed Header
        let packet_type_repr: u8 = self.packet_type as u8;
        let fh_byte1: u8 = packet_type_repr << 4u8 | self.packet_flags & 0b00001111;
        vec.push(fh_byte1); // fh_
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
        vec.push(self.connect_flags.raw_byte); // vh byte 8 - connected flags
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

    pub fn decode(_packet_bytes: &[u8]) -> ConnectPacket {
        return ConnectPacket::new();
    }
}

#[cfg(test)]
mod connect_packet_tests {
    use crate::{ConnectFlags, ConnectPacket, ControlPacketType};

    #[test]
    fn create_test() {
        let connect_packet = ConnectPacket::new();

        assert_eq!(connect_packet.packet_type, ControlPacketType::Connect);
        assert_eq!(connect_packet.remaining_length, 0);
        assert_eq!(connect_packet.protocol_name.length, 4);
        assert_eq!(connect_packet.protocol_name.name, String::from("MQTT"));
        assert_eq!(connect_packet.connect_flags, ConnectFlags::from_byte(2));
        assert_eq!(connect_packet.keep_alive, 10);
        assert_eq!(connect_packet.client_id, "mqutekitty_client");
    }

    #[test]
    fn encode_test() {
        let connect_packet = ConnectPacket::new();

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

fn main() -> Result<(), std::io::Error> {

    let connect_packet = ConnectPacket::new();
    let _connect_packet_bytes: Vec<u8> = connect_packet.encode();

    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:1883"){
        println!("Connected to the server!");

        match stream.write(&_connect_packet_bytes){
            Ok(_) => println!("Write CONNECT packet - OK"),
            Err(ref error) => {
                println!("Error writing to TcpSteam {}", error);
            }
        }
    }
    else{
        println!("Couldn't connect to the server...");
    }

    return Ok(());
}
