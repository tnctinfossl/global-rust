use super::updater::Updater;
use log::{error, warn};
use model::World;
use serde_derive::{Deserialize, Serialize};
use std::net::{Ipv4Addr, SocketAddr, UdpSocket};
use std::sync::{Arc, RwLock};
use std::thread;
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Settings {
    pub ip4: [u8; 4], //ip address  of cam or sim
    pub port: u16,    //
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            ip4: [224, 5, 23, 2],
            port: 10020,
        }
    }
}

pub struct Listener {}

impl Listener {
    pub fn spawn(settings: &Settings, world: Arc<RwLock<World>>) -> Listener {
        let settings = settings.clone();
        thread::spawn(move || {
            let addr = {
                let [a, b, c, d] = settings.ip4;
                Ipv4Addr::new(a, b, c, d)
            };
            let socket = UdpSocket::bind(&SocketAddr::from((addr, settings.port)))
                .unwrap_or_else(|e| {
                    let message = format!("Cannot bind vision server:{:?}", e);
                    error!("{}", message);
                    panic!(message);
                });
            socket
                .join_multicast_v4(&addr, &Ipv4Addr::new(0, 0, 0, 0))
                .unwrap_or_else(|e| {
                    let message = format!("Cannot join vision server:{:?}", e);
                    error!("{}", message);
                    panic!(message);
                });
            let mut buffer = [0u8; 4096];
            let updater = Updater::new(100.0, std::time::Duration::from_secs_f32(3.0));
            loop {
                let size = match socket.recv(&mut buffer) {
                    Ok(s) => s,
                    Err(e) => {
                        warn!("Receive from vision server;{:?}", e);
                        continue;
                    }
                };
                let packet = match protobuf::parse_from_bytes(&buffer[..size]) {
                    Ok(s) => s,
                    Err(e) => {
                        warn!("Parse from vision server;size={},{:?}", size, e);
                        continue;
                    }
                };

                if let Ok(mut w) = world.write() {
                    updater.update(&mut w, &packet);
                }
            }
        });

        Listener {}
    }
}
