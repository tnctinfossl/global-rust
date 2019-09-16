//use super::messages::messages_robocup_ssl_detection as direction;
//use super::messages::messages_robocup_ssl_geometry as geometry;
//use super::messages::messages_robocup_ssl_refbox_log as refbox;
//use super::messages::messages_robocup_ssl_wrapper as wrapper;
use std::io;
use std::net::{UdpSocket,Ipv4Addr,SocketAddr};



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
        println!("size:{}",size);
        for idx in 0..size{
            print!("{},",buffer[idx]);
        }  
        println!();
        Ok(())
    }
}

#[test]
fn test() {
    println!("test");
}
