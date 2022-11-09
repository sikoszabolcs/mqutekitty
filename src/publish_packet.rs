use crate::{
    connect_packet::QoS,
    control_packets::{ControlPacketFlags, Encodable, FixedHeader},
    ControlPacketType,
};

pub(crate) struct PublishPacketFlags {
    // duplicate_delivery: Option<bool>,
    // qos: Option<QoS>,
    // retain: Option<bool>
    byte_rep: u8,
}

impl From<PublishPacketFlags> for u8 {
    fn from(value: PublishPacketFlags) -> Self {
        // let dup_mask = 0b000_1000;
        // let qos_mask= 0b0000_0110;
        // let retain_mask = 0b0000_0001;
        value.byte_rep
    }
}

impl From<u8> for PublishPacketFlags {
    fn from(value: u8) -> Self {
        PublishPacketFlags { byte_rep: value }
    }
}

pub(crate) struct PublishPacket {
    pub fixed_header: FixedHeader,
    pub topic_name: String,
    // A PUBLISH Packet MUST NOT contain a Packet Identifier if its QoS value is set to 0 [MQTT-2.3.1-5].
    pub packet_id: Option<u16>,
    pub payload: Vec<u8>,
}

impl PublishPacket {
    pub fn new(flags: PublishPacketFlags, topic_name: &str, payload: &[u8]) -> Self {
        PublishPacket {
            fixed_header: FixedHeader {
                packet_flags: flags.into(),
                packet_type: ControlPacketType::Publish,
                remaining_length: 0,
            },
            topic_name: topic_name.to_string(),
            packet_id: None,
            payload: payload.to_vec(),
        }
    }

    fn calc_remaining_length(&self) -> usize {
        let mut remaining_length = 0;
        remaining_length += 2 + self.topic_name.len();
        remaining_length += match &self.packet_id {
            Some(_) => 2,
            None => 0,
        };
        remaining_length += self.payload.len();
        remaining_length
    }
}

impl Encodable for PublishPacket {
    fn encode(&self) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();
        let header = FixedHeader{
            packet_flags: self.fixed_header.packet_flags,
            packet_type: self.fixed_header.packet_type,
            remaining_length: self.calc_remaining_length()
        };
        vec.extend_from_slice(&mut header.encode());
        vec.extend_from_slice(&(self.topic_name.len() as u16).to_be_bytes());
        vec.extend_from_slice(&self.topic_name.as_bytes());
        match self.packet_id {
            Some(id) => vec.extend_from_slice(&id.to_be_bytes()),
            None => {}
        }
        vec.extend_from_slice(&self.payload);
        vec
    }
}
