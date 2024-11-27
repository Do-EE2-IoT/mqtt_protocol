use crate::connect::{ConnackPacket, ConnackReturnCode};
use crate::package::types::ControlPackets;
use crate::pubsub::{
    PubackPacket, PubcompPacket, PublishPacketGet, PubrecPacket, SubackPacket, SubackReturnCode,
    UnsubackPacket,
};

pub trait Decode {
    fn decode(&self, packet: Vec<u8>);
}

pub fn decode<Packet: Decode>(packet: Packet, buffer: Vec<u8>) {
    packet.decode(buffer);
}

impl Decode for ConnackPacket {
    fn decode(&self, packet: Vec<u8>) {
        if packet.len() != 4 {
            println!("Connack packet is invalid, need at least 4 byte");
            return;
        }

        // Fixed Header

        println!("Fixed Header: 0x{:02x} (CONNACK)", packet[0]);

        // Remaining Length
        if packet[1] != 0x02 {
            println!("Remaining Length is invalid, it must be 2 bytes");
            return;
        }
        println!("Remaining Length: {}", packet[1]);

        // Variable Header
        let session_present = packet[2] & 0x01; // Lấy bit 0 của byte thứ 3
        println!("Session Present Flag: {}", session_present);

        // Return Code
        let return_code = packet[3];
        match ConnackReturnCode::from_u8(return_code) {
            ConnackReturnCode::ConnectionAccept => {
                println!("Return Code: 0x00 (Connection Accepted)")
            }
            ConnackReturnCode::UnacceptableProtocolVersion => {
                println!("Return Code: 0x01 (Unacceptable Protocol Version)")
            }
            ConnackReturnCode::IdentifierReject => {
                println!("Return Code: 0x02 (Identifier Rejected)")
            }
            ConnackReturnCode::ServerUnavailable => {
                println!("Return Code: 0x03 (Server Unavailable)")
            }
            ConnackReturnCode::BadUsernameOrPassword => {
                println!("Return Code: 0x04 (Bad User Name or Password)")
            }
            ConnackReturnCode::NotAuthorized => println!("Return Code: 0x05 (Not Authorized)"),
            ConnackReturnCode::UnknownCode => {
                println!("Return Code: 0x{:02x} (Unknown)", return_code)
            }
        }
    }
}

impl Decode for PublishPacketGet {
    fn decode(&self, packet: Vec<u8>) {
        if packet.len() < 4 {
            println!("Packet is too short to be a valid PUBLISH packet");
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

        let payload_start = topic_end + 2;
        let payload = String::from_utf8(packet[payload_start..].to_vec()).unwrap();
        println!("Topic = {}", topic);
        println!("Message = {}", payload);
    }
}

impl Decode for SubackPacket {
    fn decode(&self, packet: Vec<u8>) {
        if packet.len() < 5 {
            println!("Invalid Suback packet, it must be at least 5 bytes");
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

impl Decode for UnsubackPacket {
    fn decode(&self, packet: Vec<u8>) {
        if packet.len() > 4 {
            println!("Invalid Unsuback packet, it must be 4 bytes");
            return;
        }
        println!("Get Packet ID = 0x{:02X}{:02X}", packet[2], packet[3]);
        println!("Get Unsuback successfully");
    }
}

impl Decode for PubackPacket {
    fn decode(&self, packet: Vec<u8>) {
        if packet.len() != 4 {
            println!("Invalid Pubrec packet, it must be 4 bytes");
            return;
        }

        println!("Get Pubrec Packet ID 0x{:02X}{:02X}", packet[2], packet[3]);
    }
}
impl Decode for PubrecPacket {
    fn decode(&self, packet: Vec<u8>) {
        if packet.len() != 4 {
            println!("Invalid Pubrec packet, it must be 4 bytes");
            return;
        }

        println!("Get Pubrec Packet ID 0x{:02X}{:02X}", packet[2], packet[3]);
    }
}

impl Decode for PubcompPacket {
    fn decode(&self, packet: Vec<u8>) {
        if packet.len() != 4 {
            println!("Invalid Pubcomp packet, it must be 4 bytes");
            return;
        }

        println!("Get Pubrec Packet ID 0x{:02X}{:02X}", packet[2], packet[3]);
    }
}
