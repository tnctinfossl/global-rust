use std::io;
use std::net::{UdpSocket,Ipv4Addr,SocketAddr};
use super::messages_robocup_ssl_wrapper::SSL_WrapperPacket;
use super::messages::World;

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

    pub fn recv(&mut self)->io::Result<Option<World>>{
        //受信
        let mut buffer=[0;1024];
        let size=self.socket.recv(&mut buffer)?;
        //解読
        let packet:SSL_WrapperPacket=protobuf::parse_from_bytes(&buffer[..size])?;
        Ok(World::from_message(&packet))
    }
}