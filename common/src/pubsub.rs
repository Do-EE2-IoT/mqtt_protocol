use crate::package::{decode::Decode, encode::Encode, types::ControlPackets};

pub enum SubackReturnCode {
    MaximumQos0,
    MaximumQos1,
    MaximumQos2,
    Failure,
    Unknown,
}
pub struct Last4BitsFixHeader {
    dup_flag: u8,
    qos_level: u8,
    retain: u8,
}

pub struct PublishPacketResponse;
pub struct SubscribePacket<'a> {
    packet_id: u16,
    topic_filter: &'a str,
    requested_qos: u8,
}

pub struct SubscriberPacketResponse;
pub struct PublishPacket<'a> {
    last4bits_fix_header: Last4BitsFixHeader,
    topic_name: &'a str,
    packet_id: u16,
    payload: &'a str,
}

impl SubackReturnCode {
    fn from_u8(code: u8) -> Self {
        match code {
            0x00 => Self::MaximumQos0,
            0x01 => Self::MaximumQos1,
            0x02 => Self::MaximumQos2,
            0x80 => Self::Failure,
            _ => Self::Unknown,
        }
    }
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
        if packet.len() < 4 {
            println!("Packet is too short to be a valid PUBLISH packet");
            return;
        }
        if packet[0] != ControlPackets::Publish as u8 {
            println!("Not a PUBLISH packet");
            return;
        }
        // packet[1] remaining length

        let topic_length = ((packet[2] as usize) << 8) | (packet[3] as usize);
        let topic_start = 4;
        let topic_end = topic_start + topic_length;

        if packet.len() < topic_end {
            println!("Packet is too short to contain the full topic");
        }

        let topic = String::from_utf8(packet[topic_start..topic_end].to_vec()).unwrap();

        let payload_start = topic_end;
        let payload = String::from_utf8(packet[payload_start..].to_vec()).unwrap();
        println!("Topic = {}", topic);
        println!("Message = {}", payload);
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

impl Decode for SubscriberPacketResponse {
    fn decode(&self, packet: Vec<u8>) {
        if packet.len() < 5 {
            println!("Invalid Suback packet, it must be at least 5 bytes");
            return;
        }
        if packet[0] != ControlPackets::Suback as u8 {
            println!(" this is not suback packet, Error");
            return;
        }

        println!("Get Suback packet, {}", packet[1]);

        if packet[1] != 0x03 {
            println!("Remaining Length is invalid, it must be 3 bytes");
            return;
        }

        println!("Get Packet ID = 0x{:02X}{:02X}", packet[2], packet[3]);

        match SubackReturnCode::from_u8(packet[4]) {
            SubackReturnCode::MaximumQos0 => println!("Success sub with qos 0"),
            SubackReturnCode::MaximumQos1 => println!("Success sub with qos 1"),
            SubackReturnCode::MaximumQos2 => println!("Success sub with qos 2"),
            SubackReturnCode::Failure => println!("Sub topic failure"),
            SubackReturnCode::Unknown => println!("Unknown code ?"),
        }
    }
}
