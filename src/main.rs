//TODO 整理する
extern crate listener;
mod settings;
extern crate model;
extern crate viewer;
use env_logger;
use log:: error;
use settings::Settings;
use std::env;
use std::sync::{RwLock,Arc};
//use gtk::prelude::*;

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
    listener::Listener::spawn(&settings.listener,world.clone());
    let _main_window = viewer::Viewer::new(&settings.viewer, world);
    gtk::main();
}
