use log::{debug, error, info, warn};
use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use crate::listener::listener;
#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub listener:listener::Settings,
    pub viewer: Viewer,
    pub logger: Logger,
}



#[derive(Serialize, Deserialize, Debug)]
pub struct Viewer {
    pub window_x: i32,
    pub window_y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Logger {
    pub level: String,
}

impl Default for Settings {
    fn default() -> Settings {
        Settings {
            listener:listener::Settings::default(),
            viewer: Viewer {
                window_x: 640,
                window_y: 480,
            },
            logger: Logger {
                level: "info".to_owned(),
            },
        } 
    }
}

impl Settings {

    pub fn load_or_create(filename: &str) -> Settings {
        if let Ok(file) = File::open(filename) {
            let reader = BufReader::new(file);
            if let Ok(json) = serde_json::from_reader(reader) {
                json
            } else {
                warn!("use default settings because {} is breken.",filename);
                Settings::default()
            }
        } else {
            info!("create settings.json because settings.json is not existed");
            let settings = Settings::default();
            if let Ok(file) = File::create(filename) {
                let writer = BufWriter::new(file);
                serde_json::to_writer(writer, &settings).unwrap();
            } else {
                warn!("cannot create {}",filename);
            }
            settings
        }
    }
}
