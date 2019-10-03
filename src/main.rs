//TODO 整理する
extern crate listener;
mod settings;
extern crate model;
extern crate viewer;
use env_logger;
use log::{debug, error, info, warn};
use settings::Settings;
use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::net::Ipv4Addr;
use std::sync::{RwLock,Arc};
use gtk::prelude::*;

fn main() {
    //init logger
    env::set_var("RUST_LOG", "info");
    env_logger::init();
    //load settings
    let settings = if env::args().any(|s| s == "default") {
        Settings::default()
    } else {
        Settings::load_or_create("settings.json")
    };
    //fix log level
    env::set_var("RUST_LOG", settings.logger.level);
    //gtk init
    if gtk::init().is_err() {
        error!("gtk cannot initialize");
        return;
    }
    //connect server
    
    let world = Arc::new(RwLock::new(model::World::default()));
    let listener = listener::Listener::new(&settings.listener,world.clone());
    let mut main_window = viewer::Viewer::new(&settings.viewer, world);

    let world_recv = listener.world_receiver;
    gtk::main();
}
