//TODO 整理する
extern crate vision;
extern crate refbox;
mod settings;
extern crate model;
extern crate viewer;
use env_logger;
use log::error;
use settings::Settings;
use std::env;
use std::sync::{Arc, RwLock};
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


    //connect server
    let world = Arc::new(RwLock::new(model::World::default()));
    if let Err(e)=refbox::RefBox::spawn(&settings.refbox, world.clone()){
        error!("{:?}",e);
        return;
    }
    
    vision::Listener::spawn(&settings.vision, world.clone());
    match viewer::Viewer::new(&settings.viewer, world) {
        Ok(main_window)=>main_window.run(),
        Err(e)=>error!("{}",e),
    }
}
