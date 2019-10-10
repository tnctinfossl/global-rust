mod refbox;
use std::sync::{Arc,RwLock};
mod referee;
mod game_event;
mod updater;
extern crate model;
fn main() {
    let world = Arc::new(RwLock::new(model::World::default()));
    refbox::RefBox::spawn(&refbox::Settings::default(), world).unwrap();
    loop{}
}
