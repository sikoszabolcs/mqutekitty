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

use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum ControlPacketType {
    Unknown = 0,
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

impl From<u8> for ControlPacketType {
    fn from(value: u8) -> Self {
        match value {
            1 => ControlPacketType::Connect,
            2 => ControlPacketType::ConnAck,
            3 => ControlPacketType::Publish,
            4 => ControlPacketType::PubAck,
            5 => ControlPacketType::PubRec,
            6 => ControlPacketType::PubRel,
            7 => ControlPacketType::PubComp,
            8 => ControlPacketType::Subscribe,
            9 => ControlPacketType::SubAck,
            10 => ControlPacketType::Unsubscribe,
            11 => ControlPacketType::UnsubAck,
            12 => ControlPacketType::PingReq,
            13 => ControlPacketType::PingResp,
            14 => ControlPacketType::Disconnect,
            _ => ControlPacketType::Unknown,
        }
    }
}

impl Into<u8> for ControlPacketType {
    fn into(self) -> u8 {
        match self {
            ControlPacketType::Connect => 1,
            ControlPacketType::ConnAck => 2,
            ControlPacketType::Publish => 3,
            ControlPacketType::PubAck => 4,
            ControlPacketType::PubRec => 5,
            ControlPacketType::PubRel => 6,
            ControlPacketType::PubComp => 7,
            ControlPacketType::Subscribe => 8,
            ControlPacketType::SubAck => 9,
            ControlPacketType::Unsubscribe => 10,
            ControlPacketType::UnsubAck => 11,
            ControlPacketType::PingReq => 12,
            ControlPacketType::PingResp => 13,
            ControlPacketType::Disconnect => 14,
            ControlPacketType::Unknown => panic!(),
        }
    }
}

// TODO: Why can't I implement TryFrom on a type after implementing the From trait?

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

const MAX_REMAINING_LENGTH: u32 = 2_097_152; //128 * 128 * 128

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
mod remaining_length_conversion_tests {
    use crate::control_packets::{decode_remaining_length, encode_remaining_length};

    #[test]
    fn decode_remaining_length_test() {
        let lengt_bytes: [u8; 2] = [193, 2];
        let length = decode_remaining_length(&lengt_bytes);
        assert_eq!(length.unwrap(), 321);
    }

    #[test]
    fn decode_remaining_length_error_test() {
        let lengt_bytes: [u8; 5] = [193, 193, 193, 193, 193];
        let length = decode_remaining_length(&lengt_bytes);
        assert!(length.is_err());
    }

    #[test]
    fn encode_remaining_length_test() {
        let length = 321;
        let length_bytes = encode_remaining_length(length).unwrap();
        assert_eq!(length_bytes.len(), 2);
        assert_eq!(length_bytes[0], 193);
        assert_eq!(length_bytes[1], 2);
    }

    #[test]
    fn encode_remaining_length_max_test() {
        let length = 268435455;
        let length_bytes = encode_remaining_length(length).unwrap();
        assert_eq!(length_bytes.len(), 4);
        assert_eq!(length_bytes[0], 255);
        assert_eq!(length_bytes[1], 255);
        assert_eq!(length_bytes[2], 255);
        assert_eq!(length_bytes[3], 127);
    }

    #[test]
    fn encode_remaining_length_error_test() {
        let length = 268435456;
        let length_bytes = encode_remaining_length(length);
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
