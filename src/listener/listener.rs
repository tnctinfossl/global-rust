use super::messages::World;
use super::messages_robocup_ssl_wrapper::SSL_WrapperPacket;
use log::{debug, error, info, warn};
use serde_derive::{Deserialize, Serialize};
use std::io;
use std::net::{Ipv4Addr, SocketAddr, UdpSocket};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Settings {
    pub vision_ip4: [u8; 4], //ip address  of cam or sim
    pub vision_port: u16,    //
    pub command_port: u16,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            vision_ip4: [224, 5, 23, 2],
            vision_port: 10020,
            command_port: 20011,
        }
    }
}

pub struct Listener {
    vision_handler: thread::JoinHandle<()>,
    world_receiver: Receiver<Box<World>>,
}

impl Listener {
    pub fn new(settings: &Settings) -> Listener {
        let (world_sender, world_receiver) = channel();

        let vision_handler = {
            let settings = settings.clone();
            thread::spawn(move || {
                let addr = {
                    let [a, b, c, d] = settings.vision_ip4;
                    Ipv4Addr::new(a, b, c, d)
                };
                let socket = UdpSocket::bind(&SocketAddr::from((addr, settings.vision_port)))
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
                let mut buffer = [0u8; 1024];
                loop {
                    let size = match socket.recv(&mut buffer) {
                        Ok(s) => s,
                        Err(e) => {
                            warn!("Receive from vision server;{:?}", e);
                            continue;
                        }
                    };
                    let packet: SSL_WrapperPacket = protobuf::parse_from_bytes(&buffer[..size]).unwrap();
                    if let Some(w)=World::from_message(&packet){
                        world_sender.send(Box::new(w)).unwrap();
                    }
                    
                }
            })
        };
        Listener {
            world_receiver: world_receiver,
            vision_handler: vision_handler,
        }
    }

    pub fn get_world(&self) -> &Receiver<Box<World>> {
        &self.world_receiver
    }


}

