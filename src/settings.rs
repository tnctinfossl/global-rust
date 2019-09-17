use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub net:Net,
    pub viewer:Viewer
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Net{
    pub vision_ip4: [u8;4],//ip address  of cam or sim
    pub vision_port: u16,//
    pub command_port: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Viewer{
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            net:Net{
            vision_ip4:[224,5,23,2],
            vision_port: 10020,
            command_port: 20011,
            },
            viewer:Viewer{

            }
        }
    }
}
