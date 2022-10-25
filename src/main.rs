
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
//  ----------------------------------------------------------------------------------------
// | bit    |    7    |    6    |    5    |    4    |    3    |    2    |    1    |    0    |
//  ----------------------------------------------------------------------------------------
// | byte 1 |  MQTT control packet type |  Flags specific to each MQTT control packet type  |
// -----------------------------------------------------------------------------------------
// | byte 2 |                               Remaining length                                |
// -----------------------------------------------------------------------------------------

// 2.2.1. MQTT Control Packet type
pub enum ControlPacket{
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
    Disconnect = 14
}

pub struct ControlPacketFlags{
    
}

// 2.2.2. Flags
impl ControlPacketFlags {
    pub const CONNECT_FLAGS:u8 = 0;
    pub const CONNACK_FLAGS:u8 = 0;
    // PUBLISH_FLAGS are not constant, they change with config
    //  ---------------------------------
    // | Bit 3 | Bit 2 | Bit 1 |  Bit 0  |
    //  ---------------------------------
    // |  DUP  |  QoS  |  QoS  | Retain  |
    //  ---------------------------------
    // DUP - Duplicate delivery of a PUBLISH Control Packet
    // QoS - PUBLISH Quality of Service
    pub const PUB_ACK_FLAGS:u8 = 0;
    pub const PUB_REC_FLAGS:u8 = 0;
    pub const PUB_REL_FLAGS:u8 = 2;
    pub const PUB_COMP_FLAGS:u8 = 0;
    pub const SUBSCRIBE_FLAGS:u8 = 2;
    pub const SUB_ACK_FLAGS:u8 = 0;
    pub const UNSUBSCRIBE_FLAGS:u8 = 2;
    pub const UNSUB_ACK_FLAGS:u8 = 0;
    pub const PING_REQ_FLAGS:u8 = 0;
    pub const PING_RESP_FLAGS:u8 = 0;
    pub const DISCONNECT_FLAGS:u8 = 0;
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
pub fn encode_remaining_length(mut length:u32) -> Vec<u8>{
    let mut vec:Vec<u8> = Vec::new();

    loop {
        let mut encoded_byte:u8 = (length % 128).try_into().unwrap();
        length = length / 128;
        if length > 0 {
            encoded_byte = encoded_byte | 128;
        }
        vec.push(encoded_byte);
        if length <= 0 {
            return vec;
        }
    }
}

const MAX_REMAINING_LENGTH : u32 = 128 * 128 * 128;

pub fn decode_remaining_length(encoded:&[u8]) -> u32{
    let mut multiplier: u32 = 1;
    let mut value: u32 = 0;
    let mut index : usize = 0;
    loop{
        value = value + (encoded[index] & 127) as u32 * multiplier;
        multiplier *= 128;
        if multiplier > MAX_REMAINING_LENGTH
        {
            // error
        }
        if (encoded[index] & 128) == 0 {
            return value;
        }
        index += 1;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn decode_remaining_length_test() {
        let lengt_bytes:[u8; 2] =[193, 2];
        let length = super::decode_remaining_length(&lengt_bytes);
        assert_eq!(length, 321);
    }

    #[test]
    fn encode_remaining_length_test() {
        let length = 321;
        let length_bytes = super::encode_remaining_length(length);
        assert_eq!(length_bytes.len(), 2);
        assert_eq!(length_bytes[0], 193);
        assert_eq!(length_bytes[1], 2);
    }
}


fn main() {

    //Ok();
}