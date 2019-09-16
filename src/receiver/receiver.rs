use std::io;
use std::net::{UdpSocket,Ipv4Addr,SocketAddr};
use super::grSim_Packet::grSim_Packet;
use super::grSim_Replacement::grSim_Replacement;
use super::packet::*;
pub struct Receiver {
    socket :UdpSocket,
}

impl Receiver {
    pub fn open(addr:Ipv4Addr,vision_port:u16)->io::Result<Receiver>{
        
        let vision_socket = SocketAddr::from((addr,vision_port));
        let socket = UdpSocket::bind(vision_socket)?;
        socket.join_multicast_v4(&addr, &Ipv4Addr::new(0,0,0,0))?;
        Ok(
            Receiver{
                socket:socket
            }
        )
    }

    pub fn recv(&mut self)->io::Result</*Replacement*/()>{
        //受信
        let mut buffer=[0;1024];
        let size=self.socket.recv(&mut buffer)?;
        //解読
        let proto=protobuf::parse_from_bytes(&buffer)?;
        let replacement=Packet::from(&proto);
        //println!("{:?}",replacement);
        //Ok(replacement)
        Ok(())
    }
}

#[test]
fn test() {
    println!("test");
}
