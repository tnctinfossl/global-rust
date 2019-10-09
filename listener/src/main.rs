use std::sync::{Arc,RwLock};
use listener::{Listener,Settings};
fn main(){
    //test code
    let settings=Settings::default();

    let world = Arc::new(RwLock::new(model::World::default()));
   
    let listener=Listener::new(&settings,world);
    let receiver=&listener.world_receiver;
    /*loop{
        if let Ok(world)=receiver.try_recv(){
            println!("{:?}",world);
        }
    }*/
}