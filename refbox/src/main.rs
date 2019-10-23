mod refbox;
use std::sync::mpsc::{channel, Receiver};
mod referee;
mod game_event;
extern crate protobuf;
extern crate model;
fn main() {
    let (tx,rx)=channel();
    refbox::RefBox::spawn(&refbox::Settings::default(),tx).unwrap();
    loop{
        println!("{:?}",rx.recv().unwrap());
    }
}
