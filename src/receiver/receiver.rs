    use std::io;
use std::net::{UdpSocket,Ipv4Addr,SocketAddr};
use super::messages_robocup_ssl_wrapper::SSL_WrapperPacket;
use super::messages::World;
use std::sync::mpsc::{channel,Sender,Receiver};

use serde_derive::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Settings{
    pub vision_ip4: [u8; 4], //ip address  of cam or sim
    pub vision_port: u16,    //
    pub command_port: u16,
}

impl Default for Settings{
    fn default()->Settings{
        Settings{
            vision_ip4: [224, 5, 23, 2],
            vision_port: 10020,
            command_port: 20011,
        }
    }
}

pub struct Listener {
    socket :UdpSocket,
}

impl Listener {

    pub fn new (net:&Settings)->io::Result<Listener>{
        let addr= {
            let [a, b, c, d] = net.vision_ip4;
            Ipv4Addr::new(a, b, c, d)
        };
        let socket = UdpSocket::bind(&SocketAddr::from((addr, net.vision_port)))?;
        socket.join_multicast_v4(&addr, &Ipv4Addr::new(0,0,0,0))?;
        Ok({
            Listener{
                socket:socket
            }
        })
    }
/*
    pub fn launch(&mut self)->Receiver<World>{
        let (tx,rx)=channel();
        thread::spawn(move || {
            let mut buffer[0;512];
            loop{
                if let Ok(size)=self.socket.recv(&mut buffer){
                    if let Ok(w)=protobuf::parse_from_bytes(&buffer[..size]){
                        tx.send(w).expect("")
                    }else{
                        warm!("parse");
                    }
                }else{
                    warm!("recv");
                }
            }
            
        });
    }
*/
    /*pub fn open(addr:Ipv4Addr,vision_port:u16)->io::Result<Receiver>{
        
        let vision_socket = SocketAddr::from((addr,vision_port));
        let socket = UdpSocket::bind(vision_socket)?;
        socket.join_multicast_v4(&addr, &Ipv4Addr::new(0,0,0,0))?;
        Ok(
            Receiver{
                socket:socket
            }
        )
    }*/

    pub fn recv(&mut self)->io::Result<Option<World>>{
        //受信
        let mut buffer=[0;1024];
        let size=self.socket.recv(&mut buffer)?;
        //解読
        let packet:SSL_WrapperPacket=protobuf::parse_from_bytes(&buffer[..size])?;
        Ok(World::from_message(&packet))
    }
}

