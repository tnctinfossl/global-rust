pub mod packet;
pub mod radio;
pub use crate::radio::*;
use packet::*;
use std::sync::mpsc::channel;
extern crate log;
use crate::radio::*;
use std::thread;
fn main() {
    let settings = Settings::default();
    let (tx, rx) = channel();
    Radio::spawn(settings, rx).unwrap();
    loop {
        tx.send(Packet::default()).unwrap();
        thread::sleep_ms(100);
    }
}
