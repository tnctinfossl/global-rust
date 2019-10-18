mod refbox;
use std::sync::{Arc,RwLock};
mod referee;
mod game_event;
extern crate protobuf;
extern crate model;
fn main() {
    let reciver=refbox::RefBox::spawn(&refbox::Settings::default()).unwrap();
    loop{
        println!("{:?}",reciver.recv().unwrap());
    }
}
