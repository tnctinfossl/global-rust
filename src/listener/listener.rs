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
    killer: Arc<AtomicBool>,
    vision_handler: thread::JoinHandle<()>,
    vision_receiver: Receiver<Box<World>>,
}

impl Listener {
    pub fn new(settings: &Settings) -> Listener {
        let killer = Arc::new(AtomicBool::new(false));
        let (vision_sender, vision_receiver) = channel();

        let vision_handler = {
            let settings = settings.clone();
            let killer = killer.clone();
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
                while !killer.load(Ordering::Relaxed) {
                    let size = match socket.recv(&mut buffer) {
                        Ok(s) => s,
                        Err(e) => {
                            warn!("Receive from vision server;{:?}", e);
                            continue;
                        }
                    };
                    let packet: SSL_WrapperPacket = protobuf::parse_from_bytes(&buffer[..size]).unwrap();
                    if let Some(w)=World::from_message(&packet){
                        vision_sender.send(Box::new(w)).unwrap();
                    }
                    
                }
            })
        };
        Listener {
            killer: killer,
            vision_receiver: vision_receiver,
            vision_handler: vision_handler,
        }
    }

    pub fn get_world(&self) -> &Receiver<Box<World>> {
        &self.vision_receiver
    }
}
/*


  pub fn run(settings: &Settings) -> io::Result<Listener> {
      //socket
      let addr = {
          let [a, b, c, d] = settings.vision_ip4;
          Ipv4Addr::new(a, b, c, d)
      };
      let socket = UdpSocket::bind(&SocketAddr::from((addr, settings.vision_port)))?;
      socket.join_multicast_v4(&addr, &Ipv4Addr::new(0, 0, 0, 0))?;
      //signal
      let kill = Arc::new(AtomicBool::new(false));
      let mut buffer = [0; 1024];
      //buffer
      let handle = {
          let kill = kill.clone();
          thread::spawn(move || {

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
          kill: kill.clone(),
          handle: thread::spawn(move || {
              let mut buffer =vec![0u8;1024];
              let kill=*kill.unw

          });
      })
  }

*/

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
