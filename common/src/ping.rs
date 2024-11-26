use crate::package::{decode::Decode, encode::Encode, types::ControlPackets};

pub struct PingPacket;

impl Encode for PingPacket {
    fn encode(&self) -> Result<Vec<u8>, String> {
        let mut packet: Vec<u8> = vec![ControlPackets::Pingreq as u8];
        let remain_length: u8 = 0;
        packet.push(remain_length);
        Ok(packet)
    }
}

pub struct PingResPacket;

impl Decode for PingResPacket {
    fn decode(&self, packet: Vec<u8>) {
        if packet.len() > 2 {
            println!("Error: Pingres is 2 bytes");
            return;
        }

        if packet[0] != ControlPackets::Pingresp as u8 {
            println!("This is not Pingres packet ");
            return;
        }

        println!("Get Pingres from broker, stable connection!");
    }
}
