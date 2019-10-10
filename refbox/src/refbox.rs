use super::referee::SSL_Referee;
use super::updater::Updater;
use log::warn;
use serde_derive::{Deserialize, Serialize};
use std::net::*;
use std::sync::{Arc, RwLock};
use std::thread;
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Settings {
    ip4: [u8; 4],
    port: u16,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            ip4: [224, 5, 23, 1],
            port: 10003,
        }
    }
}

pub struct RefBox {}

impl RefBox {
    pub fn spawn(settings: &Settings, world: Arc<RwLock<model::World>>) -> Result<RefBox, String> {
        //multicastを受け付ける
        let addr = Ipv4Addr::from(settings.ip4);
        let addr_port = (addr, settings.port);
        let socket =
            UdpSocket::bind(addr_port).map_err(|e| format!("refbox cannnot bind;{:?}", e))?;
        socket
            .join_multicast_v4(&addr, &Ipv4Addr::from([0, 0, 0, 0]))
            .map_err(|e| format!("refbox cannnot join multicast;{:?}", e))?;
        let updater = Updater::new();
        thread::spawn(move || {
            let mut buffer = [0; 1024];
            loop {
                let size = match socket.recv(&mut buffer) {
                    Ok(size) => size,
                    Err(e) => {
                        warn!("refbox failure receiving;{:?}", e);
                        continue;
                    }
                };
                let referee: SSL_Referee = match protobuf::parse_from_bytes(&buffer[..size]) {
                    Ok(referee) => referee,
                    Err(e) => {
                        warn!("refbox failure parsing;{:?}", e);
                        continue;
                    }
                };
                //println!("{:?}", packet);
                match world.write() {
                    Ok(mut w) => updater.update(&mut w, &referee),
                    Err(e) => {
                        warn!("refbox failure taking mutex;{:?}", e);
                        continue;
                    }
                }
            }
        });
        Ok(RefBox {})
    }
}
