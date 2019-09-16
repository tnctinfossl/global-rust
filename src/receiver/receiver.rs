
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
        let mut buffer=[0;1024];
        let size=self.socket.recv(&mut buffer)?;
        let mut replacement=grSim_Replacement::new();
        //replacement.merge_from(buffer);



        Ok(())
    }
}

#[test]
fn test() {
    println!("test");
}
