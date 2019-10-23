use std::sync::mpsc::channel;
use vision::{Listener,Settings};
fn main(){
    //test code
    let settings=Settings::default();

    //let world = Arc::new(RwLock::new(model::World::default()));
    let (tx,rx)=channel();
    Listener::spawn(&settings,tx).unwrap();
    loop{
        if let Ok(data)=rx.recv(){
            println!("{:?}",data);
        }
    }
}