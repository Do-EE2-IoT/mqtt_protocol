use std::convert::TryFrom;

#[derive(Debug)]
pub enum ControlPackets {
    Connect = 0b0001_0000,
    Connack = 0b0010_0000,
    Publish = 0b0011_0000,
    Puback = 0b0100_0000,
    Pubrec = 0b0101_0000,
    Pubrel = 0b0110_0010,
    Pubcomp = 0b0111_0000,
    Subscribe = 0b1000_0010,
    Suback = 0b1001_0000,
    Unsubscribe = 0b1010_0010,
    Unsuback = 0b1011_0000,
    Pingreq = 0b1100_0000,
    Pingresp = 0b1101_0000,
    Disconnect = 0b1110_0000,
    Auth = 0b1111_0000,
}

// convert a datatype to enum datatype
impl TryFrom<u8> for ControlPackets {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b0001_0000 => Ok(ControlPackets::Connect),
            0b0010_0000 => Ok(ControlPackets::Connack),
            0b0011_0000 => Ok(ControlPackets::Publish),
            0b0100_0000 => Ok(ControlPackets::Puback),
            0b0101_0000 => Ok(ControlPackets::Pubrec),
            0b0110_0010 => Ok(ControlPackets::Pubrel),
            0b0111_0000 => Ok(ControlPackets::Pubcomp),
            0b1000_0010 => Ok(ControlPackets::Subscribe),
            0b1001_0000 => Ok(ControlPackets::Suback),
            0b1010_0010 => Ok(ControlPackets::Unsubscribe),
            0b1011_0000 => Ok(ControlPackets::Unsuback),
            0b1100_0000 => Ok(ControlPackets::Pingreq),
            0b1101_0000 => Ok(ControlPackets::Pingresp),
            0b1110_0000 => Ok(ControlPackets::Disconnect),
            0b1111_0000 => Ok(ControlPackets::Auth),
            _ => Err("Unknown Control Packet"),
        }
    }
}