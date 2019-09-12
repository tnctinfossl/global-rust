use serde_derive::{Serialize,Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub command:String,
    pub port: u16,
    pub host: String,
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

