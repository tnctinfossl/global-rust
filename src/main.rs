//TODO 整理する
#[macro_use]

mod listener;
mod settings;
mod viewer;
use env_logger;
use log::{debug, error, info, warn};
use listener::listener::Listener;
use settings::Settings;
use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::net::Ipv4Addr;



fn main() {
    //init logger
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    //load settings
    let settings=Settings::load_or_create("settings.json");
    //fix log level
    env::set_var("RUST_LOG", settings.logger.level);
    //connect server
    //let (tx,rx)=std::sync::mpsc::channel();
    //let listener = Listener::run(&settings.listener,tx);
    viewer::run();
}




/*
fn init(args: &[String]) {
    //TODO きれいに書く
    let filename: &str = if let Some(name) = args.iter().nth(1) {
        name
    } else {
        "setting.json"
    };
    let file = File::create(filename).expect(&format!("Error:Cannot Create {}", filename));
    let writer = BufWriter::new(file);
    let default = Settings::default();
    serde_json::to_writer(writer, &default).expect(&format!("Error:Cannot Write {}", filename));
}

fn receive(args: &[String]) {
    //simple receiver
    let filename: &str = if let Some(name) = args.iter().nth(1) {
        name
    } else {
        "setting.json"
    };

    let file = File::open(filename).expect(&format!("Error:Cannot Read {}", filename));
    let reader = BufReader::new(file);
    let settings: Settings =
        serde_json::from_reader(reader).expect(&format!("Error:Cannot Parse {}", filename));

    let ip = {
        let [a, b, c, d] = settings.net.vision_ip4;
        Ipv4Addr::new(a, b, c, d)
    };
    //let mut r = Receiver::open(ip, settings.net.vision_port).unwrap();
   // r.recv().unwrap();
}

fn run(args: &[String]) {
    let filename: &str = if let Some(name) = args.iter().nth(1) {
        name
    } else {
        "setting.json"
    };

    let file = File::open(filename).expect(&format!("Error:Cannot Read {}", filename));
    let reader = BufReader::new(file);
    let settings: Settings =
        serde_json::from_reader(reader).expect(&format!("Error:Cannot Parse {}", filename));
    let ip = {
        let [a, b, c, d] = settings.net.vision_ip4;
        Ipv4Addr::new(a, b, c, d)
    };
  //  let mut r = Receiver::open(ip, settings.net.vision_port).unwrap();
}
*/