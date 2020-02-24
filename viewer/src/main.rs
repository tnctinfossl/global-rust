extern crate cairo;
extern crate gdk;
extern crate gio;
extern crate glib;
extern crate gtk;
extern crate model;
extern crate viewer;
mod field;
mod fps_counter;
mod size_mode;

use self::viewer::Settings;
pub use self::viewer::Viewer;
pub use size_mode::SizeMode;
use std::env;
use std::sync::mpsc::channel;
use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let world = Arc::new(RwLock::new(model::World::default()));
    let settings = Settings::default();
    match viewer::Viewer::new(&settings, world) {
        Ok(main_window) => main_window.run(),
        Err(e) => println!("{}", e),
    }
}
