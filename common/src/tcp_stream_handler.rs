use std::io::{self, Error, Read, Write};
use std::net::TcpStream;
pub struct ClientStreamHandler {
    pub socket: TcpStream,
}

impl ClientStreamHandler {
    pub fn connect(addr: &str) -> io::Result<ClientStreamHandler> {
        let socket = TcpStream::connect(addr).unwrap();
        Ok(ClientStreamHandler { socket })
    }

    pub fn send(&mut self, data: Vec<u8>) -> io::Result<()> {
        if let Err(e) = self.socket.write_all(&data) {
            println!("{e}");
        }
        Ok(())
    }

    pub fn read(&mut self) -> Result<Vec<u8>, String> {
        let mut buf: Vec<u8> = vec![0; 1024];
        let size = if let Ok(data) = self.socket.read(&mut buf) {
            data
        } else {
            return Err("Can't read data from broker".to_string());
        };
        buf.truncate(size);
        Ok(buf.to_vec())
    }
}
