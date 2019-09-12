use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub vision_address: String,
    pub vision_port: u16,
    pub command_port: u16,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            vision_address: "224.5.23.2".to_string(),
            vision_port: 10020,
            command_port: 20011,
        }
    }
}
