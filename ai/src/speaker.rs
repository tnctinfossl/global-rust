//use super::messages::World;
//use super::messages_robocup_ssl_wrapper::SSL_WrapperPacket;
//use log::{ error,  warn};
//use serde_derive::{Deserialize, Serialize};

//use std::net::{Ipv4Addr, SocketAddr, UdpSocket};

//use std::sync::mpsc::{channel, Receiver};

use std::net::UdpSocket;
//use std::{str, io};
//use std::thread;
/*use std::thread;
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Settings {
    pub vision_ip4: [u8; 4], //ip address  of cam or sim
    pub vision_port: u16,    //
    pub command_port: u16,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            vision_ip4: [000,0,00,0],
            vision_port: 00000,
            command_port: 00000,
        }
    }
}*/


fn main() {
    let socket = UdpSocket::bind("localhost:33333").expect("failed to bind socket");
    //socket.set_read_timeout(Some(Duration::from_secs(2))).unwrap();
    //socket.set_write_timeout(Some(Duration::from_secs(2))).unwrap();
    println!("c");
    socket.connect("133.85.144.27:50000").expect("connect function failed");
    println!("b");
    loop {
        let mut data = [1,2,3,4,5];
        println!("a");
        

        socket.send(&mut data).expect("couldn't send message");

       /* let mut buffer = [0u8; 1024];
        socket.recv_from(&mut buffer).expect("failed to receive");
        print!("{}", str::from_utf8(&buffer).expect("failed to convert to String"));*/
    }
}
