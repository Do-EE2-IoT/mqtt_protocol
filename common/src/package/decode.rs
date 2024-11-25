pub trait Decode {
    fn decode(&self, packet: Vec<u8>);
}

pub fn decode<Packet: Decode>(packet: Packet, buffer: Vec<u8>) {
    packet.decode(buffer);
}
