use std::sync::{Arc,RwLock};
use vision::{Listener,Settings};
fn main(){
    //test code
    let settings=Settings::default();

    let world = Arc::new(RwLock::new(model::World::default()));
   
    let receive=Listener::spawn(&settings).unwrap();
    loop{
        if let Ok(data)=receive.recv(){
            println!("{:?}",data);
        }
    }
}