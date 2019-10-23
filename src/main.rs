//TODO 整理する
extern crate refbox;
extern crate vision;
mod settings;
extern crate model;
extern crate viewer;
use env_logger;
use log::error;
use settings::Settings;
use std::env;
use std::sync::mpsc::channel;
use std::sync::{Arc, RwLock};
use std::thread;
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
    let (tx, rx) = channel();
    let world = Arc::new(RwLock::new(model::World::default()));
    refbox::RefBox::spawn(&settings.refbox, tx.clone()).unwrap();
    vision::Listener::spawn(&settings.vision, tx).unwrap();

    let world2 = world.clone();
    thread::spawn(move || loop {
        if let Ok(w) = rx.recv() {
            if let Ok(mut world) = world2.write() {
                //println!("{:?}", *world);
                world.merge(w, &model::MergeOptions::default());
            }
        }
    });

    match viewer::Viewer::new(&settings.viewer, world) {
        Ok(main_window) => main_window.run(),
        Err(e) => error!("{}", e),
    }
}
