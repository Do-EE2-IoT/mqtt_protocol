pub enum SubackReturnCode {
    MaximumQos0,
    MaximumQos1,
    MaximumQos2,
    Failure,
    Unknown,
}

#[derive(PartialEq)]
pub enum QosLevel {
    Qos0,
    Qos1,
    Qos2,
}
pub struct Last4BitsFixHeader {
    pub dup_flag: u8,
    pub qos_level: u8,
    pub retain: u8,
}

pub struct PublishPacketGet;
pub struct SubscribePacket<'a> {
    pub packet_id: u16,
    pub topic_filter: &'a str,
    pub requested_qos: u8,
}

pub struct UnsubscribePacket<'a> {
    pub packet_id: u16,
    pub topic: &'a str,
}

pub struct UnsubackPacket;

pub struct SubackPacket;
pub struct PublishPacket<'a> {
    pub last4bits_fix_header: Last4BitsFixHeader,
    pub topic_name: &'a str,
    pub packet_id: u16,
    pub payload: &'a str,
}

pub struct PubackPacket;

pub struct PubrelPacket {
    pub packet_id: u16,
}
pub struct PubrecPacket;
pub struct PubcompPacket;
impl SubackReturnCode {
    pub fn from_u8(code: u8) -> Self {
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

impl<'a> UnsubscribePacket<'a> {
    pub fn new(packet_id: u16, topic: &'a str) -> Self {
        Self { packet_id, topic }
    }
}
