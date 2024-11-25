use crate::package::{decode::Decode, encode::Encode, types::ControlPackets};

pub struct Last4BitsFixHeader {
    dup_flag: u8,
    qos_level: u8,
    retain: u8,
}

pub enum PublishPacketResponse {
    PubBack,
    PubRec,
    PubRel,
    PubComp,
}

pub struct PublishPacket<'a> {
    last4bits_fix_header: Last4BitsFixHeader,
    topic_name: &'a str,
    packet_id: u16,
    payload: &'a str,
}

impl<'a> PublishPacket<'a> {
    pub fn new(
        topic_name: &'a str,
        payload: &'a str,
        packet_id: u16,
        dup_flag: u8,
        qos_level: u8,
        retain: u8,
    ) -> Self {
        Self {
            last4bits_fix_header: Last4BitsFixHeader {
                dup_flag,
                qos_level,
                retain,
            },
            topic_name,
            packet_id,
            payload,
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

impl Decode for PublishPacketResponse {
    fn decode(&self, packet: Vec<u8>) {
        todo!()
    }
}
