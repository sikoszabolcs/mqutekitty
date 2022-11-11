use std::{error::Error, vec};

use crate::{
    connect_packet::QoS,
    control_packets::{ControlPacketFlags, ControlPacketType, Encodable, FixedHeader},
};

pub struct TopicFilter<'a> {
    pub topic_name: &'a str,
    pub requested_qos: QoS,
}

pub struct Builder<'a> {
    fixed_header: FixedHeader,
    packet_id: u16, // must be a non-zero value
    topic_filters: Vec<TopicFilter<'a>>,
}

impl<'a> Builder<'a> {
    pub fn new() -> Self {
        Builder {
            fixed_header: FixedHeader {
                packet_type: ControlPacketType::Subscribe,
                packet_flags: ControlPacketFlags::SUBSCRIBE_FLAGS,
                remaining_length: 0,
            },
            packet_id: 1,
            topic_filters: vec![],
        }
    }

    pub fn packet_id(&mut self, packet_id: u16) -> &mut Self {
        self.packet_id = packet_id;
        self
    }

    pub fn topic_filter(&mut self, topic_filter: TopicFilter<'a>) -> &mut Self {
        self.topic_filters.push(topic_filter);
        self
    }

    pub fn calc_remaining_length(&self) -> usize {
        let mut remaining_length = 0;
        remaining_length += 2;
        for topic_filter in self.topic_filters.iter() {
            remaining_length += 2 /* length bytes */ + topic_filter.topic_name.len() + 1 /* requested qos */;
        }
        remaining_length
    }

    pub fn build(&mut self) -> Result<SubscribePacket, Box<dyn Error>> {
        self.fixed_header.remaining_length = self.calc_remaining_length();
        Ok(SubscribePacket {
            fixed_header: self.fixed_header,
            packet_id: self.packet_id,
            topic_filters: &self.topic_filters,
        })
    }
}
pub struct SubscribePacket<'a> {
    fixed_header: FixedHeader,
    packet_id: u16, // must be a non-zero value
    topic_filters: &'a Vec<TopicFilter<'a>>,
}

impl<'a> Encodable for SubscribePacket<'a> {
    fn encode(&self) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();
        vec.extend_from_slice(&mut self.fixed_header.encode());
        vec.extend_from_slice(&self.packet_id.to_be_bytes());

        for topic_filter in self.topic_filters.iter() {
            vec.extend_from_slice(&(topic_filter.topic_name.len() as u16).to_be_bytes());
            vec.extend_from_slice(&topic_filter.topic_name.as_bytes());
            vec.push(topic_filter.requested_qos.into());
        }
        vec
    }
}

#[cfg(test)]
mod subscribe_packet_tests {
    use crate::control_packets::Encodable;

    #[test]
    fn encode_test() {
        let topic_name = String::from("a/b");
        let topic_filter = super::TopicFilter {
            topic_name: topic_name.as_str(),
            requested_qos: crate::connect_packet::QoS::AtMostOnce,
        };
        let mut builder = super::Builder::new();
        let subscribe_packet = builder.topic_filter(topic_filter).build();
        let subscribe_packet_bytes = subscribe_packet.unwrap().encode();

        println!("{:?}", subscribe_packet_bytes);

        assert_eq!(subscribe_packet_bytes[0], 0b1000_0010); // control packet type + reserved
        assert_eq!(subscribe_packet_bytes[1], 8); // remaining length
        assert_eq!(subscribe_packet_bytes[2], 0); // packet id MSB
        assert_eq!(subscribe_packet_bytes[3], 0); // packet id LSB
        assert_eq!(subscribe_packet_bytes[4], 0); // topic filter length MSB
        assert_eq!(subscribe_packet_bytes[5], 3); // topic filter length LSB
        assert_eq!(subscribe_packet_bytes[6], 0x61); // 'a'
        assert_eq!(subscribe_packet_bytes[7], 0x2f); // '/'
        assert_eq!(subscribe_packet_bytes[8], 0x62); // 'b'
        assert_eq!(subscribe_packet_bytes[9], 0); // topic filter requested QoS
    }
}
