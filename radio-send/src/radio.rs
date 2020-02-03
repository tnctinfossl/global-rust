use super::packet::*;
use log::warn;
use serde::*;
use serde_derive::*;
use serde_json::*;
use std::io;
use std::net::{SocketAddr, UdpSocket};
use std::sync::mpsc::Receiver;
use std::thread;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    //socketの送信元
    pub bind_ip4: [u8; 4],
    pub bind_port: u16,
    //socketの送信先
    pub connect_ip4: [u8; 4],
    pub connect_port: u16,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            bind_ip4: [0, 0, 0, 0],
            bind_port: 0,
            connect_ip4: [224, 5, 23, 2],
            connect_port: 10030,
        }
    }
}

#[derive(Debug)]
pub struct Radio {
    receiver: Receiver<Packet>,
    socket: UdpSocket,
}

impl Radio {
    pub fn spawn(settings: Settings, recv: Receiver<Packet>) -> io::Result<()> {
        let bind_addr = SocketAddr::from((settings.bind_ip4, settings.bind_port));
        let socket = UdpSocket::bind(bind_addr)?;
        let connect_addr = SocketAddr::from((settings.connect_ip4, settings.connect_port));
        socket.connect(connect_addr)?;
        let radio = Radio {
            receiver: recv,
            socket: socket,
        };
        thread::spawn(move || loop {
            if let Err(e) = radio.send() {
                warn!("radio cannot send {:?}", e);
            }
        });
        Ok(())
    }

    fn send(&self) -> io::Result<()> {
        if let Ok(packet) = self.receiver.recv() {
            //文字列として起こす
            let bytes = serde_json::to_vec(&packet)?;
            self.socket.send(&bytes)?;
        }
        Ok(())
    }
}
