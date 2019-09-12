use std::env;
use serde_derive::{Serialize,Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    command:String,
    port: u16,
    host: String,

}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            command:"".to_string(),
            port: 8001,
            host: "localhost".to_string(),
        }
    }
}

