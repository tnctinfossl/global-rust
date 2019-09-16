use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub vision_ip4: [u8;4],//ip address  of cam or sim
    pub vision_port: u16,//
    pub command_port: u16,
    
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            vision_ip4:[224,5,23,2],
            vision_port: 10020,
            command_port: 20011,
        }
    }
}
