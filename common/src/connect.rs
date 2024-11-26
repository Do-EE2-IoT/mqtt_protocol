use crate::package::decode::Decode;
use crate::package::encode::Encode;
use crate::package::types;
use crate::package::types::ControlPackets;
use std::net::TcpStream;

#[repr(u8)]
pub enum ConnackReturnCode {
    ConnectionAccept,
    UnacceptableProtocolVersion,
    IdentifierReject,
    ServerUnavailable,
    BadUsernameOrPassword,
    NotAuthorized,
    UnknownCode,
}
// MQTT ver 3.1.1
#[derive(Default)]
pub struct ConnectPacket {
    keep_alive: u16,
    client_id: String,
}

pub struct ConnackPacket;

impl ConnackReturnCode {
    fn from_u8(code: u8) -> Self {
        match code {
            0x00 => Self::ConnectionAccept,
            0x01 => Self::UnacceptableProtocolVersion,
            0x02 => Self::IdentifierReject,
            0x03 => Self::ServerUnavailable,
            0x04 => Self::BadUsernameOrPassword,
            0x05 => Self::NotAuthorized,
            _ => Self::UnknownCode,
        }
    }
}
impl ConnectPacket {
    pub fn new(keep_alive: u16, client_id: String) -> Self {
        Self {
            keep_alive,
            client_id,
        }
    }
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

impl Decode for ConnackPacket {
    fn decode(&self, packet: Vec<u8>) {
        if packet.len() != 4 {
            println!("Connack packet is invalid, need at least 4 byte");
            return;
        }

        // Fixed Header
        if packet[0] != ControlPackets::Connack as u8 {
            println!("Gói tin không phải CONNACK! Byte đầu tiên phải là 0x20.");
            return;
        }
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
