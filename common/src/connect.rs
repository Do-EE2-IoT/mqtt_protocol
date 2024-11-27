use crate::mqtt::decode::Decode;
use crate::mqtt::encode::Encode;
use crate::mqtt::types;
use crate::mqtt::types::ControlPackets;
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
    pub keep_alive: u16,
    pub client_id: String,
}

pub struct DisconnectPacket;

pub struct ConnackPacket;

impl ConnackReturnCode {
    pub fn from_u8(code: u8) -> Self {
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
