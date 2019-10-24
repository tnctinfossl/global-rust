extern crate model;
use std::sync::{Arc, RwLock};
mod viewer;
mod field;
mod size_mode;
mod fps_counter;
mod info_tree;
fn main() {
    let world = Arc::new(RwLock::new(model::World::default()));
    let settings = viewer::Settings::default();
    let window = viewer::Viewer::new(&settings, world).unwrap();
    window.run();
}
