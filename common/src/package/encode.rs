pub trait Encode {
    fn encode(&self) -> Result<Vec<u8>, String>;
}

pub fn encode<Packet: Encode>(packet: Packet) -> Vec<u8> {
    packet.encode().unwrap()
}
