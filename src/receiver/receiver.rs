use std::io;
use std::net::{UdpSocket,Ipv4Addr,SocketAddr};
use super::grSim_Replacement::grSim_Replacement;
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

    pub fn recv(&mut self)->io::Result<()>{
        //受信
        let mut buffer=[0;1024];
        let size=self.socket.recv(&mut buffer)?;
        //解読
        let _replacement:grSim_Replacement=protobuf::parse_from_bytes(&buffer[0..size])?;
        Ok(())//TODO ちゃんと書く
    }
}

#[test]
fn test() {
    println!("test");
}
