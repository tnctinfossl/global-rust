use super::messages::World;
use super::messages_robocup_ssl_wrapper::SSL_WrapperPacket;
use log::{ error,  warn};
use serde_derive::{Deserialize, Serialize};

use std::net::{Ipv4Addr, SocketAddr, UdpSocket};

use std::sync::mpsc::{channel, Receiver};

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
            vision_ip4: [000,0,00,0],
            vision_port: 00000,
            command_port: 00000,
        }
    }
}