#[repr(u8)]
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
