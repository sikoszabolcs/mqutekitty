use std::error::Error;

use crate::{
    control_packets::{as_u16_be, encode_remaining_length, Encodable, FixedHeader},
    ControlPacketType,
};

#[derive(Default, Clone, Copy)]
pub struct PublishPacketFlags {
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

pub struct Builder<'a> {
    packet_flags: PublishPacketFlags,
    packet_id: Option<u16>,
    topic_name: Option<&'a str>,
    payload: Option<&'a [u8]>,
}

impl<'a> Builder<'a> {
    pub fn new() -> Self {
        Builder {
            packet_flags: PublishPacketFlags::default(),
            packet_id: None,
            topic_name: None,
            payload: None,
        }
    }

    pub fn packet_flags(&mut self, packet_flags: PublishPacketFlags) -> &mut Self {
        self.packet_flags = packet_flags;
        self
    }

    pub fn packet_id(&mut self, packet_id: u16) -> &mut Self {
        self.packet_id = Some(packet_id);
        self
    }

    pub fn topic_name(&mut self, topic_name: &'a str) -> &mut Self {
        self.topic_name = Some(topic_name);
        self
    }

    pub fn payload(&mut self, payload: &'a [u8]) -> &mut Self {
        self.payload = Some(payload);
        self
    }

    pub fn calc_remaining_length(&self) -> usize {
        let mut remaining_length = 0;
        remaining_length += match &self.topic_name {
            Some(topic) => 2 + topic.len(),
            None => 0,
        };

        remaining_length += match &self.packet_id {
            Some(_) => 2,
            None => 0,
        };

        remaining_length += match &self.payload {
            Some(payload) => payload.len(),
            None => 0,
        };

        remaining_length
    }

    pub fn build(&mut self) -> Result<PublishPacket, Box<dyn Error>> {
        let remaining_length = self.calc_remaining_length();
        Ok(PublishPacket {
            fixed_header: FixedHeader {
                packet_flags: self.packet_flags.into(),
                packet_type: ControlPacketType::Publish,
                remaining_length,
            },
            topic_name: self.topic_name.unwrap(), // TODO: threat this more gracefully
            packet_id: self.packet_id,
            payload: self.payload.unwrap(),
        })
    }
}

#[derive(Debug)]
pub struct PublishPacket<'a> {
    pub fixed_header: FixedHeader,
    pub topic_name: &'a str,
    // A PUBLISH Packet MUST NOT contain a Packet Identifier if its QoS value is set to 0 [MQTT-2.3.1-5].
    pub packet_id: Option<u16>,
    pub payload: &'a [u8],
}

impl<'a> PublishPacket<'a> {
    pub fn new(flags: PublishPacketFlags, topic_name: &'a str, payload: &'a [u8]) -> Self {
        PublishPacket {
            fixed_header: FixedHeader {
                packet_flags: flags.into(),
                packet_type: ControlPacketType::Publish,
                remaining_length: 0,
            },
            topic_name: topic_name,
            packet_id: None,
            payload: payload,
        }
    }
}

impl<'a> Encodable for PublishPacket<'a> {
    fn encode(&self) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();
        vec.extend_from_slice(&mut self.fixed_header.encode());
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

impl<'a> From<&'a [u8]> for PublishPacket<'a> {
    fn from(bytes: &'a [u8]) -> Self {
        let fixed_header = FixedHeader::from(bytes);
        let remaining_length_byte_count = encode_remaining_length(fixed_header.remaining_length)
            .unwrap()
            .len();

        // TODO: No packet id at QoS 0
        let mut cursor = 1 + remaining_length_byte_count;
        let topic_length_si = cursor;
        let topic_length_ei = topic_length_si + 2;
        let topic_length = as_u16_be(&bytes[topic_length_si..topic_length_ei]);
        cursor += 2;
        let topic_ei = cursor + topic_length as usize;
        let topic = std::str::from_utf8(&bytes[cursor..topic_ei]);
        cursor = topic_ei;
        let payload_ei =
            cursor + fixed_header.remaining_length - (cursor - 1 - remaining_length_byte_count);
        let payload = &bytes[cursor..payload_ei];

        // | control packet type byte | remaining_length bytes | topic_length bytes | topic bytes | payload bytes |
        //                                                                                        ^               ^
        //                                                                                      cursor        payload_ei
        //                                                     |                 remaining length                 |
        // remaining_length - (cursor - 1 - rem_len_bytes)

        Self {
            fixed_header: FixedHeader::from(bytes),
            packet_id: None,
            topic_name: topic.unwrap(),
            payload: payload,
        }
    }
}

#[cfg(test)]
mod publish_packet_tests {
    use crate::control_packets::ControlPacketType;

    use super::PublishPacket;

    #[test]
    fn test() {
        let publish_packet_bytes: Vec<u8> = vec![
            0b0011_0000,
            9,
            0,
            3,
            0x61,
            0x2F,
            0x62,
            0x74,
            0x65,
            0x73,
            0x74,
        ];

        let publish_packet = PublishPacket::from(publish_packet_bytes.as_slice());

        assert_eq!(
            publish_packet.fixed_header.packet_type,
            ControlPacketType::Publish
        );
    }
}
