use crate::connect::{ConnectPacket, Disconnect};
use crate::package::types::ControlPackets;
use crate::pubsub::{PublishPacket, SubscribePacket, UnsubscribePacket};
pub trait Encode {
    fn encode(&self) -> Result<Vec<u8>, String>;
}

pub fn encode<Packet: Encode>(packet: Packet) -> Vec<u8> {
    packet.encode().unwrap()
}

impl Encode for ConnectPacket {
    fn encode(&self) -> Result<Vec<u8>, String> {
        let mut packet = Vec::new();

        if !self.client_id.is_empty() && self.client_id.len() < 24 {
            packet.push(ControlPackets::Connect as u8);
            let remaining_length = 10 + 2 + self.client_id.len();
            packet.push(remaining_length as u8);
            packet.extend_from_slice(&[0x00, 0x04, b'M', b'Q', b'T', b'T', 0x04, 0x02]);
            packet
                .extend_from_slice(&[(self.keep_alive >> 8) as u8, (self.keep_alive & 0xFF) as u8]);
            packet.extend_from_slice(&[
                (self.client_id.len() >> 8) as u8,
                (self.client_id.len() & 0xFF) as u8,
            ]);
            packet.extend_from_slice(self.client_id.as_bytes());
            Ok(packet)
        } else {
            Err("Can't send connect packet to mqtt broker".to_string())
        }
    }
}

impl<'a> Encode for PublishPacket<'a> {
    fn encode(&self) -> Result<Vec<u8>, String> {
        let mut packet = vec![
            (ControlPackets::Publish as u8)
                | (self.last4bits_fix_header.dup_flag << 3)
                | (self.last4bits_fix_header.qos_level << 1)
                | self.last4bits_fix_header.retain,
        ];
        let remain_length = 2 + self.topic_name.len() + 2 + self.payload.len();
        packet.push(remain_length as u8);
        packet.extend_from_slice(&[
            (self.topic_name.len() >> 8) as u8,
            (self.topic_name.len() & 0xFF) as u8,
        ]);
        packet.extend_from_slice(self.topic_name.as_bytes());
        packet.extend_from_slice(&[(self.packet_id >> 8) as u8, (self.packet_id & 0xFF) as u8]);
        packet.extend_from_slice(self.payload.as_bytes());

        Ok(packet)
    }
}

impl<'a> SubscribePacket<'a> {
    pub fn new(packet_id: u16, topic_filter: &'a str, requested_qos: u8) -> Self {
        Self {
            packet_id,
            topic_filter,
            requested_qos,
        }
    }
}

impl<'a> Encode for SubscribePacket<'a> {
    fn encode(&self) -> Result<Vec<u8>, String> {
        let mut packet: Vec<u8> = vec![ControlPackets::Subscribe as u8];
        let remain_length = 2 + 2 + self.topic_filter.len() + 1;
        packet.push(remain_length as u8);
        packet.extend_from_slice(&[(self.packet_id >> 8) as u8, (self.packet_id & 0xFF) as u8]);
        packet.extend_from_slice(&[
            (self.topic_filter.len() >> 8) as u8,
            (self.topic_filter.len() & 0xFF) as u8,
        ]);
        packet.extend_from_slice(self.topic_filter.as_bytes());
        packet.push(self.requested_qos);
        Ok(packet)
    }
}

impl<'a> Encode for UnsubscribePacket<'a> {
    fn encode(&self) -> Result<Vec<u8>, String> {
        let mut packet: Vec<u8> = vec![ControlPackets::Unsubscribe as u8];
        let remaining_length = 2 + 2 + self.topic.len();
        packet.push(remaining_length as u8);
        packet.extend_from_slice(&[(self.packet_id >> 8) as u8, (self.packet_id & 0xFF) as u8]);
        packet.extend_from_slice(&[
            (self.topic.len() >> 8) as u8,
            (self.topic.len() & 0xFF) as u8,
        ]);
        packet.extend_from_slice(self.topic.as_bytes());
        Ok(packet)
    }
}

impl Encode for Disconnect {
    fn encode(&self) -> Result<Vec<u8>, String> {
        let mut packet = vec![ControlPackets::Disconnect as u8];
        packet.push(0x00);

        Ok(packet)
    }
}
