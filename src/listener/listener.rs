use super::messages::World;
use super::messages_robocup_ssl_wrapper::SSL_WrapperPacket;
use serde_derive::{Deserialize, Serialize};
use std::io;
use std::net::{Ipv4Addr, SocketAddr, UdpSocket};
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
#[derive(Serialize, Deserialize, Debug)]
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
    kill: Arc<bool>,
    handle: thread::JoinHandle<()>,
}

impl Listener {
    pub fn run(settings: &Settings, sender: Sender<World>) -> io::Result<Listener> {
        //socket
        let addr = {
            let [a, b, c, d] = settings.vision_ip4;
            Ipv4Addr::new(a, b, c, d)
        };
        let socket = UdpSocket::bind(&SocketAddr::from((addr, settings.vision_port)))?;
        socket.join_multicast_v4(&addr, &Ipv4Addr::new(0, 0, 0, 0))?;
        //signal
        let kill = Arc::new(false);
        //buffer
        let handle = {
            let kill = kill.clone();
            thread::spawn(move || {
                let mut buffer = [0; 1024];
                while !*kill {
                    let size = socket.recv(&mut buffer).unwrap();
                    let packet: SSL_WrapperPacket =
                        protobuf::parse_from_bytes(&buffer[..size]).unwrap();
                    if let Some(w) = World::from_message(&packet) {
                        sender.send(w).unwrap();
                    }
                }
            })
        };
        Ok(Listener {
            kill: kill,
            handle: handle,
        })
    }

    /*
        pub fn recv(&mut self)->io::Result<Option<World>>{
            //受信
            let mut buffer=[0;1024];
            let size=self.socket.recv(&mut buffer)?;
            //解読
            let packet:SSL_WrapperPacket=protobuf::parse_from_bytes(&buffer[..size])?;
            Ok(World::from_message(&packet))
        }
    */
}
